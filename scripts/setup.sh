#!/usr/bin/env bash
set -euo pipefail

root_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

cd "$root_dir"

echo "[IFB] Checking Rust toolchain..."
if ! command -v rustc >/dev/null 2>&1; then
  echo "[IFB] ERROR: rustc not found. Install Rust via https://rustup.rs" >&2
  exit 1
fi

required_channel=$(awk -F '"' '/channel/ {print $2}' rust-toolchain.toml || true)
current_version=$(rustc --version)

if [[ -n "${required_channel}" ]]; then
  echo "[IFB] Required toolchain: ${required_channel}"
fi

echo "[IFB] Detected: ${current_version}"

echo "[IFB] Checking build dependencies..."
for cmd in clang pkg-config; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "[IFB] Missing dependency: $cmd" >&2
    echo "[IFB] Install it using your system package manager (e.g. apt-get install -y $cmd)." >&2
    exit 1
  fi
done

if ! pkg-config --exists libcurl; then
  echo "[IFB] libcurl development files not found via pkg-config." >&2
  echo "[IFB] Install libcurl dev package (e.g. apt-get install -y libcurl4-openssl-dev)." >&2
  exit 1
fi

echo "[IFB] Environment looks good."
