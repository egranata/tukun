use clap::Parser;
use corelib::register_corelib;
use runtime::{
    environ::Environment, module_definition::ModuleDef, runloop::run_loop,
    runtime_module::RuntimeModule,
};

trait ModuleSource {
    fn description(&self) -> String;
    fn read(&self) -> std::io::Result<ModuleDef>;
}

struct FileModuleSource {
    path: String,
}

impl FileModuleSource {
    fn new(path: &str) -> Self {
        Self {
            path: path.to_owned(),
        }
    }
}

impl ModuleSource for FileModuleSource {
    fn description(&self) -> String {
        format!("file: {}", self.path)
    }

    fn read(&self) -> std::io::Result<ModuleDef> {
        let bytes = std::fs::read(&self.path);
        match bytes {
            Ok(bytes) => {
                let mdef: Result<ModuleDef, Box<bincode::ErrorKind>> = bincode::deserialize(&bytes);
                match mdef {
                    Ok(mdef) => Ok(mdef),
                    Err(err) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
                }
            }
            Err(err) => Err(err),
        }
    }
}

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(required = true)]
    inputs: Vec<String>,
    #[arg(short, long, default_value = "")]
    main_f: String,
    #[arg(short, long, default_value_t = false)]
    dump_stack: bool,
    #[arg(short, long, default_value_t = false)]
    omit_corelib: bool,
}
fn main() {
    let args = Cli::parse();

    let mut env = Environment::default();

    let module_sources = args
        .inputs
        .iter()
        .map(|input| {
            let b: Box<dyn ModuleSource> = Box::new(FileModuleSource::new(input));
            b
        })
        .collect::<Vec<Box<dyn ModuleSource>>>();

    let mut module_defs: Vec<ModuleDef> = Vec::new();
    for ms in module_sources {
        match ms.read() {
            Ok(mdef) => {
                module_defs.push(mdef);
            }
            Err(err) => {
                panic!("error trying to read {}: {}", ms.description(), err);
            }
        }
    }

    module_defs.iter().map(RuntimeModule::from).for_each(|rm| {
        env.add_module(rm);
    });

    if !args.omit_corelib {
        register_corelib(&mut env);
    }

    let main_f = if args.main_f.is_empty() {
        if let Some(last) = module_defs.last() {
            format!("{}.main", last.name())
        } else {
            panic!("unable to infer main function to run");
        }
    } else {
        args.main_f
    };

    match env.lookup_function(&main_f) {
        Some(f) => {
            run_loop(&f, &mut env);
        }
        None => panic!("unable to find main function"),
    }

    if args.dump_stack {
        while !env.is_stack_empty() {
            let v = env.pop_value();
            println!("{v}");
        }
    }
}
