#!/bin/bash
echo "=== Procesos del Fuzzer ==="
ps aux | grep -E "fuzzer_main|run_fuzzer" | grep -v grep || echo "No hay procesos corriendo"

echo ""
echo "=== Últimas 30 líneas del log ==="
tail -30 /home/test/IFB/fuzzer_core/fuzzer_output.log 2>/dev/null || echo "No hay log"

echo ""
echo "=== Tamaño del log ==="
wc -l /home/test/IFB/fuzzer_core/fuzzer_output.log 2>/dev/null || echo "No hay log"

echo ""
echo "=== Corpus ==="
ls -la /home/test/IFB/fuzzer_core/corpus/ 2>/dev/null | head -10 || echo "Corpus vacío"

echo ""
echo "=== Crashes ==="
ls -la /home/test/IFB/fuzzer_core/crashes/ 2>/dev/null || echo "No hay crashes"

