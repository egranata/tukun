use runtime::instruction_def::InstructionDef;

use crate::ast::{instructions::Instruction, module::Module};

pub(crate) fn lower_instruction(
    _mdef: &Module,
    input: &Instruction,
    _b: &mut runtime::builder::Builder,
) -> Vec<InstructionDef> {
    if let Instruction::FROMSLOT(idx) = input {
        vec![InstructionDef::FROMSLOT(*idx)]
    } else {
        panic!("invalid lowering");
    }
}
