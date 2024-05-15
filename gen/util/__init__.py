#!/usr/bin/env python3

import os,json,subprocess

def find_workspace_root():
    stdout = subprocess.check_output("cargo locate-project --workspace", shell=True)
    cargo = json.loads(stdout)['root']
    parts = os.path.split(cargo)
    return os.path.abspath(parts[0])
