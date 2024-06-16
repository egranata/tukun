from gen.ops import EnumVisitor, OpcodeVisitor
from gen.runtime.util import RuntimeSizeMethodVisitor

class RuntimeInstructionEnumVisitor(EnumVisitor):
    def __init__(self, src,dst):
        super().__init__("RuntimeInstruction",src,dst,eq=True)
    def opcode(self, opcode):
            operands = opcode.runtime_operands
            if len(operands) == 0:
                yield f'    {opcode.name},'
            else:
                operands = "(" + ','.join(operands) + ")"
                yield f'    {opcode.name}{operands},'

class RuntimeInstructionFromBytecodeMethodVisitor(OpcodeVisitor):
    def prefix(self):
        yield "impl RuntimeInstruction {\n" + \
            "pub fn from_bytecode(bc: &crate::bytecode::Bytecode, i: usize) -> Option<(RuntimeInstruction, usize)> {\n" + \
            "let mut idx = i; let b = crate::opcodes::Opcode::from(bc.read_u8(idx)); \n" + \
            "idx += 1; match b {\n"
    def opcode(self, opcode):
        result = ""
        name = opcode.name
        runtime_operands = opcode.runtime_operands
        result = f"          crate::opcodes::Opcode::{name} => {{"
        if len(runtime_operands) == 0:
            result = result + "\n" + f"              Some((RuntimeInstruction::{name}, idx))"
        else:
            ops = ""
            for i in range(len(runtime_operands)):
                argi = runtime_operands[i]
                result = result + "\n" + f"              let arg{i} = bc.read{"_u8" if argi == "u8" else "_u16"}(idx);"
                result = result + "\n" + f"              idx += {"1" if argi == "u8" else "2"};"
                ops = ops + f"arg{i},"
            result = result + "\n" + f"              Some((RuntimeInstruction::{name}({ops}), idx))"
        result = result + "\n" + "          }"
        yield result
    def suffix(self):
        yield '_ => { None }\n}\n}\n}'

def gen_instruction_runtime(src, path):
    with open(path, "w") as dst:
        RuntimeInstructionEnumVisitor(src,dst).run()
        RuntimeInstructionFromBytecodeMethodVisitor(src,dst).run()

