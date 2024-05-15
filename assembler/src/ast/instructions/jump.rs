use crate::{parser::Rule, result::AssemblerResult};

use super::Instruction;

pub(crate) fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>) -> AssemblerResult<Instruction> {
    let rule = p.as_rule();

    if matches!(rule, Rule::stmt_JUMP) {
        let tgt = p
            .into_inner()
            .find_first_tagged("tgt")
            .expect("need a target")
            .as_str();
        Ok(Instruction::JUMP(tgt.to_owned()))
    } else {
        panic!("unexpected instruction");
    }
}
