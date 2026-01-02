# 🚀 Project IFB (In-Process Fuzzing Boilerplate)
> *High Performance. Static Linking. Zero Sockets.*

![Rust](https://img.shields.io/badge/made_with-Rust-red) ![LibAFL](https://img.shields.io/badge/powered_by-LibAFL-green) ![License](https://img.shields.io/badge/license-MIT-blue)

**Project IFB** es una plantilla de ingeniería para construir fuzzers **In-Process** de alto rendimiento para targets C/C++ complejos (Apache, Nginx, OpenSSL, etc.) sin la complejidad de configurar AFLNet ni el cuello de botella del Kernel TCP/IP.



## 🔥 ¿Por qué IFB?

| Feature | Fuzzing Tradicional (AFLNet) | Project IFB (LibAFL In-Process) |
| :--- | :--- | :--- |
| **Arquitectura** | Network / Sockets | **Function Calls (Linkeado Estático)** |
| **Overhead** | Kernel, Syscalls, Context Switch | **Cero (Instrucciones CPU puras)** |
| **Velocidad** | ~200 - 2,000 exec/s | **> 50,000 exec/s** |
| **Profundidad** | Superficial (Blackbox) | **Profunda (Whitebox/Snapshot)** |
| **Crash Detection** | Timeout / Connection Reset | **Precisa (Signal Handler interno)** |

## 🛠 Requisitos

- **Docker** (Recomendado para reproducibilidad)
- **Rust Nightly**
- **Clang 18+**

## ⚡ Quick Start

1.  **Clona este repo:**
    ```bash
    git clone [https://github.com/tu-usuario/project-ifb.git](https://github.com/tu-usuario/project-ifb.git)
    cd project-ifb
    ```

2.  **Define tu Target:**
    Edita `TARGET_WORKSHEET.md`. Investiga qué librerías necesitas linkear y cuál es tu función de entrada.

3.  **Prepara el Build Script:**
    Copia `scripts/build_target_template.sh` a `scripts/build_target.sh` y rellena los TODOs para compilar tu target como librería estática (`.a`).
    *¡No olvides el paso de eliminar el símbolo `main`!*

4.  **Implementa el Harness:**
    Edita `fuzzer_core/src/harness/mod.rs`. Conecta los bytes de Rust con la función C de tu target.

5.  **Fuzz:**
    ```bash
    cd fuzzer_core
    cargo run --release
    ```

## 🧠 Filosofía "Build, Don't Configure"

No configuramos un fuzzer externo. **Construimos** un binario personalizado que contiene al fuzzer y al target en el mismo espacio de memoria.

1.  **Static Linking:** Convertimos el software objetivo en una librería (`libtarget.a`).
2.  **Main Surgery:** Extirpamos quirúrgicamente el símbolo `main()` usando `ar d`.
3.  **Virtual Context:** Simulamos el entorno (configuraciones, memoria) dentro del harness.

---
*Created by [Tu Nombre] - Based on the IFB Methodology.*
