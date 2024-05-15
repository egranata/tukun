use serde::{Deserialize, Serialize};

pub mod array;
pub mod record;
pub mod typedef;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeType {
    Integer,
    Logical,
    String,
    Function,
    Arr(Box<array::ArrayType>),
    Record(Box<record::RecordType>),
    Type(Box<RuntimeType>),
}

impl RuntimeType {
    pub fn to_typedef(&self, name: &str) -> typedef::TypeDef {
        typedef::TypeDef::new(name, self)
    }
}

impl std::fmt::Display for RuntimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeType::Integer => write!(f, "type::integer"),
            RuntimeType::Logical => write!(f, "type::logical"),
            RuntimeType::String => write!(f, "type::string"),
            RuntimeType::Function => write!(f, "type::function"),
            RuntimeType::Arr(at) => write!(f, "type::array[t={},l={}]", at.value_type, at.len),
            RuntimeType::Record(rt) => write!(f, "type::record[l={}]", rt.len()),
            RuntimeType::Type(t) => write!(f, "type::type[t={}]", t),
        }
    }
}
