# ✅ Fuzzer Listo para Ejecutar

## Estado

La refactorización completa está implementada:

1. ✅ **neuro_mutator.rs** - Implementa trait Mutator con coverage feedback
2. ✅ **fuzzer_main.rs** - Usa StdFuzzer::fuzz_loop completo
3. ✅ **build.sh** - Coverage flags agregados
4. ✅ **Cargo.toml** - Dependencias correctas

## Para Compilar y Ejecutar

```bash
cd /home/test/IFB/fuzzer_core

export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so

# Compilar
cargo build --release

# Ejecutar
./target/release/fuzzer_main
```

## Con LLM (Opcional)

```bash
# Asegúrate de que Ollama esté corriendo
ollama serve &

# Compilar con feature LLM
cargo build --release --features llm

# Ejecutar
./target/release/fuzzer_main
```

## Qué Buscar

1. **Crashes**: Se guardarán en `./crashes/`
2. **Corpus**: Inputs interesantes en `./corpus/`
3. **Coverage**: El fuzzer trackeará coverage y solo guardará inputs que lo aumenten
4. **LLM**: Si está activo, generará mutaciones basadas en inputs que aumentaron coverage

## Objetivo

🎯 **Encontrar vulnerabilidades reales en cURL usando coverage-guided fuzzing con LLM**

