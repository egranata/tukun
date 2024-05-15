#!/usr/bin/env python3

from gen.ops import OpcodeVisitor


class RuntimeSizeMethodVisitor(OpcodeVisitor):
    def __init__(self, t, src, dst):
        super().__init__(src,dst)
        self.t = t
    def prefix(self):
        yield f'impl {self.t} ' + '{\npub fn runtime_size(&self) -> usize {\nmatch self{\n'
    def opcode(self, opcode):
        name = opcode.name
        operands = opcode.runtime_operands
        if len(operands) == 0:
            yield f'            {self.t}::{name} => 1,'
        else:
            operands = " + ".join([f"core::mem::size_of::<{t}>()" for t in operands])
            yield f'            {self.t}::{name}(_) => 1 + {operands},'
    def suffix(self):
        yield "}\n}\n}"
