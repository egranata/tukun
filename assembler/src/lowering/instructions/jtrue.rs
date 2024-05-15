use runtime::instruction_def::InstructionDef;

use crate::ast::{instructions::Instruction, module::Module};

pub(crate) fn lower_instruction(
    _mdef: &Module,
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
