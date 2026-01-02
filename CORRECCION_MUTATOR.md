# 🔧 Corrección: ScheduledMutator y MutationalStage

## Error

```
error[E0782]: expected a type, found a trait
   --> src/bin/fuzzer_main.rs:110:25
    |
110 |     let havoc_mutator = ScheduledMutator::new(havoc_mutations());
    |                         ^^^^^^^^^^^^^^^^
```

`ScheduledMutator` y `MutationalStage` son traits, no tipos concretos.

## Solución Aplicada

En LibAFL 0.15, `MutationalStage::new()` toma directamente la tupla de mutaciones, no un `ScheduledMutator`.

**Cambio:**
- ❌ `let havoc_mutator = ScheduledMutator::new(havoc_mutations());`
- ✅ `let mutator = tuple_list!(havoc_mutations());`
- Luego: `MutationalStage::new(mutator)`

**Por favor prueba la compilación:**

```bash
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release
```

