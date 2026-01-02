#!/usr/bin/env bash
set -euo pipefail

clang -c -O2 -fPIC vuln.c -o vuln.o
ar rcs libdummy.a vuln.o
ar d libdummy.a main.o || true
