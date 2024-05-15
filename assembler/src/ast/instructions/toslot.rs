use crate::{parser::Rule, result::AssemblerResult};

use super::Instruction;

pub(crate) fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>) -> AssemblerResult<Instruction> {
    let rule = p.as_rule();

    if matches!(rule, Rule::stmt_TOSLOT) {
        let idx = p
            .into_inner()
            .find_first_tagged("idx")
            .expect("need an index");
        let idx = idx.as_str().parse::<u16>().expect("invalid slot index");
        Ok(Instruction::TOSLOT(idx))
    } else {
        panic!("unexpected instruction");
    }
}
