# 🚀 Ejecutar el Fuzzer

## Problema: ASan Runtime

El error `ASan runtime does not come first in initial library list` ocurre porque ASan necesita ser preloaded antes de ejecutar el programa.

## Solución

Ejecuta el fuzzer con `LD_PRELOAD` apuntando a `libasan.so`:

```bash
cd /home/test/IFB/fuzzer_core
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
export LD_PRELOAD=/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so
./target/release/fuzzer_main
```

## Script Automático

He creado un script que hace esto automáticamente:

```bash
cd /home/test/IFB/fuzzer_core
./run_fuzzer.sh
```

## Encontrar libasan.so

Si la ruta `/usr/lib/gcc/x86_64-linux-gnu/13/libasan.so` no existe, busca la librería:

```bash
find /usr/lib -name "libasan.so*" 2>/dev/null
```

Luego usa la ruta encontrada en `LD_PRELOAD`.

## Verificación

Una vez ejecutado correctamente, deberías ver:
- El banner del fuzzer
- Estadísticas de ejecución
- Información sobre el corpus
- El fuzzer comenzando a mutar inputs

