use either::Either;

use crate::{ast::parse_string_trim, parser::Rule, result::AssemblerResult};

use super::Instruction;

pub(crate) fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>) -> AssemblerResult<Instruction> {
    let rule = p.as_rule();

    if matches!(rule, Rule::stmt_PUSH) {
        let idx = p
            .into_inner()
            .find_first_tagged("idx")
            .expect("need an index");
        let idx: Either<u16, String> = match idx.as_rule() {
            Rule::number => {
                Either::Left(idx.as_str().parse::<u16>().expect("invalid numeric index"))
            }
            Rule::string => Either::Right(parse_string_trim(idx.as_str())),
            _ => panic!("unexpected index {idx}"),
        };
        Ok(Instruction::PUSH(idx))
    } else {
        panic!("unexpected instruction");
    }
}
