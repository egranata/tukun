from gen.ops import EnumVisitor, OpcodeVisitor
from gen.runtime.util import RuntimeSizeMethodVisitor

class InstructionDefIsTerminalMethodVisitor(OpcodeVisitor):
    def prefix(self):
        yield "impl InstructionDef {\npub fn is_terminal(&self) -> bool {\nmatch self{"
    def opcode(self, opcode):
        name = opcode.name
        operands = opcode.runtime_operands
        is_terminal = opcode.is_terminal
        is_terminal = 'true' if is_terminal else 'false'
        operands = "" if len(operands) == 0 else "(_)"
        yield f'            InstructionDef::{name}{operands} => {is_terminal},'
    def suffix(self):
        yield "}\n}\n}"

class InstructionDefWriteMethodVisitor(OpcodeVisitor):
    def prefix(self):
        yield "impl InstructionDef {\npub fn write(&self, bc: &mut crate::bytecode::Bytecode) {\nmatch self{"
    def opcode(self, opcode):
        result = ""
        name = opcode.name
        runtime_operands = opcode.runtime_operands
        if len(runtime_operands) == 0:
            operands = ""
        else:
            operands = "("
            for i in range(len(runtime_operands)):
                operands += f"arg{i},"
            operands += ")"
        operand_writers = opcode.operand_writers
        result = f'            InstructionDef::{name}{operands} => {{'
        result = result + "\n" + f'               bc.write_u8(u8::from(crate::opcodes::Opcode::{name}));'
        for i in range(len(runtime_operands)):
            runtime_operand = runtime_operands[i]
            operand_writer = operand_writers[i]
            runtime_operand = f"bc.write_u8({operand_writer})" if runtime_operand == "u8" else f'bc.write_u16({operand_writer})'
            result = result + "\n" + f'               {runtime_operand};'
        result = result + "\n" + f'            }},'
        yield result
    def suffix(self):
        yield "}\n}\n}"

class InstructionDefEnumVisitor(EnumVisitor):
    def __init__(self, src,dst):
        super().__init__("InstructionDef",src,dst)
    def opcode(self, opcode):
            operands = opcode.builder_operands
            if len(operands) == 0:
                yield f'    {opcode.name},'
            else:
                operands = "(" + ','.join(operands) + ")"
                yield f'    {opcode.name}{operands},'

def gen_instruction_def(src, path):
    with open(path, "w") as dst:
        InstructionDefEnumVisitor(src,dst).run()
        RuntimeSizeMethodVisitor("InstructionDef",src,dst).run()
        InstructionDefIsTerminalMethodVisitor(src,dst).run()
        InstructionDefWriteMethodVisitor(src,dst).run()
