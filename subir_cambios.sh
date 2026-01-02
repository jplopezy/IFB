#!/bin/bash
# Script para subir todos los cambios al repositorio

cd /home/test/IFB

echo "📋 Verificando estado del repositorio..."
git status

echo ""
echo "➕ Agregando todos los archivos..."
git add -A

echo ""
echo "📝 Estado después de agregar archivos:"
git status

echo ""
echo "💾 Creando commit..."
git commit -m "Trabajo completo: Integración LibAFL 0.15, correcciones aplicadas y documentación de errores

- Refactorización completa hacia LibAFL 0.15 API
- Actualizado fuzzer_main.rs con StdFuzzer, InProcessExecutor, StdMapObserver
- Actualizado neuro_mutator.rs para ValueInput<Vec<u8>>
- Múltiples correcciones aplicadas (StdState::new, InProcessExecutor::new, etc.)
- Documentación completa del proceso: errores, intentos, recomendaciones
- Archivos de estado: ESTADO_CRITICO.md, RECOMENDACION_FINAL.md
- Estado actual: errores de trait bounds con LibAFL 0.15 API requieren más investigación"

echo ""
echo "🚀 Subiendo cambios al repositorio..."
git push

echo ""
echo "✅ ¡Completado!"

