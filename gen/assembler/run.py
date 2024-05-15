#!/usr/bin/env python3

import os
from gen.ops import load_opcodes_from_path
from gen.assembler import gen_instruction_enum, gen_pest
from gen.util import find_workspace_root


def main():
    cargo_root = find_workspace_root()
    assembler_ops = os.path.join(cargo_root, "assembler", "src", "ops.json")
    pest_path = os.path.join(cargo_root, "assembler", "src", "tukun.pest.ops")
    ast_instr_enum_path = os.path.join(cargo_root, "assembler", "src", "ast", "instructions", "mod.rs")
    src = load_opcodes_from_path(assembler_ops)
    gen_pest.gen_grammar(src, pest_path)
    gen_instruction_enum.gen_enum(src, ast_instr_enum_path)

if __name__ == "__main__":
    main()
