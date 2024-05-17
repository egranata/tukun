use serde::{Deserialize, Serialize};

use super::{typedef::TypeDef, RuntimeType};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArrayType {
    pub(crate) value_type: RuntimeType,
    pub(crate) len: usize,
}

impl ArrayType {
    pub fn new(t: RuntimeType, l: usize) -> Self {
        Self {
            value_type: t.clone(),
            len: l,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn value_type(&self) -> &RuntimeType {
        &self.value_type
    }
}

impl ArrayType {
    pub fn to_typedef(&self, name: &str) -> TypeDef {
        TypeDef::new(name, &RuntimeType::Arr(Box::new(self.clone())))
    }
}

impl From<&ArrayType> for String {
    fn from(value: &ArrayType) -> Self {
        format!("type::array[l={},et={}]", value.len(), value.value_type())
    }
}
impl From<ArrayType> for String {
    fn from(value: ArrayType) -> Self {
        String::from(&value)
    }
}
