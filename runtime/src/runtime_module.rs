use std::{cell::RefCell, collections::HashMap, rc::Rc};

use either::Either;

use crate::{
    environ::Environment,
    intern_value::InternValue,
    module_definition::{FunctionDef, ModuleDef},
    runloop::RunloopResult,
    types::typedef::TypeDef,
};

pub trait NativeCallable {
    fn call(&self, env: &mut Environment) -> RunloopResult;
    fn name(&self) -> String;
}

impl std::fmt::Debug for dyn NativeCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "native function {}", self.name())
    }
}

#[derive(Debug)]
pub(crate) struct RuntimeFunctionImpl {
    pub(crate) owner: RuntimeModule,
    pub(crate) content: Either<FunctionDef, Box<dyn NativeCallable>>,
}

impl RuntimeFunctionImpl {
    fn name(&self) -> String {
        match &self.content {
            Either::Left(f) => f.name(),
            Either::Right(f) => f.name(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeCallable {
    pub(crate) f: Rc<RuntimeFunctionImpl>,
}

impl PartialEq<RuntimeCallable> for RuntimeCallable {
    fn eq(&self, other: &RuntimeCallable) -> bool {
        self.fullname() == other.fullname()
    }
}

impl Eq for RuntimeCallable {}

impl RuntimeCallable {
    pub fn from_fdef(m: &RuntimeModule, f: FunctionDef) -> Self {
        Self {
            f: Rc::new(RuntimeFunctionImpl {
                owner: m.clone(),
                content: Either::Left(f),
            }),
        }
    }

    pub fn from_native(m: &RuntimeModule, f: Box<dyn NativeCallable>) -> Self {
        Self {
            f: Rc::new(RuntimeFunctionImpl {
                owner: m.clone(),
                content: Either::Right(f),
            }),
        }
    }

    pub fn name(&self) -> String {
        self.f.name()
    }

    pub fn fullname(&self) -> String {
        format!("{}.{}", self.f.owner.name(), self.name())
    }

    pub fn module(&self) -> RuntimeModule {
        self.f.owner.clone()
    }
}

#[derive(Debug)]
pub(crate) struct RuntimeTypeDefImpl {
    pub(crate) owner: RuntimeModule,
    pub(crate) content: TypeDef,
}

impl RuntimeTypeDefImpl {
    fn name(&self) -> String {
        self.content.name().to_owned()
    }

    fn target(&self) -> &crate::types::RuntimeType {
        self.content.target()
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeTypeDef {
    pub(crate) f: Rc<RuntimeTypeDefImpl>,
}

impl PartialEq<RuntimeTypeDef> for RuntimeTypeDef {
    fn eq(&self, other: &RuntimeTypeDef) -> bool {
        self.fullname() == other.fullname()
    }
}

impl Eq for RuntimeTypeDef {}

impl RuntimeTypeDef {
    pub fn from_tdef(m: &RuntimeModule, f: TypeDef) -> Self {
        Self {
            f: Rc::new(RuntimeTypeDefImpl {
                owner: m.clone(),
                content: f,
            }),
        }
    }

    pub fn name(&self) -> String {
        self.f.name()
    }

    pub fn fullname(&self) -> String {
        format!("{}.{}", self.f.owner.name(), self.name())
    }

    pub fn module(&self) -> RuntimeModule {
        self.f.owner.clone()
    }

    pub fn target(&self) -> &crate::types::RuntimeType {
        self.f.target()
    }
}

#[derive(Debug)]
struct RuntimeModuleImpl {
    name: String,
    functions: HashMap<String, RuntimeCallable>,
    named_types: HashMap<String, RuntimeTypeDef>,
    intern_values: Vec<Rc<InternValue>>,
}

#[derive(Debug, Clone)]
pub struct RuntimeModule {
    m: Rc<RefCell<RuntimeModuleImpl>>,
}

impl RuntimeModule {
    pub fn from(md: &ModuleDef) -> RuntimeModule {
        let mut this = Self::new(md.name());
        md.functions().for_each(|f| {
            this.add_function_fdef(f);
        });
        md.named_types().for_each(|r| {
            this.add_named_type(r);
        });
        md.interned_values().for_each(|i| {
            this.add_intern_value(i);
        });
        this
    }

    pub fn new(name: &str) -> RuntimeModule {
        Self {
            m: Rc::new(RefCell::new(RuntimeModuleImpl {
                name: name.to_string(),
                functions: HashMap::new(),
                named_types: HashMap::new(),
                intern_values: vec![],
            })),
        }
    }

    pub fn add_named_type(&mut self, r: &TypeDef) -> RuntimeTypeDef {
        let t = RuntimeTypeDef::from_tdef(self, r.clone());
        self.m
            .borrow_mut()
            .named_types
            .insert(r.name().to_string(), t.clone());
        t
    }

    pub fn add_function_fdef(&mut self, f: &FunctionDef) -> RuntimeCallable {
        let f = RuntimeCallable::from_fdef(self, f.clone());
        self.m
            .borrow_mut()
            .functions
            .insert(f.name().to_string(), f.clone());
        f
    }

    pub fn add_function_native(&mut self, f: Box<dyn NativeCallable>) -> RuntimeCallable {
        let f = RuntimeCallable::from_native(self, f);
        self.m
            .borrow_mut()
            .functions
            .insert(f.name().to_string(), f.clone());
        f
    }

    pub fn find_function(&self, name: &str) -> Option<RuntimeCallable> {
        self.m.borrow().functions.get(name).cloned()
    }

    pub fn find_named_type(&self, name: &str) -> Option<RuntimeTypeDef> {
        self.m.borrow().named_types.get(name).cloned()
    }

    pub fn add_intern_value(&mut self, iv: &InternValue) -> Rc<InternValue> {
        let iv = Rc::new(iv.clone());
        self.m.borrow_mut().intern_values.push(iv.clone());
        iv
    }

    pub fn get_intern_value(&self, idx: u16) -> Option<Rc<InternValue>> {
        self.m.borrow().intern_values.get(idx as usize).cloned()
    }

    pub fn name(&self) -> String {
        self.m.borrow().name.clone()
    }
}
