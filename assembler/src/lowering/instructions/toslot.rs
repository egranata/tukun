use runtime::instruction_def::InstructionDef;

use crate::ast::{instructions::Instruction, module::Module};

pub(crate) fn lower_instruction(
    _mdef: &Module,
    input: &Instruction,
    _b: &mut runtime::builder::Builder,
) -> Vec<InstructionDef> {
    if let Instruction::TOSLOT(idx) = input {
        vec![InstructionDef::TOSLOT(*idx)]
    } else {
        panic!("invalid lowering");
    }
}
