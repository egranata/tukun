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

pub fn parse_integer_value(s: &str) -> Result<u64, std::num::ParseIntError> {
    if let Some(hex_digits) = s.strip_prefix('x') {
        let hex_digits = hex_digits.strip_suffix(';').expect("invalid hex digits");
        u64::from_str_radix(hex_digits, 16)
    } else {
        s.parse::<u64>()
    }
}
