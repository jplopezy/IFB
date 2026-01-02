# 🔄 Intento: Corregir Orden de Argumentos y StdMutationalStage

Basándome en los errores, he hecho dos cambios:

1. **Cambiado el orden de argumentos en `InProcessExecutor::new()`:**
   - ❌ Antes: `(harness, observers, state, mgr, objective)`
   - ✅ Ahora: `(harness, observers, mgr, state)` - removido objective, cambiado orden

2. **Usado `StdMutationalStage::new()` en lugar de `MutationalStage::new()`:**
   - `MutationalStage` es un trait, `StdMutationalStage` es el tipo concreto

**Prueba:**

```bash
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release
```

Si esto no funciona, necesitamos ver exactamente qué hizo el PR que se mergeó.

