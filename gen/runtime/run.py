#!/usr/bin/env python3

import os
from gen.ops import load_opcodes_from_path
from gen.runtime import gen_instruction_def, gen_opcodes, gen_runtime_def
from gen.util import find_workspace_root


def main():
    cargo_root = find_workspace_root()
    runtime_ops = os.path.join(cargo_root, "runtime", "src", "ops.json")
    opcodes_path = os.path.join(cargo_root, "runtime", "src", "opcodes.rs")
    inst_def_path = os.path.join(cargo_root, "runtime", "src", "instruction_def.rs")
    inst_rt_path = os.path.join(cargo_root, "runtime", "src", "instruction_runtime.rs")
    src = load_opcodes_from_path(runtime_ops)
    gen_opcodes.gen_opcodes(src, opcodes_path)
    gen_instruction_def.gen_instruction_def(src, inst_def_path)
    gen_runtime_def.gen_instruction_runtime(src, inst_rt_path)

if __name__ == "__main__":
    main()
