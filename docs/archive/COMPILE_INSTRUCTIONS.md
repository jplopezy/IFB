# 🔧 Instrucciones para Recompilar

Se corrigió el Cargo.toml (removida feature `alloc` que no existe).

**Para recompilar y ejecutar el nuevo código:**

```bash
cd /home/test/IFB/fuzzer_core

# Limpiar compilación anterior
cargo clean

# Recompilar
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release

# Ejecutar
export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so
./target/release/fuzzer_main
```

**O usa el script:**
```bash
cd /home/test/IFB
./fix_and_run.sh
```

El nuevo código debería mostrar:
- `[IFB] 🚀 Operation Cloud Breaker - Full LibAFL Coverage-Guided Fuzzer`
- `[IFB] 📊 Using StdFuzzer::fuzz_loop for maximum performance`

Si ves el mensaje viejo, significa que está ejecutando el binario anterior.

