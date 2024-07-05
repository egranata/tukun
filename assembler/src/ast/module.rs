use std::collections::HashMap;

use pest::iterators::Pair;
use runtime::intern_value::InternValue;

use crate::{
    ast::{attribute::Attribute, types::TypeAlias},
    parser::Rule,
    result::AssemblerResult,
};

use super::{constant::Constant, function::Function, types::ValueType};

#[derive(Debug)]
pub struct Module {
    pub(crate) name: String,
    pub(crate) constants: Vec<Constant>,
    pub(crate) constant_names: HashMap<String, usize>,
    pub(crate) functions: Vec<Function>,
    pub(crate) attributes: HashMap<String, String>,
    pub(crate) types: HashMap<String, ValueType>,
}

impl Module {
    pub fn from_parse_tree(p: Pair<'_, Rule>) -> AssemblerResult<Self> {
        assert!(p.as_rule() == Rule::module);

        let mut ret = Module {
            name: "com.tukunc.module".to_owned(),
            constants: Default::default(),
            constant_names: Default::default(),
            functions: vec![],
            attributes: Default::default(),
            types: Default::default(),
        };

        ret.types.insert(
            "integer".to_owned(),
            ValueType::B(crate::ast::types::BuiltinType::Integer),
        );
        ret.types.insert(
            "string".to_owned(),
            ValueType::B(crate::ast::types::BuiltinType::String),
        );
        ret.types.insert(
            "logical".to_owned(),
            ValueType::B(crate::ast::types::BuiltinType::Logical),
        );

        for bf in p.into_inner() {
            match bf.as_rule() {
                Rule::function => {
                    let f = Function::from_parse_tree(bf)?;
                    ret.functions.push(f);
                }
                Rule::interned_value => {
                    let c = Constant::from_parse_tree(bf)?;
                    ret.add_constant(c);
                }
                Rule::typedef => {
                    let t = TypeAlias::from_parse_tree(bf, &ret.types)?;
                    ret.add_typealias(t);
                }
                Rule::attribute => {
                    let a = Attribute::from_parse_tree(bf)?;
                    ret.attributes.insert(a.name, a.value);
                }
                Rule::EOI => {}
                _ => panic!("unexpected entry {bf}"),
            }
        }

        if let Some(name) = ret.attributes.get("modname") {
            name.clone_into(&mut ret.name);
        }

        Ok(ret)
    }
}

impl Module {
    pub(crate) fn constant_idx_by_name(&self, name: &str) -> usize {
        #[allow(clippy::expect_fun_call)]
        *self
            .constant_names
            .get(name)
            .expect(&format!("invalid const name {}", name))
    }

    pub(crate) fn add_constant(&mut self, c: Constant) -> InternValue {
        let idx = self.constants.len();
        let name = c.name.clone();
        self.constants.push(c.clone());
        self.constant_names.insert(name, idx);
        c.val
    }

    pub(crate) fn add_typealias(&mut self, t: TypeAlias) {
        let name = t.name();
        let dest = t.target();
        self.types.insert(name.to_owned(), dest.clone());
    }
}
