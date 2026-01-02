# ⚠️ Plan de Respaldo: API LibAFL 0.15 Compleja

La API de LibAFL 0.15 es significativamente diferente y está causando muchos errores de compilación.

## Problemas Encontrados

1. ✅ Corregido: `bolts` → `libafl_bolts`
2. ✅ Corregido: `BytesInput` → `ValueInput<Vec<u8>>`
3. ✅ Corregido: `MaxMapFeedback::new()` solo 1 argumento
4. ⚠️ En proceso: `StdState::new()` necesita 5 argumentos
5. ⚠️ En proceso: `InProcessExecutor::new()` API diferente
6. ✅ Corregido: Nombres de mutators/stages

## Opciones

### Opción A: Continuar corrigiendo (recomendado)
- Seguir corrigiendo errores uno por uno
- La API es compleja pero debería funcionar eventualmente

### Opción B: Versión híbrida
- Mantener loop simple pero agregar coverage tracking manual
- Más simple pero menos "LibAFL completo"

### Opción C: Usar código viejo funcionando
- El código viejo funciona y está ejecutando
- Podemos mejorarlo incrementalmente

¿Cuál prefieres?

