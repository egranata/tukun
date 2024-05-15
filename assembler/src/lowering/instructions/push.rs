use runtime::instruction_def::InstructionDef;

use crate::ast::{instructions::Instruction, module::Module};

pub(crate) fn lower_instruction(
    mdef: &Module,
    input: &Instruction,
    _b: &mut runtime::builder::Builder,
) -> Vec<InstructionDef> {
    if let Instruction::PUSH(x) = input {
        vec![InstructionDef::PUSH(match x {
            either::Either::Left(idx) => *idx,
            either::Either::Right(name) => {
                let idx = mdef.constant_idx_by_name(name);
                idx as u16
            }
        })]
    } else {
        panic!("invalid lowering");
    }
}
