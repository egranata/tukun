#!/usr/bin/env python3

import json

from gen.util import write_header

class Opcode(object):
    def __init__(self, node):
        self.attributes = {}
        for key in node:
            value = node[key]
            self.attributes[key] = value

    def __getattr__(self, name):
        return self.attributes[name]

class OpcodeVisitor(object):
    def __init__(self, opcodes, dest):
        self.ops = opcodes
        self.dest = dest
        write_header(self.dest)
    
    def prefix(self):
        pass

    def suffix(self):
        pass

    def do_prefix(self):
        p = self.prefix()
        if p:
            for p in self.prefix():
                print(p, file=self.dest)
    
    def do_suffix(self):
        p = self.suffix()
        if p:
            for p in self.suffix():
                print(p, file=self.dest)
    
    def opcode(self, opcode):
        pass

    def do_opcode(self, opcode):
        for p in self.opcode(opcode):
            if p:
                print(p, file=self.dest)
    
    def do_opcodes(self):
        for op in self.ops:
            self.do_opcode(op)

    def run(self):
        self.do_prefix()
        self.do_opcodes()
        self.do_suffix()

def load_opcodes_from_path(path):
    nodes = json.load(open(path))
    opcodes = []
    for node in nodes:
        opcodes.append(Opcode(node))
    return opcodes

class EnumVisitor(OpcodeVisitor):
    def __init__(self,n,src,dst,eq=False):
        super().__init__(src,dst)
        self.n = n
        self.eq = eq
    def prefix(self):
        if self.eq:
            yield "#[derive(Debug, Clone, PartialEq, Eq)]"
        else:
            yield "#[derive(Debug, Clone)]"
        yield f'pub enum {self.n} {{'
    def suffix(self):
        yield "}"

