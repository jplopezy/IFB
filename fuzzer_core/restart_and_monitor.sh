#!/bin/bash
# Script para matar procesos, reiniciar y monitorear el fuzzer

cd /home/test/IFB/fuzzer_core

echo "=== Matando procesos del fuzzer ==="
pkill -9 -f fuzzer_main 2>/dev/null
sleep 1
if ps aux | grep -v grep | grep -q fuzzer_main; then
    echo "⚠️  Algunos procesos aún están corriendo"
    ps aux | grep -v grep | grep fuzzer_main
else
    echo "✅ Todos los procesos del fuzzer fueron terminados"
fi

echo ""
echo "=== Compilando ==="
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release 2>&1 | tail -5

echo ""
echo "=== Iniciando fuzzer ==="
rm -f fuzzer_output.log
nohup ./run_fuzzer.sh > fuzzer_output.log 2>&1 &
FUZZER_PID=$!
sleep 3

if ps -p $FUZZER_PID > /dev/null 2>&1; then
    echo "✅ Fuzzer iniciado (PID: $FUZZER_PID)"
else
    echo "❌ Fuzzer no se inició correctamente"
    echo "Últimas líneas del log:"
    tail -20 fuzzer_output.log
    exit 1
fi

echo ""
echo "=== Esperando 5 segundos para que arranque ==="
sleep 5

echo ""
echo "=== Estado inicial ==="
if [ -f "fuzzer_output.log" ]; then
    echo "Últimas 20 líneas del log:"
    tail -20 fuzzer_output.log
else
    echo "⚠️  No se encontró fuzzer_output.log"
fi

echo ""
echo "=== Monitoreo continuo (Ctrl+C para salir) ==="
echo "El fuzzer está corriendo. Monitoreando cada 10 segundos..."
echo ""

while true; do
    clear
    echo "=== Estado del Fuzzer - $(date) ==="
    echo ""
    
    if ps -p $FUZZER_PID > /dev/null 2>&1; then
        echo "✅ Fuzzer corriendo (PID: $FUZZER_PID)"
        MEM=$(ps -p $FUZZER_PID -o rss= 2>/dev/null | awk '{printf "%.1f MB", $1/1024}')
        CPU=$(ps -p $FUZZER_PID -o %cpu= 2>/dev/null | awk '{print $1"%"}')
        echo "   Memoria: $MEM | CPU: $CPU"
    else
        echo "❌ Fuzzer NO está corriendo"
        echo "Últimas líneas del log:"
        tail -10 fuzzer_output.log
        break
    fi
    
    echo ""
    echo "=== Estadísticas ==="
    if [ -f "fuzzer_output.log" ]; then
        grep -E "(Heartbeat|corpus|executions|objectives|exec/sec)" fuzzer_output.log | tail -3
    fi
    
    echo ""
    echo "=== Corpus ==="
    if [ -d "./corpus" ]; then
        CORPUS_COUNT=$(find ./corpus -type f 2>/dev/null | wc -l)
        echo "📚 Archivos: $CORPUS_COUNT"
    else
        echo "Directorio corpus no existe aún"
    fi
    
    echo ""
    echo "=== Crashes ==="
    if [ -d "./crashes" ]; then
        CRASH_COUNT=$(find ./crashes -type f 2>/dev/null | wc -l)
        if [ $CRASH_COUNT -gt 0 ]; then
            echo "🎉 ¡CRASHES ENCONTRADOS: $CRASH_COUNT!"
        else
            echo "No crashes aún"
        fi
    else
        echo "Directorio crashes no existe aún"
    fi
    
    echo ""
    echo "Últimas 3 líneas del log:"
    tail -3 fuzzer_output.log 2>/dev/null || echo "No hay log aún"
    
    echo ""
    echo "Presiona Ctrl+C para salir del monitoreo (el fuzzer seguirá corriendo)"
    sleep 10
done

