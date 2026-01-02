# 🔄 Intento: Sin MutationalStage, Usar Mutaciones Directamente

He removido `StdMutationalStage::new()` y estoy usando las mutaciones directamente en la tupla de stages.

**Cambios:**
- ❌ `StdMutationalStage::new(mutator)`
- ✅ `mutator` directamente en `tuple_list!`

En LibAFL 0.15, es posible que los stages se creen de manera diferente o que las mutaciones se pasen directamente.

**Prueba:**

```bash
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release 2>&1 | grep -E "^(Compiling|Finished|error)" | head -30
```

Si esto no funciona, necesitamos ver exactamente cómo se hace en el PR que se mergeó.

