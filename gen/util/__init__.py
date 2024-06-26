#!/usr/bin/env python3

import os,json,subprocess

def find_workspace_root():
    stdout = subprocess.check_output("cargo locate-project --workspace", shell=True)
    cargo = json.loads(stdout)['root']
    parts = os.path.split(cargo)
    return os.path.abspath(parts[0])

def write_header(f):
    print("// this file is autogenerated, do not edit manually", file=f)
    print("// to change this file consult gen/genall.sh", file=f)
