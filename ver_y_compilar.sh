#!/bin/bash
cd /home/test/IFB

echo "=== Ver código del PR (commit 4aa7b99) ==="
git show 4aa7b99:fuzzer_core/src/bin/fuzzer_main.rs | grep -A 15 "InProcessExecutor::new"

echo ""
echo "=== Compilar ==="
cd fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release 2>&1 | tail -50

