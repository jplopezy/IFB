#!/bin/bash
# Script para sincronizar y probar después del update

cd /home/test/IFB

echo "🔄 Sincronizando repositorio..."
git pull

echo ""
echo "📋 Últimos commits:"
git log --oneline -5

echo ""
echo "📝 Archivos modificados en último commit:"
git diff HEAD~1 HEAD --name-only

echo ""
echo "🔨 Compilando fuzzer..."
cd fuzzer_core

export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"

echo "Compilando..."
cargo build --release 2>&1 | tee /tmp/compile_output.txt

echo ""
echo "📊 Resultado de compilación:"
if grep -q "Finished" /tmp/compile_output.txt; then
    echo "✅ ¡Compilación exitosa!"
    echo ""
    echo "🚀 Ejecutando fuzzer..."
    ./target/release/fuzzer_main 2>&1 | head -50
else
    echo "❌ Errores de compilación encontrados:"
    echo ""
    grep -E "^error" /tmp/compile_output.txt | head -20
    echo ""
    echo "📋 Ver todos los errores:"
    echo "cat /tmp/compile_output.txt | grep -A 10 '^error'"
fi

