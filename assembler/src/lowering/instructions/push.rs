use runtime::{instruction_def::InstructionDef, module_definition::ModuleDef};

use crate::ast::{instructions::Instruction, module::Module};

pub(crate) fn lower_instruction(
    ast: &Module,
    _mdef: &mut ModuleDef,
    input: &Instruction,
    _b: &mut runtime::builder::Builder,
) -> Vec<InstructionDef> {
    if let Instruction::PUSH(x) = input {
        vec![InstructionDef::PUSH(match x {
            either::Either::Left(idx) => *idx,
            either::Either::Right(name) => {
                let idx = ast.constant_idx_by_name(name);
                idx as u16
            }
        })]
    } else {
        panic!("invalid lowering");
    }
}
