use runtime::intern_value::InternValue;

use crate::{ast::parse_string_trim, parser::Rule, result::AssemblerResult};

use super::Instruction;

pub(crate) fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>) -> AssemblerResult<Instruction> {
    let rule = p.as_rule();

    if matches!(rule, Rule::stmt_LPUSH) {
        let val = p
            .into_inner()
            .find_first_tagged("val")
            .expect("need a value to push");
        let val = match val.as_rule() {
            Rule::float => {
                InternValue::Float(val.as_str().parse::<f64>().expect("invalid floating value"))
            }
            Rule::integer => {
                InternValue::Integer(val.as_str().parse::<u64>().expect("invalid integer value"))
            }
            Rule::string => InternValue::String(parse_string_trim(val.as_str())),
            _ => panic!("unexpected value {val}"),
        };
        Ok(Instruction::LPUSH(val))
    } else {
        panic!("unexpected instruction");
    }
}
