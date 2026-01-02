#!/bin/bash
# Script to compile and run the LibAFL fuzzer

set -e

echo "🚀 IFB Fuzzer - Coverage-Guided Fuzzing para cURL"
echo "=================================================="
echo ""

cd "$(dirname "$0")/fuzzer_core"

# Setup environment
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so

echo "📦 Configuración:"
echo "   IFB_STATIC_LIB_DIR: $IFB_STATIC_LIB_DIR"
echo "   IFB_INCLUDE_DIR: $IFB_INCLUDE_DIR"
echo "   IFB_STATIC_LIBS: $IFB_STATIC_LIBS"
echo "   LD_PRELOAD: $LD_PRELOAD"
echo ""

# Check if libcurl exists
if [ ! -f "$IFB_STATIC_LIB_DIR/libcurl.a" ]; then
    echo "❌ Error: libcurl.a no encontrado en $IFB_STATIC_LIB_DIR"
    echo "   Ejecuta primero: cd cases/curl_easy && ./build.sh"
    exit 1
fi

echo "🔨 Compilando fuzzer..."
if cargo build --release 2>&1 | tee /tmp/ifb_compile.log; then
    echo ""
    echo "✅ Compilación exitosa!"
    echo ""
    echo "🏃 Ejecutando fuzzer..."
    echo "   (Presiona Ctrl+C para detener)"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    ./target/release/fuzzer_main
else
    echo ""
    echo "❌ Error de compilación"
    echo "   Ver logs en: /tmp/ifb_compile.log"
    echo "   Últimas líneas:"
    tail -30 /tmp/ifb_compile.log
    exit 1
fi

