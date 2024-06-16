use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! iv_int {
    ($l:expr) => {
        $crate::intern_value::InternValue::Integer($l)
    };
}

#[macro_export]
macro_rules! iv_str {
    ($l:expr) => {
        $crate::intern_value::InternValue::String($l.to_owned())
    };
}

#[derive(Clone, Debug, Serialize, Deserialize, EnumAsInner)]
pub enum InternValue {
    Integer(u64),
    Float(f64),
    String(String),
}
