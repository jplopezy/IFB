# 📝 Resumen de Issues para GitHub

Lista rápida de issues que deben crearse en GitHub:

## 🔴 Issue Crítico (Crear Primero)

1. **Errores de Trait Bounds con LibAFL 0.15 API**
   - `StdState` no implementa `HasObjective`
   - `SimpleEventManager` no implementa traits necesarios
   - `MapFeedback` no implementa `EventFirer` y `EventRestarter`
   - **Bloquea:** Todo el progreso de integración LibAFL 0.15

## 🟡 Issues Importantes

2. **Ownership de feedback y objective**
   - Conflictos al usar `feedback` y `objective` en múltiples componentes
   - Relacionado con Issue #1

3. **Integración de Coverage-Guided Fuzzing con LLM**
   - Completar integración una vez que Issue #1 se resuelva
   - Funcionalidad principal del proyecto

## 🟢 Issues de Documentación/Mejora

4. **Falta documentación de API de LibAFL 0.15**
   - Mejorar documentación para futuros desarrolladores

5. **Verificar SanitizerCoverage**
   - Testing y validación una vez que compile

## ✅ Issues Resueltos (Documentar para referencia)

6. **SubRangeSlice a &[u8]**
   - Ya resuelto usando `.as_slice()`
   - Puede documentarse como referencia

---

**Ver detalles completos en:** `ISSUES.md`

