#!/bin/bash
# Script para monitorear el progreso del fuzzer

echo "=== Estado del Fuzzer ==="
echo ""

# Verificar si el proceso está corriendo
if ps aux | grep -v grep | grep -q "fuzzer_main"; then
    PID=$(pgrep -f "fuzzer_main" | head -1)
    echo "✅ Fuzzer está corriendo (PID: $PID)"
    
    # Memoria y CPU
    MEM=$(ps -p $PID -o rss= 2>/dev/null | awk '{printf "%.1f MB", $1/1024}')
    CPU=$(ps -p $PID -o %cpu= 2>/dev/null | awk '{print $1"%"}')
    echo "   Memoria: $MEM"
    echo "   CPU: $CPU"
    echo ""
    
    # Tiempo de ejecución
    ETIME=$(ps -p $PID -o etime= 2>/dev/null | tr -d ' ')
    echo "   Tiempo de ejecución: $ETIME"
else
    echo "❌ Fuzzer NO está corriendo"
    echo ""
fi

echo "=== Últimas estadísticas del log ==="
if [ -f "fuzzer_output.log" ]; then
    echo ""
    # Buscar las últimas líneas con estadísticas
    grep -E "(Heartbeat|corpus|executions|objectives|exec/sec)" fuzzer_output.log | tail -3
    echo ""
    echo "Últimas 5 líneas del log:"
    tail -5 fuzzer_output.log
else
    echo "No se encontró fuzzer_output.log"
fi

echo ""
echo "=== Corpus ==="
if [ -d "./corpus" ]; then
    CORPUS_COUNT=$(find ./corpus -type f 2>/dev/null | wc -l)
    echo "📚 Archivos en corpus: $CORPUS_COUNT"
    if [ $CORPUS_COUNT -gt 0 ]; then
        echo "   Últimos archivos agregados:"
        ls -lt ./corpus 2>/dev/null | head -6 | tail -5 | awk '{print "   - " $9 " (" $6 " " $7 " " $8 ")"}'
    fi
else
    echo "Directorio corpus no existe aún"
fi

echo ""
echo "=== Crashes ==="
if [ -d "./crashes" ]; then
    CRASH_COUNT=$(find ./crashes -type f 2>/dev/null | wc -l)
    if [ $CRASH_COUNT -gt 0 ]; then
        echo "🎉 ¡CRASHES ENCONTRADOS: $CRASH_COUNT!"
        ls -lt ./crashes 2>/dev/null | head -6 | tail -5 | awk '{print "   - " $9 " (" $6 " " $7 " " $8 ")"}'
    else
        echo "No crashes encontrados aún"
    fi
else
    echo "Directorio crashes no existe aún"
fi

echo ""
echo "=== Ejecuciones por segundo ==="
if [ -f "fuzzer_output.log" ]; then
    # Extraer el último exec/sec
    LAST_EXEC=$(grep "exec/sec" fuzzer_output.log | tail -1 | grep -oE "exec/sec: [0-9.]+" | cut -d: -f2 | tr -d ' ')
    if [ -n "$LAST_EXEC" ]; then
        echo "⚡ Último exec/sec: $LAST_EXEC"
    else
        echo "No hay datos de exec/sec aún"
    fi
fi

