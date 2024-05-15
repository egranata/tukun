use serde::{Deserialize, Serialize};

use crate::{bytecode::Bytecode, intern_value::InternValue, types::typedef::TypeDef};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDef {
    name: String,
    body: Bytecode,
}

impl FunctionDef {
    pub fn new(name: &str, body: Bytecode) -> FunctionDef {
        Self {
            name: String::from(name),
            body,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn body(&self) -> &Bytecode {
        &self.body
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleDef {
    name: String,
    functions: Vec<FunctionDef>,
    named_types: Vec<TypeDef>,
    intern_values: Vec<InternValue>,
}

impl ModuleDef {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            functions: vec![],
            named_types: vec![],
            intern_values: vec![],
        }
    }

    pub fn add_function(&mut self, f: FunctionDef) {
        self.functions.push(f)
    }

    pub fn add_named_type(&mut self, f: &TypeDef) {
        self.named_types.push(f.clone())
    }

    pub fn add_interned_value(&mut self, i: InternValue) -> usize {
        self.intern_values.push(i);
        self.intern_values.len() - 1
    }

    pub fn functions(&self) -> std::slice::Iter<'_, FunctionDef> {
        self.functions.iter()
    }

    pub fn named_types(&self) -> std::slice::Iter<'_, TypeDef> {
        self.named_types.iter()
    }

    pub fn interned_values(&self) -> std::slice::Iter<'_, InternValue> {
        self.intern_values.iter()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
