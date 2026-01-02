#!/bin/bash
# Script para verificar el estado del fuzzer

echo "=== Estado del Fuzzer ==="
echo ""

# Verificar si el proceso está corriendo
if ps aux | grep -v grep | grep -q "fuzzer_main"; then
    PID=$(pgrep -f "fuzzer_main" | head -1)
    echo "✅ Fuzzer está corriendo (PID: $PID)"
    echo "   Memoria: $(ps -p $PID -o rss= | awk '{print $1/1024 " MB"}')"
    echo "   CPU: $(ps -p $PID -o %cpu=)%"
else
    echo "❌ Fuzzer NO está corriendo"
fi

echo ""
echo "=== Últimas líneas del log ==="
if [ -f "fuzzer_output.log" ]; then
    tail -10 fuzzer_output.log
else
    echo "No se encontró fuzzer_output.log"
fi

echo ""
echo "=== Estadísticas del corpus ==="
if [ -d "./corpus" ]; then
    CORPUS_COUNT=$(find ./corpus -type f 2>/dev/null | wc -l)
    echo "Archivos en corpus: $CORPUS_COUNT"
    if [ $CORPUS_COUNT -gt 0 ]; then
        echo "Últimos 5 archivos:"
        ls -lt ./corpus | head -6 | tail -5
    fi
else
    echo "Directorio corpus no existe"
fi

echo ""
echo "=== Crashes encontrados ==="
if [ -d "./crashes" ]; then
    CRASH_COUNT=$(find ./crashes -type f 2>/dev/null | wc -l)
    echo "Crashes: $CRASH_COUNT"
    if [ $CRASH_COUNT -gt 0 ]; then
        echo "🎉 ¡CRASHES ENCONTRADOS!"
        ls -lt ./crashes | head -6 | tail -5
    fi
else
    echo "Directorio crashes no existe"
fi

echo ""
echo "=== Ejecuciones por segundo (últimas 5 líneas) ==="
if [ -f "fuzzer_output.log" ]; then
    grep -E "executions|exec/sec" fuzzer_output.log | tail -5
fi

