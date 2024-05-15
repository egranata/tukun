use runtime::instruction_def::InstructionDef;

use crate::ast::{instructions::Instruction, module::Module};

pub(crate) fn lower_instruction(
    mdef: &Module,
    input: &Instruction,
    _b: &mut runtime::builder::Builder,
) -> Vec<InstructionDef> {
    if let Instruction::FCALL(tgt) = input {
        let idx = mdef.constant_idx_by_name(tgt);
        let push = InstructionDef::PUSH(idx as u16);
        let flookup = InstructionDef::FLOOKUP;
        let call = InstructionDef::CALL;
        vec![push, flookup, call]
    } else {
        panic!("invalid lowering");
    }
}
