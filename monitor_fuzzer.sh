#!/bin/bash
echo "🔍 Monitoring cURL Fuzzer Progress..."

while true; do
    # Check if fuzzer is still running
    if ! ps aux | grep fuzzer_main | grep -v grep > /dev/null; then
        echo "❌ Fuzzer process not found - may have crashed!"
        exit 1
    fi
    
    # Check for crashes
    CRASH_COUNT=$(ls -1 /home/test/IFB/fuzzer_core/crashes/ 2>/dev/null | wc -l)
    if [ "$CRASH_COUNT" -gt 0 ]; then
        echo "💥 CRASH DETECTED! Found $CRASH_COUNT crash(es)!"
        ls -la /home/test/IFB/fuzzer_core/crashes/
        echo "🎉 SUCCESS! Potential vulnerability found!"
        exit 0
    fi
    
    # Show current status
    echo "✅ Fuzzer running - $(date) - No crashes yet ($CRASH_COUNT files in crashes dir)"
    
    sleep 60  # Check every minute
done
