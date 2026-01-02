#!/usr/bin/env bash
set -euo pipefail

root_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root_dir"

cargo build --release

echo "[IFB] Running smoke test for ~10s (baseline, no LLM)"
IFB_TARGET=dummy timeout 10s ./target/release/fuzzer_main --stats-interval 2
