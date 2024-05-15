from gen.ops import OpcodeVisitor


class GenOpcodesVisitor(OpcodeVisitor):
    def opcode(self, opcode):
        yield f"{opcode.name},"
    def prefix(self):
        yield "#[repr(u8)]\n#[derive(Debug)]\npub enum Opcode {"
    def suffix(self):
        yield \
r"    MAX," \
r"}" \
r"" \
r"impl From<u8> for Opcode {" \
r"    fn from(value: u8) -> Self {" \
r"       unsafe {" \
r"            if value >= std::mem::transmute(Opcode::MAX) {" \
r'                panic!("invalid opcode {value}")' \
r"            } else {" \
r"                std::mem::transmute(value)" \
r"            }" \
r"        }" \
r"    }" \
r"}" \
r"" \
r"impl From<Opcode> for u8 {" \
r"    fn from(value: Opcode) -> Self {" \
r"        unsafe { std::mem::transmute(value) }" \
r"    }" \
r"}"

def gen_opcodes(src, path):
    with open(path, "w") as dst:
        gen = GenOpcodesVisitor(src,dst)
        gen.run()
