#!/bin/bash

# Script para ejecutar el fuzzer con ASan preloaded

cd /home/test/IFB/fuzzer_core

# Configurar variables de entorno
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"

# Preload ASan runtime (requerido para que ASan funcione correctamente)
export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so

# Si la ruta anterior no existe, intentar otras rutas comunes
if [ ! -f "$LD_PRELOAD" ]; then
    # Intentar encontrar libasan.so
    ASAN_LIB=$(find /usr/lib -name "libasan.so*" 2>/dev/null | head -1)
    if [ -n "$ASAN_LIB" ]; then
        export LD_PRELOAD="$ASAN_LIB"
        echo "[IFB] Usando ASan desde: $LD_PRELOAD"
    else
        echo "[IFB] ⚠️  Advertencia: No se encontró libasan.so, ASan puede no funcionar correctamente"
    fi
fi

echo "[IFB] 🚀 Ejecutando fuzzer..."
echo "[IFB] 📊 Coverage-guided fuzzing con LibAFL"
echo ""

# Ejecutar el fuzzer
./target/release/fuzzer_main

