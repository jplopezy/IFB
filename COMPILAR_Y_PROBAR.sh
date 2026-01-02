#!/bin/bash

cd /home/test/IFB/fuzzer_core

export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"

echo "🔨 Compilando fuzzer con todas las mejoras..."
cargo build --release 2>&1 | tee /tmp/compile_output.txt

if [ $? -eq 0 ]; then
    echo "✅ Compilación exitosa!"
    echo ""
    echo "🚀 Ejecutando fuzzer..."
    export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so
    ./target/release/fuzzer_main
else
    echo "❌ Errores de compilación. Revisa /tmp/compile_output.txt"
    tail -50 /tmp/compile_output.txt
fi

