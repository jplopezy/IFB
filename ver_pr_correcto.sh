#!/bin/bash
# Ver el código completo del PR que corrigió InProcessExecutor

cd /home/test/IFB
echo "=== Código completo del commit 4aa7b99 ==="
git show 4aa7b99:fuzzer_core/src/bin/fuzzer_main.rs | grep -A 50 "InProcessExecutor" | head -60

