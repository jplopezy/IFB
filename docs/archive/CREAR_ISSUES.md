# 📋 Guía para Crear Issues en GitHub

Este documento explica cómo crear los issues documentados en `ISSUES.md`.

## Pasos para Crear Issues

### Opción 1: Desde la Interfaz Web de GitHub

1. Ve a tu repositorio en GitHub
2. Haz clic en la pestaña "Issues"
3. Haz clic en "New Issue"
4. Copia y pega el contenido de cada issue desde `ISSUES.md`
5. Agrega las etiquetas apropiadas
6. Asigna prioridad si es relevante
7. Haz clic en "Submit new issue"

### Opción 2: Usando GitHub CLI (gh)

Si tienes `gh` instalado, puedes crear issues desde la línea de comandos:

```bash
cd /home/test/IFB

# Issue #1: Errores de Trait Bounds
gh issue create \
  --title "StdState no implementa HasObjective - Errores de compatibilidad con LibAFL 0.15" \
  --label "bug,libafl,api-compatibility,blocking" \
  --body-file <(cat <<'EOF'
[Pega aquí el contenido del Issue #1 desde ISSUES.md]
EOF
)

# Issue #2: SubRangeSlice (ya resuelto, pero puede documentarse)
gh issue create \
  --title "SubRangeSlice no se convierte directamente a &[u8] en target_function" \
  --label "bug,libafl,type-conversion" \
  --body-file <(cat <<'EOF'
[Pega aquí el contenido del Issue #2 desde ISSUES.md]
EOF
)

# Continuar con los demás issues...
```

### Opción 3: Script Automático

Puedo crear un script que genere los issues automáticamente si tienes `gh` configurado.

## Issues Recomendados para Crear Primero

1. **Issue #1** - Es el más crítico y bloquea todo el progreso
2. **Issue #3** - Relacionado con el Issue #1
3. **Issue #5** - Es la funcionalidad principal que queremos lograr

Los issues #2 y #4 pueden esperar o combinarse con otros.

## Notas

- Asegúrate de que las etiquetas existan en tu repositorio, o créalas primero
- Puedes agregar milestones si estás usando project management
- Considera crear un proyecto de GitHub para organizar estos issues

