#!/bin/sh

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

PYTHONPATH=${SCRIPT_DIR}/.. ${SCRIPT_DIR}/runtime/run.py
PYTHONPATH=${SCRIPT_DIR}/.. ${SCRIPT_DIR}/assembler/run.py
