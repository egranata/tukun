use crate::{ast::parse_string_trim, parser::Rule, result::AssemblerResult};

use super::Instruction;

pub(crate) fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>) -> AssemblerResult<Instruction> {
    let rule = p.as_rule();

    if matches!(rule, Rule::stmt_FCALL) {
        let tgt = parse_string_trim(
            p.into_inner()
                .find_first_tagged("tgt")
                .expect("need a target")
                .as_str(),
        );
        Ok(Instruction::FCALL(tgt))
    } else {
        panic!("unexpected instruction");
    }
}
