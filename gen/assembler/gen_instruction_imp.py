from gen.ops import OpcodeVisitor


class GenInstructionImp(OpcodeVisitor):
    def __init__(self, opcodes, dest):
        super().__init__(opcodes, dest)
        self.names = []

    def opcode(self, opcode):
        name = opcode.name
        triv = opcode.trivial_ast
        if triv:
            yield f'    trivial_ast!(rule, stmt_{name}, {name});'
        else:
            yield f'if matches!(rule, Rule::stmt_{name}) {{'
            yield f'    return super::{name.lower()}::from_parse_tree(p);'
            yield "}"

    def prefix(self):
        yield "use crate::{parser::Rule, result::AssemblerResult};"
        yield "macro_rules! trivial_ast {"
        yield "    ($val:expr,$src:ident,$dst:ident) => {"
        yield "        if matches!($val, Rule::$src) {"
        yield "            return Ok(super::Instruction::$dst);"
        yield "        }"
        yield "    };"
        yield "}"
        yield "impl super::Instruction {"
        yield "    pub(crate) fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>) -> AssemblerResult<Self> {"
        yield "        let rule = p.as_rule();"

    def suffix(self):
        yield 'Err(crate::result::AssemblerError::AstGenerationError(format!('
        yield '    "invalid rule does not match an instruction {:?}",'
        yield '    rule'
        yield ')))'
        yield '}'
        yield '}'

def gen_instr_imp(src, path):
    with open(path, "w") as dst:
        gen = GenInstructionImp(src,dst)
        gen.run()
