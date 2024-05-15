use pest::iterators::Pair;

use crate::{parser::Rule, result::AssemblerResult};

use self::module::Module;

pub mod attribute;
pub mod block;
pub mod constant;
pub mod function;
pub mod instructions;
pub mod module;
pub mod types;

fn parse_string_trim(s: &str) -> String {
    let mut value = s[1..].to_string();
    let _ = value.split_off(value.len() - 1);
    value
}

pub fn parse_tree_to_ast(input: Pair<'_, Rule>) -> AssemblerResult<Module> {
    Module::from_parse_tree(input)
}
