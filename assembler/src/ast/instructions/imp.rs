use crate::{parser::Rule, result::AssemblerResult};

macro_rules! trivial_ast {
    ($val:expr,$src:ident,$dst:ident) => {
        if matches!($val, Rule::$src) {
            return Ok(super::Instruction::$dst);
        }
    };
}

impl super::Instruction {
    pub(crate) fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>) -> AssemblerResult<Self> {
        let rule = p.as_rule();
        trivial_ast!(rule, stmt_ADD, ADD);
        trivial_ast!(rule, stmt_DUP, DUP);
        trivial_ast!(rule, stmt_POP, POP);
        trivial_ast!(rule, stmt_NOP, NOP);
        trivial_ast!(rule, stmt_RET, RET);
        trivial_ast!(rule, stmt_FLOOKUP, FLOOKUP);
        trivial_ast!(rule, stmt_TLOOKUP, TLOOKUP);
        trivial_ast!(rule, stmt_CALL, CALL);
        trivial_ast!(rule, stmt_NEWARR, NEWARR);
        trivial_ast!(rule, stmt_NEWREC, NEWREC);
        trivial_ast!(rule, stmt_EQUAL, EQUAL);
        trivial_ast!(rule, stmt_ARRGET, ARRGET);
        trivial_ast!(rule, stmt_ARRSET, ARRSET);
        trivial_ast!(rule, stmt_ARRLEN, ARRLEN);
        trivial_ast!(rule, stmt_TYPEOF, TYPEOF);
        trivial_ast!(rule, stmt_MKARRTYPE, MKARRTYPE);
        trivial_ast!(rule, stmt_MKRECTYPE, MKRECTYPE);
        trivial_ast!(rule, stmt_NOT, NOT);

        if matches!(rule, Rule::stmt_FCALL) {
            return super::fcall::from_parse_tree(p);
        }

        if matches!(rule, Rule::stmt_FROMSLOT) {
            return super::fromslot::from_parse_tree(p);
        }

        if matches!(rule, Rule::stmt_TOSLOT) {
            return super::toslot::from_parse_tree(p);
        }

        if matches!(rule, Rule::stmt_PUSH) {
            return super::push::from_parse_tree(p);
        }
        if matches!(rule, Rule::stmt_JUMP) {
            return super::jump::from_parse_tree(p);
        }
        if matches!(rule, Rule::stmt_JTRUE) {
            return super::jtrue::from_parse_tree(p);
        }
        Err(crate::result::AssemblerError::AstGenerationError(format!(
            "invalid rule does not match an instruction {:?}",
            rule
        )))
    }
}
