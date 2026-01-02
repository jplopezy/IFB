# ✅ Corrección: Objective con CrashFeedback

## Problema

El error era:
```
KeyExists("Tried to add a metadata of type \"libafl::feedbacks::map::MapFeedbackMetadata<u8>\" named \"edges\"")
```

Esto ocurría porque tanto `feedback` como `objective` usaban `MaxMapFeedback` con el mismo observer `edges_observer`, causando un conflicto de metadata.

## Solución

Cambiado el `objective` para usar `CrashFeedback` en lugar de `MaxMapFeedback`:

```rust
// Feedback: MaxMapFeedback determines if an input is "interesting" based on coverage
let mut feedback = MaxMapFeedback::new(&edges_observer);
// Objective: CrashFeedback detects crashes (ExitKind::Crash)
// This is separate from feedback and doesn't conflict with MaxMapFeedback metadata
let mut objective = CrashFeedback::new();
```

## Explicación

- **Feedback**: `MaxMapFeedback` se usa para determinar si un input es "interesante" (aumenta coverage)
- **Objective**: `CrashFeedback` se usa para detectar crashes (ExitKind::Crash)
- Estos dos no entran en conflicto porque usan metadata diferente

## Próximos Pasos

Recompilar y ejecutar:

```bash
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release
./run_fuzzer.sh
```

