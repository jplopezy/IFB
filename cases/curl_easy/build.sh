#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PREFIX_DIR="${SCRIPT_DIR}/build"

if command -v clang-18 >/dev/null 2>&1; then
  CC_BIN="clang-18"
else
  CC_BIN="clang"
fi

CFLAGS="-O1 -g -fPIC -fsanitize=address,fuzzer-no-link -fno-omit-frame-pointer"
LDFLAGS="-fsanitize=address,fuzzer-no-link"

TMP_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "${TMP_DIR}"
}
trap cleanup EXIT

rm -rf "${PREFIX_DIR}"
mkdir -p "${PREFIX_DIR}"

cd "${TMP_DIR}"

git clone --depth 1 https://github.com/curl/curl.git
cd curl

if [ -x "./buildconf" ]; then
  ./buildconf
fi

CC="${CC_BIN}" \
CFLAGS="${CFLAGS}" \
LDFLAGS="${LDFLAGS}" \
./configure \
  --disable-shared \
  --enable-static \
  --disable-ldap \
  --disable-ldaps \
  --without-ssl \
  --prefix="${PREFIX_DIR}"

make -j"$(nproc)"
make install
