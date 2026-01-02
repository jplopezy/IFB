# IFB Architecture

- **Static Everything**: compile the target as `.a` and link it into the fuzzer.
- **No Main**: the fuzzer owns the process.
- **Virtual Context**: the harness simulates global state.

## Flow

1. Build target as a static library.
2. Bindgen generates Rust bindings from headers.
3. LibAFL executes the in-process loop.
