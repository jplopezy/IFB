#!/bin/bash
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
echo "=== Error E0061 (StdState::new necesita 5 argumentos) ==="
cargo build --release 2>&1 | grep -A 20 "error\[E0061\]" | head -30
echo ""
echo "=== Error E0277 (HasObjective) ==="
cargo build --release 2>&1 | grep -A 20 "error\[E0277\].*HasObjective" | head -30
echo ""
echo "=== Error E0631 (type mismatch InProcessExecutor) ==="
cargo build --release 2>&1 | grep -A 20 "error\[E0631\]" | head -30

