use runtime::{instruction_def::InstructionDef, module_definition::ModuleDef};

use crate::ast::{instructions::Instruction, module::Module};

pub(crate) fn lower_instruction(
    _ast: &Module,
    _mdef: &mut ModuleDef,
    input: &Instruction,
    _b: &mut runtime::builder::Builder,
) -> Vec<InstructionDef> {
    if let Instruction::FROMSLOT(idx) = input {
        vec![InstructionDef::FROMSLOT(*idx)]
    } else {
        panic!("invalid lowering");
    }
}
