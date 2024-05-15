from gen.ops import EnumVisitor


class GenInstructionEnum(EnumVisitor):
    def __init__(self, opcodes, dest):
        super().__init__('Instruction', opcodes, dest)

    def opcode(self, opcode):
        name = opcode.name
        args = opcode.ast_args
        if len(args) == 0:
            args = ""
        else:
            args = '(' + ",".join(args) + ')'
        yield f'{name}{args},'
    def prefix(self):
        yield "mod imp;"
        yield "mod fcall;"
        yield "mod fromslot;"
        yield "mod jtrue;"
        yield "mod jump;"
        yield "mod push;"
        yield "mod toslot;"
        yield "use either::Either;"
        for p in super().prefix():
            yield p

def gen_enum(src, path):
    with open(path, "w") as dst:
        gen = GenInstructionEnum(src,dst)
        gen.run()
