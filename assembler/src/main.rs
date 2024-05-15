pub(crate) mod assembler;
pub mod ast;
pub mod lowering;
pub mod parser;
pub mod result;

#[cfg(test)]
pub mod test;

use assembler::do_assemble;
use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(required = true)]
    input: String,
    #[arg(short, long, default_value = "a.out")]
    output: String,
}

impl Cli {
    fn try_read_input(&self) -> std::io::Result<String> {
        std::fs::read_to_string(&self.input)
    }
}

fn main() {
    let args = Cli::parse();
    let input = args.try_read_input();
    if let Err(err) = input {
        panic!("error: {err}");
    }
    let input = input.unwrap();
    let output = do_assemble(&input);
    if let Err(err) = output {
        panic!("error: {err}");
    }
    let output = output.unwrap();
    let result = std::fs::write(args.output, output);
    if let Err(err) = result {
        panic!("error: {err}");
    }
}
