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

impl From<&RuntimeType> for String {
    fn from(value: &RuntimeType) -> Self {
        match value {
            RuntimeType::Integer => String::from("type::integer"),
            RuntimeType::Logical => String::from("type::logical"),
            RuntimeType::String => String::from("type::string"),
            RuntimeType::Function => String::from("type::function"),
            RuntimeType::Arr(at) => String::from(at.as_ref()),
            RuntimeType::Record(rt) => String::from(rt.as_ref()),
            RuntimeType::Type(t) => format!("type::type[t={t}]"),
        }
    }
}
impl From<RuntimeType> for String {
    fn from(value: RuntimeType) -> Self {
        String::from(&value)
    }
}

impl std::fmt::Display for RuntimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
