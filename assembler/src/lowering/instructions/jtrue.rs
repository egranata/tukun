use runtime::{instruction_def::InstructionDef, module_definition::ModuleDef};

use crate::ast::{instructions::Instruction, module::Module};

pub(crate) fn lower_instruction(
    _ast: &Module,
    _mdef: &mut ModuleDef,
    input: &Instruction,
    b: &mut runtime::builder::Builder,
) -> Vec<InstructionDef> {
    if let Instruction::JTRUE(tgt) = input {
        let tgt = b.find_block(tgt).expect("missing target label");
        vec![InstructionDef::JTRUE(tgt)]
    } else {
        panic!("invalid lowering");
    }
}
