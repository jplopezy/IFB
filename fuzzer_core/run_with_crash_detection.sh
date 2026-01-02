#!/bin/bash
# Wrapper script to run fuzzer and detect crashes
# The fuzzer runs in-process, and if ASan detects a bug, the process will crash
# This script monitors for crashes and saves the inputs

FUZZER_BIN="./target/release/fuzzer_main"
CRASHES_DIR="./crashes"
CRASH_COUNT=0

echo "🚀 Starting IFB Fuzzer with Crash Detection"
echo "============================================"
echo "🎯 Goal: Find REAL crashes in cURL"
echo ""

# Ensure crashes directory exists
mkdir -p "$CRASHES_DIR"

# Set up environment
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so

echo "⚙️  Environment configured"
echo "   ASan: $LD_PRELOAD"
echo ""

# Run fuzzer - it will run until it crashes or is interrupted
# When it crashes, ASan will print to stderr and the process will exit
echo "[$(date)] Starting fuzzer (will run until crash or Ctrl+C)..."
echo ""

"$FUZZER_BIN" 2>&1 | tee /tmp/fuzzer_output.log &
FUZZER_PID=$!

# Wait for fuzzer to exit (either crash or normal termination)
wait $FUZZER_PID
EXIT_CODE=$?

echo ""
echo "============================================"

if [ $EXIT_CODE -ne 0 ] && [ $EXIT_CODE -ne 130 ]; then
    # Crash detected (130 is SIGINT/Ctrl+C)
    echo "💥 CRASH DETECTED! Exit code: $EXIT_CODE"
    
    # Check if last_input exists (the input that caused the crash)
    if [ -f "$CRASHES_DIR/last_input" ]; then
        CRASH_COUNT=$((CRASH_COUNT + 1))
        CRASH_FILE="$CRASHES_DIR/crash_$(date +%s)_${CRASH_COUNT}"
        
        # Save the crash input
        cp "$CRASHES_DIR/last_input" "$CRASH_FILE"
        echo "💾 Saved crash input to: $CRASH_FILE"
        echo "   Input size: $(wc -c < "$CRASH_FILE") bytes"
        
        # Also save ASan output
        if [ -f /tmp/fuzzer_output.log ]; then
            cp /tmp/fuzzer_output.log "${CRASH_FILE}.asan"
        fi
        
        # Show ASan report
        echo ""
        echo "🔍 ASan Report:"
        echo "================"
        grep -A 50 "ERROR: AddressSanitizer\|SEGV\|heap-buffer-overflow\|stack-buffer-overflow\|use-after-free\|heap-use-after-free\|stack-use-after-return" /tmp/fuzzer_output.log | head -100 || echo "No ASan report found in output"
        echo ""
        
        echo "🏆 CRASH #${CRASH_COUNT} FOUND AND SAVED!"
        echo "   File: $CRASH_FILE"
        
        # Show first 200 bytes of the crash input
        echo ""
        echo "📄 Crash input (first 200 bytes):"
        head -c 200 "$CRASH_FILE" | hexdump -C | head -15
    else
        echo "⚠️  Crash detected but last_input not found"
    fi
else
    echo "✅ Fuzzer exited normally (Ctrl+C or normal termination)"
fi

echo ""
echo "📊 Final Statistics:"
echo "   Exit code: $EXIT_CODE"
echo "   Crashes found: $CRASH_COUNT"
if [ $CRASH_COUNT -gt 0 ]; then
    echo "   Crash files saved in: $CRASHES_DIR/"
    ls -lh "$CRASHES_DIR"/crash_* 2>/dev/null | head -10 || true
fi

