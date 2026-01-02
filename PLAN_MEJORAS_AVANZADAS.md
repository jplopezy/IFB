# 🚀 Plan de Mejoras Avanzadas: LibAFL + Structure-Aware + LLM

## Objetivo

Combinar lo mejor de:
- ✅ **LibAFL** (superior a libFuzzer)
- ✅ **Structure-Aware Fuzzing** (como Trail of Bits)
- ✅ **LLM Mutations** (idea original del proyecto)

## Mejoras a Implementar

### 1. **URL Parser & Structure-Aware Mutator** 🎯

Crear un mutator que parsee URLs y mute componentes por separado:

```rust
struct ParsedURL {
    scheme: String,      // http, https, ftp
    host: String,        // example.com
    port: Option<u16>,   // 80, 443, 8080
    path: String,        // /path/to/resource
    query: String,       // ?key=value&foo=bar
    fragment: String,    // #section
    userinfo: String,    // user:pass@
}

// Mutar cada componente inteligentemente
```

**Ventajas:**
- Siempre genera URLs válidas (o casi válidas)
- Mejor coverage feedback
- Más rápido que mutar bytes crudos

### 2. **Expandir Opciones de libcurl** 📋

Agregar fuzzing de más opciones como en el artículo:

```rust
// Headers HTTP
CURLOPT_HTTPHEADER

// Cookies
CURLOPT_COOKIEFILE
CURLOPT_COOKIEJAR

// HSTS
CURLOPT_HSTS

// User-Agent
CURLOPT_USERAGENT

// Autenticación
CURLOPT_USERPWD
CURLOPT_XOAUTH2_BEARER

// Y más...
```

### 3. **LLM Mutator Mejorado con Estructura** 🧠

El LLM mutator debe:
- Recibir la URL parseada (estructura)
- Generar mutaciones inteligentes basadas en componentes
- Usar coverage feedback para evolucionar

```rust
// Si una mutación de "host" aumentó coverage:
LLM: "Este host está funcionando, genera variaciones más agresivas del host"
// Si una mutación de "path" aumentó coverage:
LLM: "Este path está explorando nuevos paths, genera paths más complejos"
```

### 4. **Diccionario de Componentes URL** 📚

Crear diccionarios para guiar mutaciones:

```rust
const URL_SCHEMES: &[&str] = &["http://", "https://", "ftp://", "file://", "://"];
const URL_PATHS: &[&str] = &["/", "/path", "/../", "/./", "/%00", "/%FF"];
const URL_QUERIES: &[&str] = &["?", "?key=value", "?%00", "?&"];
```

### 5. **Metadata Mejorada para LLM** 📊

El LLM debe saber:
- Qué componente de la URL mutó
- Si aumentó coverage
- Qué tipo de mutación funcionó

```rust
struct LLMMutationMetadata {
    component: URLComponent,  // scheme, host, path, etc.
    mutation_type: MutationType,  // insert, delete, replace
    coverage_increase: bool,
    generation: u32,
}
```

## Arquitectura Propuesta

```
┌─────────────────────────────────────────┐
│         LibAFL Fuzzer Core              │
│  (StdFuzzer, Coverage Feedback, etc.)   │
└─────────────────────────────────────────┘
                    │
        ┌───────────┴───────────┐
        │                       │
┌───────▼────────┐    ┌─────────▼──────────┐
│ Structure-Aware│    │   LLM Mutator      │
│ URL Mutator    │    │  (Evolutivo)       │
│                │    │                    │
│ - Parse URL    │    │ - Recibe estructura│
│ - Mutate parts │    │ - Genera variantes │
│ - Serialize    │    │ - Usa coverage     │
└───────┬────────┘    └─────────┬──────────┘
        │                       │
        └───────────┬───────────┘
                    │
        ┌───────────▼───────────┐
        │   libcurl Target      │
        │  (Más opciones)       │
        └───────────────────────┘
```

## Implementación por Fases

### Fase 1: URL Parser & Structure-Aware Mutator
- [ ] Crear `url_parser.rs` con `ParsedURL`
- [ ] Crear `structure_aware_mutator.rs`
- [ ] Integrar con LibAFL como mutator

### Fase 2: Expandir Opciones libcurl
- [ ] Agregar más `CURLOPT_*` al harness
- [ ] Crear seeds para diferentes opciones
- [ ] Configurar opciones globales (HSTS, cookies, etc.)

### Fase 3: LLM Mutator Mejorado
- [ ] Modificar LLM mutator para recibir estructura
- [ ] Agregar metadata de componentes
- [ ] Implementar prompts evolutivos por componente

### Fase 4: Diccionarios y Optimizaciones
- [ ] Crear diccionarios de componentes
- [ ] Optimizar mutaciones basadas en diccionarios
- [ ] Mejorar coverage feedback

## Ventajas de Este Enfoque

1. **Más rápido**: Structure-aware evita inputs inválidos
2. **Más coverage**: Más opciones de libcurl = más código probado
3. **Más inteligente**: LLM evoluciona basado en estructura
4. **Mejor que Trail of Bits**: LibAFL + LLM > libFuzzer solo

## Próximos Pasos

¿Empezamos con la Fase 1 (URL Parser & Structure-Aware Mutator)?

