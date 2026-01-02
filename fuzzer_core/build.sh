#!/bin/bash
# Script para compilar el fuzzer

export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"

echo "🔨 Compilando fuzzer..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ ¡Compilación exitosa!"
    echo "🚀 Ejecutar con: ./target/release/fuzzer_main"
else
    echo ""
    echo "❌ Error en la compilación"
    exit 1
fi

