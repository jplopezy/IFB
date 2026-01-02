# Operation Cloud Breaker: cURL Trophy Case

## Objective
Hunt for real-world parsing bugs by fuzzing the cURL URL parser via `curl_easy_setopt`.
The harness drives `CURLOPT_URL` with fuzzed input, then executes a minimal transfer
path using `curl_easy_perform`. Any AddressSanitizer red alert (e.g., heap buffer
overflow) indicates a potentially exploitable defect worth escalation.

## How to Build
Run the build script to fetch and compile a static, ASan-instrumented libcurl:

```bash
cd cases/curl_easy
./build.sh
```

The library installs into `cases/curl_easy/build`.

## How to Run
1. Link the static libcurl into the IFB harness build. Point your build system at:
   * `cases/curl_easy/build/lib/libcurl.a`
   * `cases/curl_easy/build/include`
2. Replace the IFB harness implementation with `cases/curl_easy/harness.rs` (or
   wire its `init_target`/`fuzz_iteration` into your existing harness module).
3. Run your usual `fuzzer_core` flow to begin fuzzing.

## Expected Outcome
If ASan triggers a red alert (heap buffer overflow, UAF, etc.), we have a strong
candidate for a new CVE. Preserve the crashing input and reduce it for triage.
