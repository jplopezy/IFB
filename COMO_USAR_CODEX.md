# 📚 Cómo Usar Codex en Cursor

## En el Chat (Como Estamos Haciendo)

En este contexto, **ya estamos usando las herramientas disponibles** que son similares a Codex:

- ✅ Ejecuto comandos de terminal (`run_terminal_cmd`)
- ✅ Leo y edito archivos (`read_file`, `write`, `search_replace`)
- ✅ Busco en el código (`grep`, `codebase_search`)
- ✅ Compilo y verifico errores

## Si Quieres Usar Codex Específicamente

Si tienes la extensión Codex instalada, puedes:

1. **En el chat de Cursor:**
   - Menciona `@Codex` para activar comandos específicos
   - O simplemente pregunta directamente (como estás haciendo ahora)

2. **Comandos útiles con Codex:**
   - `@Codex run cargo build --release` - Ejecutar comandos
   - `@Codex explain [código]` - Explicar código
   - `@Codex fix [error]` - Corregir errores

## Para Este Proyecto

Ya tenemos todo lo necesario. El código está corregido según el PR. Solo necesitas:

```bash
cd /home/test/IFB/fuzzer_core
./build.sh
```

O manualmente:
```bash
export IFB_STATIC_LIB_DIR="/home/test/IFB/cases/curl_easy/build/lib"
export IFB_INCLUDE_DIR="/home/test/IFB/cases/curl_easy/build/include"
export IFB_STATIC_LIBS="curl"
cargo build --release
```

## ¿Qué Prefieres Hacer Ahora?

1. **Compilar el fuzzer** - El código está listo
2. **Usar Codex para algo específico** - Dime qué necesitas
3. **Continuar corrigiendo errores** - Si aparecen al compilar

