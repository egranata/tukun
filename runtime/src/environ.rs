use std::collections::HashMap;

use crate::{
    runtime_module::{RuntimeCallable, RuntimeModule, RuntimeTypeDef},
    stack::Stack,
    values::RuntimeValue,
};

#[derive(Default)]
pub struct Environment {
    pub(crate) runtime_stack: Stack<RuntimeValue>,
    pub(crate) modules: HashMap<String, RuntimeModule>,
}

impl Environment {
    pub fn push_value(&mut self, rv: RuntimeValue) {
        self.runtime_stack.push(rv)
    }

    pub fn stack_len(&self) -> usize {
        self.runtime_stack.len()
    }

    pub fn is_stack_empty(&self) -> bool {
        self.runtime_stack.is_empty()
    }

    pub fn pop_value(&mut self) -> RuntimeValue {
        self.runtime_stack.pop()
    }

    pub fn add_module(&mut self, m: RuntimeModule) -> bool {
        self.modules.insert(m.name().to_string(), m).is_none()
    }

    pub fn find_module(&self, name: &str) -> Option<RuntimeModule> {
        self.modules.get(name).cloned()
    }

    fn lookup_module_dotted(&self, name: &str) -> Option<(RuntimeModule, String)> {
        if let Some(idx) = name.rfind('.') {
            let parts = name.split_at(idx);
            let m = parts.0;
            let f = &parts.1[1..];
            self.find_module(m).map(|m| (m, f.to_string()))
        } else {
            None
        }
    }

    pub fn lookup_function(&self, name: &str) -> Option<RuntimeCallable> {
        if let Some((m, f)) = self.lookup_module_dotted(name) {
            m.find_function(&f)
        } else {
            None
        }
    }

    pub fn lookup_named_type(&self, name: &str) -> Option<RuntimeTypeDef> {
        if let Some((m, f)) = self.lookup_module_dotted(name) {
            m.find_named_type(&f)
        } else {
            None
        }
    }
}
