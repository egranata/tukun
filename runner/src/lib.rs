use runtime::module_definition::ModuleDef;

pub trait ModuleSource {
    fn description(&self) -> String;
    fn read(&self) -> std::io::Result<ModuleDef>;
}

pub struct FileModuleSource {
    path: String,
}

impl FileModuleSource {
    pub fn new(path: &str) -> Self {
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
