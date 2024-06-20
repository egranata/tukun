use super::RuntimeValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompareResult {
    LessThan,
    EqualTo,
    GreaterThan,
    Unspecified,
}

impl From<std::cmp::Ordering> for CompareResult {
    fn from(value: std::cmp::Ordering) -> Self {
        match value {
            std::cmp::Ordering::Less => Self::LessThan,
            std::cmp::Ordering::Equal => Self::EqualTo,
            std::cmp::Ordering::Greater => Self::GreaterThan,
        }
    }
}

pub(crate) fn compare_values(v1: &RuntimeValue, v2: &RuntimeValue) -> CompareResult {
    use CompareResult::{EqualTo, Unspecified};

    if v1.get_type() != v2.get_type() {
        Unspecified
    } else {
        match (v1, v2) {
            (RuntimeValue::Integer(i1), RuntimeValue::Integer(i2)) => From::from(i1.cmp(i2)),
            (RuntimeValue::Logical(b1), RuntimeValue::Logical(b2)) => {
                if b1 == b2 {
                    EqualTo
                } else {
                    Unspecified
                }
            }
            (RuntimeValue::Float(f1), RuntimeValue::Float(f2)) => From::from(f1.total_cmp(f2)),
            (RuntimeValue::String(s1), RuntimeValue::String(s2)) => {
                if s1 == s2 {
                    EqualTo
                } else {
                    Unspecified
                }
            }
            (RuntimeValue::Function(f1), RuntimeValue::Function(f2)) => {
                if f1 == f2 {
                    EqualTo
                } else {
                    Unspecified
                }
            }
            (RuntimeValue::Arr(a1), RuntimeValue::Arr(a2)) => {
                if a1 == a2 {
                    EqualTo
                } else {
                    Unspecified
                }
            }
            (RuntimeValue::Record(r1), RuntimeValue::Record(r2)) => {
                if r1 == r2 {
                    EqualTo
                } else {
                    Unspecified
                }
            }
            (RuntimeValue::Type(t1), RuntimeValue::Type(t2)) => {
                if t1 == t2 {
                    EqualTo
                } else {
                    Unspecified
                }
            }
            _ => Unspecified,
        }
    }
}

impl PartialEq for RuntimeValue {
    fn eq(&self, v2: &Self) -> bool {
        if self.get_type() != v2.get_type() {
            false
        } else {
            match self {
                RuntimeValue::Integer(x1) => {
                    let x2 = v2.as_integer().expect("invalid value");
                    *x1 == *x2
                }
                RuntimeValue::Logical(x1) => {
                    let x2 = v2.as_logical().expect("invalid value");
                    *x1 == *x2
                }
                RuntimeValue::Float(x1) => {
                    let x2 = v2.as_float().expect("invalid value");
                    *x1 == *x2
                }
                RuntimeValue::String(x1) => {
                    let x2 = v2.as_string().expect("invalid integer value");
                    *x1 == *x2
                }
                RuntimeValue::Function(x1) => {
                    let x2 = v2.as_function().expect("invalid integer value");
                    x1.fullname() == x2.fullname()
                }
                RuntimeValue::Arr(x1) => {
                    let x2 = v2.as_arr().expect("invalid integer value");
                    x1 == x2
                }
                RuntimeValue::Record(x1) => {
                    let x2 = v2.as_record().expect("invalid integer value");
                    x1 == x2
                }
                RuntimeValue::Type(x1) => {
                    let x2 = v2.as_type().expect("invalid integer value");
                    *x1 == *x2
                }
            }
        }
    }
}

impl Eq for RuntimeValue {}
