#!/usr/bin/env bash
set -euo pipefail

# cURL Easy Fuzzing Script for IFB

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IFB_ROOT="$(cd "${SCRIPT_DIR}/../../" && pwd)"
CURL_BUILD_DIR="${SCRIPT_DIR}/build"

echo "[IFB-cURL] Setting up environment..."

# Setup IFB environment
source "${IFB_ROOT}/scripts/setup_env.sh"

# Configure paths for cURL
export IFB_STATIC_LIB_DIR="${CURL_BUILD_DIR}/lib"
export IFB_INCLUDE_DIR="${CURL_BUILD_DIR}/include"
export IFB_STATIC_LIBS="curl"

echo "[IFB-cURL] Building fuzzer with cURL..."
cd "${IFB_ROOT}/fuzzer_core"
cargo build --release

echo "[IFB-cURL] Fuzzer built successfully!"
echo "[IFB-cURL] To run fuzzing:"
echo "  cd ${IFB_ROOT}/fuzzer_core && cargo run --release"
echo "[IFB-cURL] Or run directly:"
echo "  ${IFB_ROOT}/fuzzer_core/target/release/fuzzer_main"
