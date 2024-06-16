use super::RuntimeValue;

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
                    if x1.len() != x2.len() {
                        false
                    } else {
                        for i in 0..x1.len() {
                            let x1x = x1.get(i);
                            let x2x = x2.get(i);
                            if x1x != x2x {
                                return false;
                            }
                        }
                        true
                    }
                }
                RuntimeValue::Record(x1) => {
                    let x2 = v2.as_record().expect("invalid integer value");
                    if x1.len() != x2.len() {
                        false
                    } else {
                        for i in 0..x1.len() {
                            let x1x = x1.get(i);
                            let x2x = x2.get(i);
                            if x1x != x2x {
                                return false;
                            }
                        }
                        true
                    }
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
