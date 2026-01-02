# TARGET_CONFIG.md (Template)

> This file is your checklist. No stability without it.

---

## TARGET_REPO_URL

- Git repository URL for the target:

```
<REPLACE_ME>
```

---

## BUILD_FLAGS

> Minimum flags for in-process fuzzing and usable coverage.

Example:

```
-fPIC -O1 -g -fsanitize-coverage=trace-pc-guard,trace-cmp,8bit-counters
```

Your configuration:

```
<REPLACE_ME>
```

---

## CONFLICTING_SYMBOLS

> Remove `main()` from the static library to avoid fuzzer conflicts.

**Find `main.o`:**

```bash
nm -g --defined-only build/lib/libtarget.a | grep ' main$'
```

**Remove it:**

```bash
ar d build/lib/libtarget.a main.o
```

Notes / conflicting symbols:

```
<REPLACE_ME>
```

---

## ENTRYPOINT_FUNCTION

> The C/C++ function that processes input bytes.

Example:

```
int process_packet(const uint8_t *buf, size_t len);
```

Your entrypoint:

```
<REPLACE_ME>
```
