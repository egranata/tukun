use serde::{Deserialize, Serialize};

use super::RuntimeType;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDef {
    name: String,
    target: RuntimeType,
}

impl TypeDef {
    pub fn new(name: &str, tgt: &RuntimeType) -> Self {
        Self {
            name: name.to_owned(),
            target: tgt.clone(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn target(&self) -> &RuntimeType {
        &self.target
    }
}
