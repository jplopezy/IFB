# 🎯 Idea Original vs Implementación Actual

## ✅ Idea Original

1. **Fuzzer rápido con LibAFL** (in-process, static linking)
2. **LLM que se ajusta según code coverage** (feedback-driven)
   - El LLM recibe información de qué inputs aumentaron coverage
   - Genera mutaciones inteligentes basadas en coverage
   - Evoluciona hacia inputs que maximizan coverage

## ⚠️ Estado Actual

### ✅ Lo que SÍ tenemos:
- ✅ Fuzzer rápido (in-process, static linking)
- ✅ LLM mutator (Ollama local)
- ✅ Metadata system (para rastrear generaciones)

### ❌ Lo que FALTA:
- ❌ **NO estamos usando LibAFL completamente** (loop simple)
- ❌ **NO estamos capturando code coverage**
- ❌ **LLM NO recibe información de coverage**
- ❌ **LLM NO sabe qué inputs aumentaron coverage**

## 🔧 Para Cumplir la Idea Original

### Opción 1: Integración Completa LibAFL (Recomendado)

Necesitaríamos refactorizar para usar:
- `StdFuzzer` de LibAFL
- `MapFeedback` para trackear coverage
- `Coverage` feedback para medir code coverage
- Mutator integrado con LibAFL que recibe feedback

El LLM mutator recibiría:
```rust
// Pseudocódigo
fn mutate(&mut self, state: &mut S, input: &mut I, _stage_idx: i32) -> Result<MutationResult, Error> {
    // 1. Obtener coverage del input actual
    let coverage = state.feedback().coverage_map();
    
    // 2. Obtener inputs exitosos (que aumentaron coverage)
    let successful_inputs = state.corpus().inputs_with_increased_coverage();
    
    // 3. Preguntar al LLM basándose en coverage
    let prompt = format!(
        "These inputs increased code coverage: {:?}. 
         Generate a mutation that explores new paths.",
        successful_inputs
    );
    
    // 4. LLM genera mutación basada en coverage
    // ...
}
```

### Opción 2: Simulación Simple (Rápido)

Mantener el loop simple pero agregar:
- Tracking básico de coverage (ej: usando `__sanitizer_cov_trace_pc_guard`)
- Pasar información de coverage al LLM en el prompt

## 📊 Comparación

| Aspecto | Idea Original | Estado Actual |
|---------|---------------|---------------|
| Fuzzer rápido | ✅ | ✅ |
| LibAFL completo | ✅ Necesario | ❌ Loop simple |
| Code coverage tracking | ✅ Necesario | ❌ No implementado |
| LLM usa coverage | ✅ Necesario | ❌ Solo usa prompts estáticos |
| Feedback-driven | ✅ Necesario | ⚠️ Parcial (solo metadata) |

## 🎯 Conclusión

**Nos falta la parte crítica**: integrar code coverage para que el LLM pueda ajustarse según el feedback de coverage.

¿Quieres que implemente la integración completa con LibAFL + Coverage feedback?

