from gen.ops import OpcodeVisitor


class GenGrammarVisitor(OpcodeVisitor):
    def __init__(self, opcodes, dest):
        super().__init__(opcodes, dest)
        self.names = []

    def opcode(self, opcode):
        name = opcode.name
        token_name = f'stmt_{name.upper()}'
        self.names.append(token_name)
        suffix = ""
        if opcode.pest_args != "":
            suffix = f'~ {opcode.pest_args}'
        yield f'{token_name} = {{^"{opcode.name.lower()}" {suffix}}}'
    def prefix(self):
        pass
    def suffix(self):
        yield 'statement = {' + ' | '.join(self.names) + '}'

def gen_grammar(src, path):
    with open(path, "w") as dst:
        gen = GenGrammarVisitor(src,dst)
        gen.run()
