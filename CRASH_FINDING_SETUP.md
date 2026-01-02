# 🎯 Configuración para Búsqueda de Crashes en cURL

## Cambios Realizados

### 1. **Harness Modificado** (`fuzzer_core/src/harness/mod.rs`)
- ✅ **Removido `catch_unwind`** - Ahora ASan puede detectar crashes y terminar el proceso
- ✅ El proceso se terminará si ASan detecta un bug de memoria

### 2. **Fuzzer Mejorado** (`fuzzer_core/src/bin/fuzzer_main.rs`)
- ✅ **Corpus agresivo**: 60+ casos edge incluyendo:
  - Null bytes y control characters
  - Inputs muy largos (10,000 bytes)
  - Encoding malformado
  - Format string attacks
  - Invalid IPv6
  - Nested protocols
- ✅ **Mutaciones agresivas**: 10 tipos diferentes de mutaciones
- ✅ **LLM Mutator aumentado**: 20% probabilidad (antes 10%)
- ✅ **Guardado de último input**: Antes de cada ejecución se guarda en `crashes/last_input`

### 3. **Script de Detección** (`fuzzer_core/run_with_crash_detection.sh`)
- ✅ Detecta cuando el proceso crashea
- ✅ Guarda el input que causó el crash
- ✅ Muestra el reporte de ASan
- ✅ Guarda el output completo

## 🚀 Ejecución

### Opción 1: Ejecutar con script wrapper (recomendado)

```bash
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so

./run_with_crash_detection.sh
```

### Opción 2: Ejecutar directamente

```bash
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so

./target/release/fuzzer_main > fuzzer_stdout.log 2> fuzzer_stderr.log
```

Si el fuzzer crashea, verifica:
- `crashes/last_input` - Contiene el input que causó el crash
- `fuzzer_stderr.log` - Contiene el reporte de ASan

## 📊 Qué Esperar

### Si encuentra un crash:
1. El proceso terminará inmediatamente
2. ASan imprimirá un reporte detallado en stderr
3. El archivo `crashes/last_input` contendrá el input que causó el crash
4. El script wrapper guardará el crash en `crashes/crash_<timestamp>_<num>`

### Si NO encuentra crashes:
- El fuzzer continuará ejecutándose indefinidamente
- Verás estadísticas cada 1000 iteraciones
- Puedes detenerlo con Ctrl+C

## 🎯 Objetivo

**ENCONTRAR UN CRASH REAL EN cURL**

El fuzzer está configurado para ser MUY agresivo:
- Inputs peligrosos (null bytes, inputs largos, encoding malformado)
- Mutaciones agresivas (repetir inputs, insertar nulls, format strings)
- LLM mutator con prompts diseñados para causar crashes

## ⚠️ Nota Importante

El fuzzer hace requests HTTP reales, así que verás respuestas de servidores web en la salida. Esto es normal - el parser de URLs se ejecuta ANTES de la conexión, así que si hay un bug en el parser, ASan lo detectará antes de que se intente la conexión.

## 📝 Verificación de Crashes

Para verificar si hay crashes guardados:

```bash
cd /home/test/IFB/fuzzer_core
ls -lh crashes/
cat crashes/last_input | head -c 200 | hexdump -C
```

Si hay un crash, también deberías ver un reporte de ASan en stderr que indique:
- `ERROR: AddressSanitizer`
- `heap-buffer-overflow`, `stack-buffer-overflow`, `use-after-free`, etc.

