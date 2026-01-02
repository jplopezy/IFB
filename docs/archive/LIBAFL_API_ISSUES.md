# ⚠️ Problemas con API de LibAFL 0.15

La API de LibAFL 0.15 es significativamente diferente y requiere cambios grandes:

1. `bolts` está en `libafl_bolts`, no en `libafl`
2. `BytesInput` → `ValueInput<Vec<u8>>`
3. `MaxMapFeedback::new()` solo toma 1 argumento (observer)
4. `StdState::new()` necesita 5 argumentos (incluye objective)
5. `InProcessExecutor::new()` tiene API diferente
6. `StdScheduledMutator` → `ScheduledMutator`
7. `StdMutationalStage` → `MutationalStage`

El código está siendo corregido pero la API es compleja y requiere más investigación.

