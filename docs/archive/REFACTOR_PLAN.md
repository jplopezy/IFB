# 🔧 Plan de Refactorización: LibAFL Full Integration

## Estado Actual vs Objetivo

### ❌ Problema Actual
- Loop simple sin coverage feedback
- LLM mutator no usa información de coverage
- No usa arquitectura LibAFL completa

### ✅ Objetivo
- StdFuzzer::fuzz_loop con coverage feedback
- LLM mutator que usa coverage para guiar mutaciones
- Arquitectura LibAFL completa (Observer, Feedback, Executor, Fuzzer)

## Implementación Paso a Paso

### Fase 1: Setup Básico LibAFL ✅
- [x] Actualizar Cargo.toml con dependencias
- [x] Actualizar build.sh con sanitize-coverage
- [ ] Crear estructura básica de fuzzer_main

### Fase 2: Coverage Observer ⏳
- [ ] Implementar StdMapObserver
- [ ] Setup coverage map (65536 bytes)
- [ ] Integrar con sanitizer coverage

### Fase 3: Feedback System ⏳
- [ ] MaxMapFeedback para coverage
- [ ] MapFeedbackState
- [ ] Determinar qué inputs son "interesting"

### Fase 4: Executor ⏳
- [ ] InProcessExecutor
- [ ] Integrar harness::fuzz_iteration
- [ ] Reset coverage map antes de cada ejecución

### Fase 5: LLM Mutator con Coverage ⏳
- [ ] Implementar trait Mutator de LibAFL
- [ ] Leer metadata de coverage del input
- [ ] Prompt engineering basado en coverage:
  - Input nuevo en corpus → "Este input aumentó coverage, explora más profundo"
  - Input con historia → "Esta variación funciona, duplica la estrategia"

### Fase 6: Fuzzer Loop ⏳
- [ ] StdFuzzer::fuzz_loop
- [ ] Scheduler (QueueScheduler)
- [ ] Corpus management
- [ ] Stages con mutators

## Nota Importante

La API de LibAFL 0.15 es compleja y requiere una estructura muy específica. 
Necesitamos crear una implementación que compile y funcione paso a paso.

