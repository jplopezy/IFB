#!/usr/bin/env bash
set -euo pipefail

# IFB: Environment setup helper

export CC=clang-18
export CXX=clang++-18
export AR=llvm-ar-18
export RANLIB=llvm-ranlib-18

export RUSTFLAGS="-C target-cpu=native"

echo "[IFB] Environment configured: CC=${CC}, CXX=${CXX}"
