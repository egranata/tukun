use enum_as_inner::EnumAsInner;

use crate::{intern_value::InternValue, runtime_module::RuntimeCallable, types::RuntimeType};

pub mod array;
pub mod comparators;
pub mod record;

#[derive(Clone, Debug, EnumAsInner)]
pub enum RuntimeValue {
    Integer(u64),
    Logical(bool),
    Float(f64),
    String(String),
    Function(RuntimeCallable),
    Arr(array::Array),
    Record(record::Record),
    Type(RuntimeType),
}

impl From<&InternValue> for RuntimeValue {
    fn from(value: &InternValue) -> Self {
        match value {
            InternValue::Integer(x) => Self::Integer(*x),
            InternValue::String(s) => Self::String(s.clone()),
            InternValue::Float(x) => Self::Float(*x),
        }
    }
}

impl RuntimeValue {
    pub fn get_type(&self) -> RuntimeType {
        match self {
            RuntimeValue::Integer(_) => RuntimeType::Integer,
            RuntimeValue::Logical(_) => RuntimeType::Logical,
            RuntimeValue::Float(_) => RuntimeType::Float,
            RuntimeValue::String(_) => RuntimeType::String,
            RuntimeValue::Function(_) => RuntimeType::Function,
            RuntimeValue::Arr(v) => v.get_type(),
            RuntimeValue::Record(r) => r.get_type(),
            RuntimeValue::Type(x) => RuntimeType::Type(Box::new(x.clone())),
        }
    }
}

impl std::fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Integer(x) => {
                write!(f, "Integer({x})")
            }
            RuntimeValue::Logical(b) => {
                write!(f, "Logical({b})")
            }
            RuntimeValue::Float(g) => {
                write!(f, "Float({g})")
            }
            RuntimeValue::String(s) => {
                write!(f, "String({s:?})")
            }
            RuntimeValue::Function(t) => {
                write!(f, "Function({})", t.fullname())
            }
            RuntimeValue::Type(t) => {
                write!(f, "Type({t:?})")
            }
            RuntimeValue::Arr(v) => {
                write!(f, "Arr({v:?})")
            }
            RuntimeValue::Record(r) => {
                write!(f, "Record({r:?})")
            }
        }
    }
}

#[macro_export]
macro_rules! rv_int {
    ($l:expr) => {
        $crate::values::RuntimeValue::Integer($l)
    };
}

#[macro_export]
macro_rules! rv_flt {
    ($l:expr) => {
        $crate::values::RuntimeValue::Float($l)
    };
}

#[macro_export]
macro_rules! rv_bool {
    ($l:expr) => {
        $crate::values::RuntimeValue::Logical($l)
    };
}

#[macro_export]
macro_rules! rv_str {
    ($l:expr) => {
        $crate::values::RuntimeValue::String($l.to_owned())
    };
}

#[macro_export]
macro_rules! rv_arr {
    ( $( $x:expr ),* ) => {
        $crate::values::RuntimeValue::Arr($crate::values::array::Array::new_inferred(&[$($x),*]))
    };
}
