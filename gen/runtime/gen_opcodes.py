from gen.ops import OpcodeVisitor


class GenOpcodesVisitor(OpcodeVisitor):
    def __init__(self,src,dst):
        super().__init__(src,dst)
        self.i = 0
    def opcode(self, opcode):
        yield f"{opcode.name} = {self.i},"
        self.i += 1
    def prefix(self):
        yield "#[repr(u8)]"
        yield "#[derive(Debug)]"
        yield "pub enum Opcode {"
    def suffix(self):
        yield "MAX,"
        yield "}"
        yield "impl From<u8> for Opcode {"
        yield "fn from(value: u8) -> Self {"
        yield "let max = unsafe { std::mem::transmute::<Opcode, u8>(Opcode::MAX) };"
        yield 'if value >= max { panic!("invalid opcode {value}") }'
        yield "unsafe { std::mem::transmute(value) }"
        yield "}"
        yield "}"
        yield "impl From<Opcode> for u8 {"
        yield "fn from(value: Opcode) -> Self {"
        yield "unsafe { std::mem::transmute(value) }"
        yield "}"
        yield "}"

def gen_opcodes(src, path):
    with open(path, "w") as dst:
        gen = GenOpcodesVisitor(src,dst)
        gen.run()
