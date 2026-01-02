# 🐛 Issues de GitHub - Problemas Encontrados

Este documento lista todos los issues que deben ser creados en el repositorio de GitHub.

---

## Issue #1: Errores de Trait Bounds con LibAFL 0.15 API

**Título:** `StdState` no implementa `HasObjective` - Errores de compatibilidad con LibAFL 0.15

**Etiquetas:** `bug`, `libafl`, `api-compatibility`, `blocking`

**Prioridad:** Alta

### Descripción

Al intentar integrar LibAFL 0.15 completo en el fuzzer, encontramos múltiples errores de trait bounds que sugieren incompatibilidad con la API esperada.

### Errores Específicos

1. **`StdState` no implementa `HasObjective`**
   ```
   error[E0277]: the trait bound `StdState<InMemoryCorpus<ValueInput<...>>, ..., ..., ...>: HasObjective` is not satisfied
   ```
   - El `InProcessExecutor::new()` requiere que el state implemente `HasObjective`
   - Pero `StdState` no lo implementa directamente

2. **`SimpleEventManager` no implementa traits necesarios**
   ```
   error[E0277]: the trait bound `SimpleEventManager<...>: HasExecutions` is not satisfied
   error[E0277]: the trait bound `SimpleEventManager<...>: HasSolutions<...>` is not satisfied
   error[E0277]: the trait bound `SimpleEventManager<...>: HasCurrentTestcase<...>` is not satisfied
   ```

3. **`MapFeedback` no implementa `EventFirer` y `EventRestarter`**
   ```
   error[E0277]: the trait bound `MapFeedback<...>: EventFirer<..., ...>` is not satisfied
   error[E0277]: the trait bound `MapFeedback<...>: EventRestarter<...>` is not satisfied
   ```

### Código Afectado

- `fuzzer_core/src/bin/fuzzer_main.rs` - Líneas 77-105 (creación de state, executor)

### Intentos Realizados

1. ✅ Corregido `StdState::new()` para usar 5 argumentos: `(rand, corpus, solutions, &mut feedback, &mut objective)`
2. ✅ Intentado pasar `&mut feedback` a `InProcessExecutor::new()` - falla porque `MapFeedback` no implementa `HasObjective`
3. ✅ Intentado pasar `&mut objective` como 5to argumento - errores de trait bounds
4. ✅ Removido feedback de `InProcessExecutor::new()` - error E0061 (necesita 5 argumentos)

### Contexto

Estamos intentando integrar LibAFL 0.15 completo para habilitar fuzzing guiado por cobertura (coverage-guided fuzzing) con feedback del LLM.

### Referencias

- Documentación: `ESTADO_CRITICO.md`, `RECOMENDACION_FINAL.md`
- Intentos: `INTENTO_*.md`, `CORRECCIONES_*.md`

### Posibles Soluciones

1. Buscar ejemplos funcionales de LibAFL 0.15 que demuestren la estructura correcta
2. Revisar la documentación oficial de LibAFL 0.15 para entender la API correcta
3. Considerar usar una versión diferente de LibAFL si la API es muy compleja
4. Consultar con la comunidad de LibAFL sobre estos errores

---

## Issue #2: Conversión de SubRangeSlice a &[u8] en target_function

**Título:** `SubRangeSlice` no se convierte directamente a `&[u8]` en `target_function`

**Etiquetas:** `bug`, `libafl`, `type-conversion`

**Prioridad:** Media (Ya resuelto, pero documentar para referencia)

### Descripción

Cuando usamos `ValueInput::sub_bytes(..)` obtenemos un `SubRangeSlice<'_, u8>` que no se puede pasar directamente a funciones que esperan `&[u8]`.

### Solución Aplicada

✅ Usar `.as_slice()` para convertir:
```rust
let bytes = input.sub_bytes(..);
harness::fuzz_iteration(bytes.as_slice());
```

### Estado

Resuelto, pero documentado para referencia futura.

---

## Issue #3: Ownership de feedback y objective en múltiples lugares

**Título:** Conflictos de ownership al usar `feedback` y `objective` en múltiples componentes

**Etiquetas:** `bug`, `rust`, `ownership`, `libafl`

**Prioridad:** Alta

### Descripción

Los objetos `feedback` y `objective` (tipo `MaxMapFeedback`) se necesitan en múltiples lugares:

1. `StdState::new()` - requiere `&mut feedback` y `&mut objective`
2. `InProcessExecutor::new()` - puede requerir `&mut feedback` o `&mut objective` (dependiendo de la API)
3. `StdFuzzer::new()` - requiere `feedback` y `objective` (ownership)

### Problema

En Rust, no podemos tener múltiples referencias mutables al mismo valor simultáneamente, ni podemos mover un valor después de prestarlo como referencia mutable.

### Código Afectado

- `fuzzer_core/src/bin/fuzzer_main.rs` - Líneas 43-127

### Solución Propuesta

Necesitamos entender cómo LibAFL 0.15 maneja esto:
- ¿El feedback se copia/clona entre componentes?
- ¿Hay una estructura diferente que debemos usar?
- ¿El feedback se almacena en el state y se accede desde ahí?

---

## Issue #4: Falta documentación/clarificación de API de LibAFL 0.15

**Título:** Documentación insuficiente para integrar LibAFL 0.15 completamente

**Etiquetas:** `documentation`, `libafl`, `enhancement`

**Prioridad:** Media

### Descripción

La integración completa de LibAFL 0.15 requiere documentación más clara sobre:

1. La firma exacta de `StdState::new()` y qué tipos espera
2. La firma exacta de `InProcessExecutor::new()` y qué argumentos necesita
3. Cómo estructurar feedback y objective correctamente
4. Cómo manejar ownership de feedback entre componentes

### Solución Propuesta

1. Buscar ejemplos funcionales en el repositorio oficial de LibAFL
2. Crear documentación propia basada en lo que aprendamos
3. Documentar la estructura correcta una vez que funcione

### Referencias

- Repositorio oficial: https://github.com/AFLplusplus/LibAFL
- Documentación: https://aflplus.plus/libafl-book/

---

## Issue #5: Integración de Coverage-Guided Fuzzing con LLM

**Título:** Completar integración de feedback de cobertura con mutador LLM

**Etiquetas:** `feature`, `llm`, `coverage-guided`, `enhancement`

**Prioridad:** Alta

### Descripción

Una vez que se resuelvan los errores de LibAFL 0.15, necesitamos:

1. ✅ Asegurar que el `StdMapObserver` capture correctamente la cobertura
2. ✅ Integrar `MaxMapFeedback` para identificar inputs "interesantes"
3. ⏳ Actualizar `LLMMutator` para usar información de cobertura del corpus
4. ⏳ Implementar prompt evolutivo basado en inputs que aumentaron cobertura

### Estado Actual

- El código base está preparado pero no compila debido a los errores de trait bounds
- `LLMMutator` está actualizado para usar `ValueInput<Vec<u8>>`
- La estructura de coverage-guided está implementada pero no funcional

### Próximos Pasos

1. Resolver Issue #1 (errores de trait bounds)
2. Verificar que el coverage map se está capturando correctamente
3. Implementar lógica evolutiva en `LLMMutator` usando metadata del corpus

---

## Issue #6: Verificar que SanitizerCoverage está funcionando correctamente

**Título:** Validar que la instrumentación de cobertura está activa y funcionando

**Etiquetas:** `testing`, `coverage`, `sanitizer`

**Prioridad:** Media

### Descripción

Aunque compilamos libcurl con `-fsanitize-coverage=trace-pc-guard`, necesitamos verificar que:

1. El coverage map (`COVERAGE_MAP`) se está actualizando correctamente
2. `__sanitizer_cov_trace_pc_guard` está siendo llamado
3. El `StdMapObserver` está leyendo el mapa correctamente

### Verificación Necesaria

1. Agregar logs/debugging para ver si el coverage map cambia
2. Verificar que los inputs diferentes producen diferentes patrones de cobertura
3. Asegurar que inputs "interesantes" se están identificando correctamente

### Referencias

- `cases/curl_easy/build.sh` - Flags de compilación
- `fuzzer_core/src/bin/fuzzer_main.rs` - Coverage map y observer

---

## Template para Crear Issues en GitHub

Para cada issue, puedes copiar el formato siguiente:

```markdown
**Título:** [Título del issue]

**Etiquetas:** [etiquetas relevantes]

**Prioridad:** [Alta/Media/Baja]

## Descripción
[Descripción del problema]

## Errores/Comportamiento
[Detalles específicos]

## Código Afectado
[Ubicaciones de archivos y líneas]

## Solución Propuesta
[Ideas para resolver]

## Referencias
[Archivos de documentación relacionados]
```

