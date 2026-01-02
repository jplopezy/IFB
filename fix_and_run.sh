#!/bin/bash
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so

echo "🔨 Recompilando con LibAFL..."
cargo clean
cargo build --release 2>&1 | tail -20

if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo ""
    echo "✅ Compilación exitosa!"
    echo "🏃 Ejecutando nuevo fuzzer con LibAFL..."
    echo ""
    ./target/release/fuzzer_main
else
    echo "❌ Error de compilación"
    exit 1
fi

