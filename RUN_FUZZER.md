# 🚀 Ejecutar Fuzzer - Búsqueda de Vulnerabilidades en cURL

## Comandos Rápidos

### Compilar
```bash
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release
```

### Ejecutar (sin LLM)
```bash
export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so
./target/release/fuzzer_main
```

### Ejecutar (con LLM)
```bash
# Terminal 1: Ollama
ollama serve

# Terminal 2: Fuzzer
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so
cargo run --release --features llm
```

## Lo que Verás

- Progress del fuzzer con estadísticas
- Inputs que aumentan coverage se guardan automáticamente
- Si encuentra crashes, se guardan en `./crashes/`
- LLM genera mutaciones inteligentes (si está activo)

## Detener

Ctrl+C para detener gracefully

