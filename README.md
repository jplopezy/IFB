# 🚀 Project IFB (In-Process Fuzzing Boilerplate)
> *High Performance. Static Linking. Zero Sockets.*

**Project IFB** es una plantilla agresiva y directa para fuzzing **in-process** con Rust + LibAFL. Si tu target es C/C++ y querés velocidad real (100k exec/s), esto es tu punto de partida.

---

## 💥 ¿Por qué usar IFB?

> Dejá de usar sockets. Fuzzeá a 100k exec/s linkeando tu target como librería.

**IFB = Binario único, sin forks, sin red, sin overhead.**

- 🚀 **Speed**: elimina `fork()` y el kernel (hasta ~50x más rápido que AFL++).
- 🧠 **Smart**: arquitectura basada en LibAFL.
- 🛠 **Static**: guía para linkear `.a` directo al fuzzer.
- 🩹 **Conflict Resolver**: patrones para resolver colisiones con `main()`.

---

## ⚡ Quick Start

1. **Editá el builder del target**
   ```bash
   nano scripts/build_target.sh
   ```

2. **Definí tus headers y libs**
   - Revisá `fuzzer_core/headers.h` y `fuzzer_core/build.rs`.

3. **Implementá el harness**
   - `fuzzer_core/src/harness/mod.rs`

4. **Fuzzeá**
   ```bash
   cd fuzzer_core
   cargo run --release
   ```

---

## 🛠 Requirements

- Docker
- Rust Nightly
- Clang 18+

---

## 📁 Repo Layout

```
project-ifb/
  ├── README.md
  ├── TARGET_CONFIG.md
  ├── docker/
  │   └── Dockerfile
  ├── scripts/
  │   ├── build_target.sh
  │   └── setup_env.sh
  ├── fuzzer_core/
  │   ├── Cargo.toml
  │   ├── build.rs
  │   ├── headers.h
  │   └── src/
  │       ├── bin/
  │       │   └── fuzzer_main.rs
  │       ├── bindings/
  │       │   └── mod.rs
  │       ├── harness/
  │       │   └── mod.rs
  │       └── mutators/
  │           └── neuro_mutator.rs
  └── docs/
      ├── ARCHITECTURE.md
      └── TROUBLESHOOTING.md
```

---

*Project IFB (In-Process Fuzzing Boilerplate) — plantilla reusable para fuzzing estático y de alto rendimiento.*
