use runtime::{instruction_def::InstructionDef, module_definition::ModuleDef};

use crate::ast::{instructions::Instruction, module::Module};

pub(crate) fn lower_instruction(
    _ast: &Module,
    mdef: &mut ModuleDef,
    input: &Instruction,
    _b: &mut runtime::builder::Builder,
) -> Vec<InstructionDef> {
    if let Instruction::LPUSH(x) = input {
        let idx = mdef.add_interned_value(x.clone());
        vec![InstructionDef::PUSH(idx as u16)]
    } else {
        panic!("invalid lowering");
    }
}
