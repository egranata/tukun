use serde::{Deserialize, Serialize};

use super::RuntimeType;

use super::typedef::TypeDef;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordType {
    pub(crate) types: Vec<RuntimeType>,
}

impl RecordType {
    pub fn new(v: &[RuntimeType]) -> Self {
        Self { types: v.to_vec() }
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    pub fn get(&self, idx: usize) -> &RuntimeType {
        &self.types[idx]
    }
}

impl RecordType {
    pub fn to_typedef(&self, name: &str) -> TypeDef {
        TypeDef::new(name, &RuntimeType::Record(Box::new(self.clone())))
    }
}
