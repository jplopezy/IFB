# 🎯 Fuzzing cURL - Modo Original (Sin LLM)

## Idea Original IFB
- ✅ **Zero Sockets** - No network, no overhead
- ✅ **Static Linking** - Binario único
- ✅ **High Performance** - 100k+ exec/s
- ✅ **In-process** - No forks

## Ejecución SIN LLM (Modo Original)

```bash
cd /home/test/IFB/fuzzer_core

# Compilar SIN feature LLM (modo original)
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so

# Compilar sin LLM
cargo build --release  # Sin --features llm

# Ejecutar
./target/release/fuzzer_main > fuzzer_stdout.log 2> fuzzer_stderr.log
```

## Ejecución CON LLM (Modo Experimental)

```bash
# Compilar CON LLM
cargo build --release --features llm

# Ejecutar (requiere Ollama corriendo)
ollama serve &  # En otra terminal
./target/release/fuzzer_main
```

## Diferencia

- **Sin LLM**: Fuzzing tradicional rápido (mutaciones aleatorias) - **IDEAL ORIGINAL**
- **Con LLM**: Fuzzing inteligente (usa Ollama local) - **EXPERIMENTAL**

## Recomendación

Para mantener la idea original de IFB ("Zero Sockets"), ejecuta **SIN la feature LLM**.

El objetivo (encontrar crashes en cURL) se cumple en ambos modos.

