# ✅ Corrección: Feature serdeany_autoreg

## Problema

El fuzzer fallaba con este error:
```
Empty types registry. Please enable the `serdeany_autoreg` feature in libafl_bolts
```

## Solución

Agregado el feature `serdeany_autoreg` a `libafl_bolts` en `Cargo.toml`:

```toml
libafl_bolts = { version = "0.15", default-features = false, features = ["std", "serdeany_autoreg"] }
```

## Próximos Pasos

1. **Recompilar**:
   ```bash
   cd /home/test/IFB/fuzzer_core
   export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
   export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
   export IFB_STATIC_LIBS="curl"
   cargo build --release
   ```

2. **Ejecutar**:
   ```bash
   ./run_fuzzer.sh
   ```

O manualmente:
   ```bash
   export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so
   ./target/release/fuzzer_main
   ```

