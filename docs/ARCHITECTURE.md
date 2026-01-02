# IFB Architecture

- **Static Everything**: el target se compila como `.a` y se linkea al fuzzer.
- **No Main**: el fuzzer es dueño del proceso.
- **Virtual Context**: el harness simula el entorno global.

## Flow

1. Build target as static library.
2. Bindgen generates Rust bindings from headers.
3. LibAFL executes in-process loop.
