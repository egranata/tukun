from gen.ops import OpcodeVisitor


class GenInstructionLowering(OpcodeVisitor):
    def __init__(self, opcodes, dest):
        super().__init__(opcodes, dest)
        self.names = []

    def opcode(self, opcode):
        name = opcode.name
        triv = opcode.trivial_lowering
        if triv:
            yield f'    trivial_lowering!(input, {name});'
        else:
            if len(opcode.ast_args) == 0:
                yield f"    if let Instruction::{name} = input {{"
            else:
                yield f"    if let Instruction::{name}(_) = input {{"
            yield f"        return {name.lower()}::lower_instruction(ast, mdef, input, b);"
            yield "    }"

    def prefix(self):
        yield "mod fcall;"
        yield "mod fromslot;"
        yield "mod jtrue;"
        yield "mod jump;"
        yield "mod lpush;"
        yield "mod push;"
        yield "mod toslot;"
        yield "use runtime::{instruction_def::InstructionDef, module_definition::ModuleDef};"
        yield "use crate::ast::{instructions::Instruction, module::Module};"
        yield "macro_rules! trivial_lowering {"
        yield "    ($input:expr, $candidate:ident) => {"
        yield "        if matches!($input, Instruction::$candidate) {"
        yield "            return vec![InstructionDef::$candidate];"
        yield "        }"
        yield "    };"
        yield "}"
        yield "pub(crate) fn lower_instruction("
        yield "    ast: &Module,"
        yield "    mdef: &mut ModuleDef,"
        yield "    input: &Instruction,"
        yield "    b: &mut runtime::builder::Builder,"
        yield ") -> Vec<InstructionDef> {"

    def suffix(self):
        yield "    panic!("
        yield '            "instruction {:?} should have been handled but is not",'
        yield "            input"
        yield "        );"
        yield "    }"

def gen_lower(src, path):
    with open(path, "w") as dst:
        gen = GenInstructionLowering(src,dst)
        gen.run()
