# 🔧 Compilar y Ver Errores

Ejecuta esto para ver los errores actuales:

```bash
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release 2>&1 | grep -E "^error" | head -20
```

Los errores principales que necesitamos corregir son sobre la API de LibAFL 0.15, específicamente cómo se pasan feedback y objective a `StdState::new()` y cómo se usa después.

