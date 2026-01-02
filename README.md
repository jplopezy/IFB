# IFB (Instalación de Fuzzing Berreta)

IFB is a high-throughput, in-process LibAFL fuzzer template focused on coverage-guided fuzzing. The default path is **no-LLM** and optimized for throughput. LLM support is **opt-in** and only used for **seed injection when the fuzzer stagnates**.

## Build (reproducible)

```bash
./scripts/setup.sh
./scripts/build_release.sh
```

This repo pins the Rust toolchain in `rust-toolchain.toml` and uses a locked dependency graph (`Cargo.lock`). See [docs/BUILD.md](docs/BUILD.md) for update instructions.

## Run baseline (no LLM)

```bash
./target/release/fuzzer_main --stats-interval 5
```

Notes:
- Default target is libcurl (fast URL parsing path).
- To run a fast local dummy target (no external dependencies), set:

```bash
IFB_TARGET=dummy ./target/release/fuzzer_main --stats-interval 2
```

## Optional LLM seed injection

LLM support is **opt-in** and never runs in the hot loop. It only triggers when the fuzzer stagnates (no new coverage for a configurable time).

```bash
cargo build --release --features llm
# Either provide a local URL...
export IFB_LLM_URL="http://127.0.0.1:11434/api/generate"
# ...or set an API key (for external providers)
export OPENAI_API_KEY="..."
./target/release/fuzzer_main \
  --stats-interval 5 \
  --llm-stagnation-secs 30 \
  --llm-batch 4
```

When enabled you will see logs for:
- stagnation trigger,
- LLM batch size,
- seeds rejected by sanitization,
- seeds accepted into corpus after positive signal.

## Smoke test

```bash
./scripts/smoke_run.sh
```

## Documentation

- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- [docs/BUILD.md](docs/BUILD.md)
- [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)
