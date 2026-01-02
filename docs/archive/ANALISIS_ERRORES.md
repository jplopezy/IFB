# 🔍 Análisis de Errores Actuales

Los errores muestran que `InProcessExecutor::new()` espera los argumentos en un orden diferente:

## Errores Clave:

1. **`SimpleEventManager` se está pasando como `S` (state)** cuando debería ser `EM` (event manager)
   - Error dice: `HasExecutions` está implementado para `StdState`, no para `SimpleEventManager`
   - Esto significa que estamos pasando `mgr` donde debería ir `state`

2. **`StdState` se está pasando donde debería ir el event manager**
   - Error sobre `HasObjective` sugiere que `state` está siendo usado como `Z` (que debe implementar `HasObjective`)

3. **`MapFeedback` (objective) está siendo usado como event manager**
   - Error: `MapFeedback` no implementa `EventFirer` y `EventRestarter`
   - Esto sugiere que `objective` está donde debería ir `mgr`

## Conclusión:

Los argumentos de `InProcessExecutor::new()` están en el orden incorrecto. Necesitamos ver cómo se hizo en el PR que se mergeó.

