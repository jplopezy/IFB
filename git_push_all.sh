#!/bin/bash
# Script para hacer commit y push de todos los cambios

cd /home/test/IFB

echo "=== Verificando estado del repositorio ==="
git status --short | head -20

echo ""
echo "=== Agregando todos los cambios ==="
git add -A

echo ""
echo "=== Estado después de git add ==="
git status --short | head -30

echo ""
echo "=== Haciendo commit ==="
git commit -m "feat: Complete LibAFL integration with coverage-guided fuzzing and LLM mutator

- Full LibAFL 0.15 integration with StdFuzzer::fuzz_loop
- Coverage-guided fuzzing with MaxMapFeedback and StdMapObserver
- Structure-aware mutator for intelligent URL component mutations
- LLM mutator (30% probability) for evolutionary fuzzing when corpus stagnates
- Comprehensive cURL knowledge base (schemes, headers, protocols, RFCs)
- URL parser for structured mutations
- Optimized harness with 1ms timeouts and HEAD requests for fast fuzzing
- Fixed all NonZeroUsize issues in mutators
- Added extensive logging and diagnostics
- Created monitoring and restart scripts"

echo ""
echo "=== Haciendo push ==="
git push origin main

echo ""
echo "=== Estado final ==="
git status

