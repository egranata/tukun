use std::{cell::RefCell, rc::Rc};

use crate::{
    types::{record::RecordType, RuntimeType},
    values::RuntimeValue,
};

#[derive(Debug, PartialEq, Eq)]
struct RecordImpl {
    value_type: RecordType,
    values: Vec<RuntimeValue>,
}

#[derive(Clone)]
pub struct Record {
    a: Rc<RefCell<RecordImpl>>,
}

impl std::fmt::Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.a.borrow().fmt(f)
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        write!(f, "{{")?;
        let a = self.a.borrow();
        for item in &a.values {
            if first {
                write!(f, "{:?}", item)?;
                first = false;
            } else {
                write!(f, ", {:?}", item)?;
            }
        }
        write!(f, "}}")
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.a.borrow().eq(&other.a.borrow())
    }
}

impl Eq for Record {}

impl Record {
    pub fn new_typed(t: RecordType, v: &[RuntimeValue]) -> Self {
        assert!(t.types.len() == v.len());
        for (vt, tt) in std::iter::zip(v.iter(), t.types.iter()) {
            assert!(vt.get_type() == *tt);
        }

        Self {
            a: Rc::new(RefCell::new(RecordImpl {
                value_type: t,
                values: v.to_owned(),
            })),
        }
    }

    pub fn new_inferred(v: &[RuntimeValue]) -> Self {
        let et: Vec<RuntimeType> = v
            .iter()
            .map(RuntimeValue::get_type)
            .collect::<Vec<RuntimeType>>();
        let rt = RecordType::new(&et);
        Self::new_typed(rt, v)
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
        assert!(val.get_type() == self.a.borrow().value_type.types[idx]);
        self.a.borrow_mut().values[idx] = val.clone()
    }

    pub fn get_type(&self) -> RuntimeType {
        RuntimeType::Record(Box::new(self.a.borrow().value_type.clone()))
    }
}
