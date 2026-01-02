#!/usr/bin/env bash
set -euo pipefail

# IFB: Universal Target Builder (Template)

TARGET_DIR="${TARGET_DIR:-target_src}"

# TODO: Clone target repository
# git clone <REPO_URL> "${TARGET_DIR}"

# TODO: Configure build (autotools/cmake/etc)
# cd "${TARGET_DIR}"
# ./configure CFLAGS="<FLAGS>" --disable-shared
# or
# cmake -S . -B build -DCMAKE_C_FLAGS="<FLAGS>" -DBUILD_SHARED_LIBS=OFF

# TODO: Build static library (.a)
# make -C build

# [IFB TIP] Remove main symbol to avoid linker errors
# ar d build/lib/libtarget.a main.o

# TODO: Copy libtarget.a and headers to a known location
# cp build/lib/libtarget.a /path/to/ifb/target/lib/
# cp -r include /path/to/ifb/target/include/
