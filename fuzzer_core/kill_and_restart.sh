#!/bin/bash
cd /home/test/IFB/fuzzer_core

echo "=== Matando procesos ==="
pkill -9 -f fuzzer_main
pkill -9 -f run_fuzzer
sleep 2

echo "=== Verificando que no queden procesos ==="
if ps aux | grep -v grep | grep -q "fuzzer"; then
    echo "⚠️  Aún hay procesos corriendo:"
    ps aux | grep -v grep | grep fuzzer
    pkill -9 -f fuzzer
    sleep 1
fi

echo "=== Compilando ==="
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release 2>&1 | grep -E "(Compiling|Finished|error)" | tail -5

echo ""
echo "=== Iniciando fuzzer ==="
rm -f fuzzer_output.log
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
./run_fuzzer.sh > fuzzer_output.log 2>&1 &
FUZZER_PID=$!
echo "Fuzzer iniciado con PID: $FUZZER_PID"

echo ""
echo "=== Esperando 5 segundos ==="
sleep 5

echo ""
echo "=== Verificando estado ==="
if ps -p $FUZZER_PID > /dev/null 2>&1; then
    echo "✅ Fuzzer está corriendo (PID: $FUZZER_PID)"
    echo ""
    echo "=== Últimas líneas del log ==="
    tail -20 fuzzer_output.log
    echo ""
    echo "=== Monitoreo continuo (Ctrl+C para salir) ==="
    echo "El fuzzer seguirá corriendo en segundo plano"
    echo ""
    tail -f fuzzer_output.log
else
    echo "❌ Fuzzer se detuvo. Verificando error:"
    tail -30 fuzzer_output.log
fi

