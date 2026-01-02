#!/bin/bash
cd /home/test/IFB/fuzzer_core
pkill -9 -f fuzzer_main 2>/dev/null
sleep 1
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release
rm -f fuzzer_output.log
nohup ./run_fuzzer.sh > fuzzer_output.log 2>&1 &
echo "Fuzzer iniciado. PID: $!"
sleep 3
tail -20 fuzzer_output.log

