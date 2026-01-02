# 🔄 Intento: Remover feedback de InProcessExecutor

El error sugiere que `MapFeedback` no implementa `HasObjective`, lo cual significa que `InProcessExecutor::new()` probablemente no debería tomar `feedback` directamente.

El feedback ya está en el `state`, así que el executor debería poder acceder a él a través del state.

**He removido `&mut feedback` de `InProcessExecutor::new()` nuevamente.**

Por favor prueba:

```bash
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release 2>&1 | grep -E "^error" | head -30
```

Si esto no funciona, necesito ver los detalles completos del error E0277 para entender qué espera exactamente `InProcessExecutor::new()`.

