# TARGET_CONFIG.md (Template)

> Este archivo es tu checklist. Sin esto, no hay fuzzing estable.

---

## TARGET_REPO_URL

- URL del repositorio del target (git):

```
<REPLACE_ME>
```

---

## BUILD_FLAGS

> Flags mínimos para fuzzing in-process y coverage usable.

Ejemplo:

```
-fPIC -O1 -g -fsanitize-coverage=trace-pc-guard,trace-cmp,8bit-counters
```

Tu configuración:

```
<REPLACE_ME>
```

---

## CONFLICTING_SYMBOLS

> Elimina `main()` de la librería estática para evitar conflictos con el fuzzer.

**Cómo encontrar `main.o`:**

```bash
nm -g --defined-only build/lib/libtarget.a | grep ' main$'
```

**Cómo removerlo:**

```bash
ar d build/lib/libtarget.a main.o
```

Notas / símbolos conflictivos:

```
<REPLACE_ME>
```

---

## ENTRYPOINT_FUNCTION

> La función de C/C++ que procesa los bytes.

Ejemplo:

```
int process_packet(const uint8_t *buf, size_t len);
```

Tu entrypoint:

```
<REPLACE_ME>
```
