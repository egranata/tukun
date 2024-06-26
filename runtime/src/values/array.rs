use std::{cell::RefCell, rc::Rc};

use crate::{
    types::{array::ArrayType, RuntimeType},
    values::RuntimeValue,
};

#[derive(Debug)]
struct ArrayImpl {
    at: ArrayType,
    values: Vec<RuntimeValue>,
}

#[derive(Clone)]
pub struct Array {
    a: Rc<RefCell<ArrayImpl>>,
}

impl std::fmt::Debug for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.a.borrow().fmt(f)
    }
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        write!(f, "[")?;
        let a = self.a.borrow();
        for item in &a.values {
            if first {
                write!(f, "{:?}", item)?;
                first = false;
            } else {
                write!(f, ", {:?}", item)?;
            }
        }
        write!(f, "]")
    }
}

impl Array {
    pub fn new_typed(t: RuntimeType, v: &[RuntimeValue]) -> Self {
        let values = v.to_vec();
        for value in &values {
            assert!(value.get_type() == t);
        }
        let at = ArrayType::new(t, values.len());
        Self {
            a: Rc::new(RefCell::new(ArrayImpl { at, values })),
        }
    }

    pub fn new_inferred(v: &[RuntimeValue]) -> Self {
        assert!(!v.is_empty());
        let t = v[0].get_type();
        Self::new_typed(t, v)
    }

    pub fn len(&self) -> usize {
        self.a.borrow().values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.a.borrow().values.is_empty()
    }

    pub fn get(&self, idx: usize) -> RuntimeValue {
        self.a.borrow().values[idx].clone()
    }

    pub fn set(&mut self, idx: usize, val: &RuntimeValue) {
        assert!(val.get_type() == self.get_element_type());
        self.a.borrow_mut().values[idx] = val.clone()
    }

    pub fn get_type(&self) -> RuntimeType {
        RuntimeType::Arr(Box::new(self.a.borrow().at.clone()))
    }

    fn get_element_type(&self) -> RuntimeType {
        self.a.borrow().at.value_type.clone()
    }
}

impl Array {
    pub fn as_runtime_value(self) -> RuntimeValue {
        RuntimeValue::Arr(self)
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                let x1x = self.get(i);
                let x2x = other.get(i);
                if x1x != x2x {
                    return false;
                }
            }
            true
        }
    }
}

impl Eq for Array {}
