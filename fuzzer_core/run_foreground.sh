#!/bin/bash
# Ejecutar fuzzer en foreground para ver errores

cd /home/test/IFB/fuzzer_core

# Matar procesos anteriores
pkill -9 -f fuzzer_main 2>/dev/null
sleep 1

# Configurar variables
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"

# Compilar
echo "Compilando..."
cargo build --release 2>&1 | tail -3

echo ""
echo "Ejecutando fuzzer en foreground (verás todos los mensajes)..."
echo "Presiona Ctrl+C para detener"
echo ""

# Ejecutar en foreground
./run_fuzzer.sh

