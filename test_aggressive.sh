#!/bin/bash
echo "🧪 Testing aggressive URL patterns..."

cd /home/test/IFB/fuzzer_core

# Test some potentially dangerous URLs
TEST_URLS=(
    $'http://example.com/\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f'
    $'http://example.com/'$(printf '%0.s%%00' {1..1000})
    $'http://example.com/'$(printf '%0.s../' {1..1000})'etc/passwd'
    $'http://example.com/'$(python3 -c "print('A'*10000)")
    $'http://\x00\x00\x00\x00\x00\x00\x00\x00.example.com'
    $'http://example.com:99999/path'
    $'http://example.com:0/path'
    $'http://:0/'
    $'http://@/'
    $'http://user:pass@/path'
)

export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include" 
export IFB_STATIC_LIBS="curl"

echo "Testing ${#TEST_URLS[@]} aggressive URLs..."

for i in "${!TEST_URLS[@]}"; do
    URL="${TEST_URLS[$i]}"
    echo "Testing URL $((i+1))/${#TEST_URLS[@]} (length: ${#URL})"
    
    # Create a simple test program
    cat > /tmp/test_url.c << TEST_PROGRAM
#include <curl/curl.h>
#include <stdio.h>
#include <string.h>

int main() {
    CURL *curl = curl_easy_init();
    if (!curl) return 1;
    
    curl_easy_setopt(curl, CURLOPT_URL, "$URL");
    curl_easy_setopt(curl, CURLOPT_TIMEOUT_MS, 100L);
    
    CURLcode res = curl_easy_perform(curl);
    curl_easy_cleanup(curl);
    
    return res;
}
TEST_PROGRAM

    # Compile and run
    if gcc -o /tmp/test_url /tmp/test_url.c -I/home/test/IFB/cases/curl_easy/build/include -L/home/test/IFB/cases/curl_easy/build/lib -lcurl -lz -lpsl -ldl 2>/dev/null; then
        LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so timeout 5 /tmp/test_url 2>&1 | head -20
        if [ $? -ne 0 ]; then
            echo "⚠️  Potential issue with URL: $URL"
        fi
    fi
done

echo "Aggressive testing completed."
