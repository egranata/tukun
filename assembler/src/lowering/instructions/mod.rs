// this file is autogenerated, do not edit manually
// to change this file consult gen/genall.sh
mod fcall;
mod fromslot;
mod jtrue;
mod jump;
mod lpush;
mod push;
mod toslot;
use crate::ast::{instructions::Instruction, module::Module};
use runtime::{instruction_def::InstructionDef, module_definition::ModuleDef};
macro_rules! trivial_lowering {
    ($input:expr, $candidate:ident) => {
        if matches!($input, Instruction::$candidate) {
            return vec![InstructionDef::$candidate];
        }
    };
}
pub(crate) fn lower_instruction(
    ast: &Module,
    mdef: &mut ModuleDef,
    input: &Instruction,
    b: &mut runtime::builder::Builder,
) -> Vec<InstructionDef> {
    trivial_lowering!(input, NOP);
    trivial_lowering!(input, ADD);
    trivial_lowering!(input, SUB);
    trivial_lowering!(input, RET);
    trivial_lowering!(input, FLOOKUP);
    trivial_lowering!(input, TLOOKUP);
    trivial_lowering!(input, CALL);
    trivial_lowering!(input, NEWARR);
    trivial_lowering!(input, NEWREC);
    trivial_lowering!(input, EQUAL);
    trivial_lowering!(input, NOT);
    trivial_lowering!(input, DUP);
    trivial_lowering!(input, SWAP);
    trivial_lowering!(input, POP);
    trivial_lowering!(input, ARRGET);
    trivial_lowering!(input, ARRSET);
    trivial_lowering!(input, ARRLEN);
    trivial_lowering!(input, RECGET);
    trivial_lowering!(input, RECSET);
    trivial_lowering!(input, TYPEOF);
    trivial_lowering!(input, MKARRTYPE);
    trivial_lowering!(input, MKRECTYPE);
    if let Instruction::PUSH(_) = input {
        return push::lower_instruction(ast, mdef, input, b);
    }
    if let Instruction::LPUSH(_) = input {
        return lpush::lower_instruction(ast, mdef, input, b);
    }
    if let Instruction::JUMP(_) = input {
        return jump::lower_instruction(ast, mdef, input, b);
    }
    if let Instruction::JTRUE(_) = input {
        return jtrue::lower_instruction(ast, mdef, input, b);
    }
    if let Instruction::FCALL(_) = input {
        return fcall::lower_instruction(ast, mdef, input, b);
    }
    if let Instruction::FROMSLOT(_) = input {
        return fromslot::lower_instruction(ast, mdef, input, b);
    }
    if let Instruction::TOSLOT(_) = input {
        return toslot::lower_instruction(ast, mdef, input, b);
    }
    panic!(
        "instruction {:?} should have been handled but is not",
        input
    );
}
