#!/usr/bin/env python3
"""Script to compile and run the IFB fuzzer"""
import subprocess
import os
import sys

def main():
    os.chdir("/home/test/IFB/fuzzer_core")
    
    env = os.environ.copy()
    env["IFB_STATIC_LIB_DIR"] = "/home/test/IFB/cases/curl_easy/build/lib"
    env["IFB_INCLUDE_DIR"] = "/home/test/IFB/cases/curl_easy/build/include"
    env["IFB_STATIC_LIBS"] = "curl"
    env["LD_PRELOAD"] = "/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so"
    
    print("🔨 Compilando fuzzer...")
    result = subprocess.run(
        ["cargo", "build", "--release"],
        env=env,
        capture_output=False
    )
    
    if result.returncode != 0:
        print("\n❌ Error de compilación")
        sys.exit(1)
    
    print("\n✅ Compilación exitosa!")
    print("\n🏃 Ejecutando fuzzer...\n")
    
    subprocess.run(
        ["./target/release/fuzzer_main"],
        env=env
    )

if __name__ == "__main__":
    main()

