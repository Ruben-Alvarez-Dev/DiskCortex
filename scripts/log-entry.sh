#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════
# DISKCORTEX LOG ENTRY SCRIPT
# Sistema de logging estructurado siguiendo ELITE_SCRUM_TEMPLATE
# ═══════════════════════════════════════════════════════════════════════════

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuración
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_DIR="$PROJECT_ROOT/project/logs"
MAX_ENTRIES=500

# Función para obtener emoji según tipo
get_emoji() {
    local type="$1"
    case "$type" in
        CREATE)   echo "📝" ;;
        MODIFY)   echo "🔧" ;;
        DELETE)   echo "❌" ;;
        MIGRATE)  echo "🔄" ;;
        DECISION) echo "🧠" ;;
        SNAPSHOT) echo "📸" ;;
        RED)      echo "🔴" ;;
        GREEN)    echo "🟢" ;;
        REFACTOR) echo "🔵" ;;
        DONE)     echo "✅" ;;
        WIP)      echo "🚧" ;;
        WARN)     echo "⚠️" ;;
        ERROR)    echo "❗" ;;
        TEST)     echo "🧪" ;;
        BUILD)    echo "🏗️" ;;
        DEPLOY)   echo "🚀" ;;
        *)        echo "📌" ;;
    esac
}

# Función de ayuda
show_help() {
    echo "Uso: $0 <TIPO> <descripción>"
    echo ""
    echo "Tipos disponibles:"
    echo "  CREATE   - Creación de archivos/servicios"
    echo "  MODIFY   - Modificación de existentes"
    echo "  DELETE   - Eliminación"
    echo "  MIGRATE  - Migración entre ubicaciones"
    echo "  DECISION - Decisión arquitectónica"
    echo "  SNAPSHOT - Snapshot de seguridad"
    echo "  RED      - Test escrito (TDD)"
    echo "  GREEN    - Test pasando"
    echo "  REFACTOR - Código mejorado"
    echo "  DONE     - Tarea completada"
    echo "  WIP      - En progreso"
    echo "  WARN     - Advertencia"
    echo "  ERROR    - Error"
    echo "  TEST     - Tests ejecutados"
    echo "  BUILD    - Build/compilación"
    echo "  DEPLOY   - Deploy/despliegue"
    echo ""
    echo "Ejemplo:"
    echo "  $0 CREATE \"apiClient.ts - Cliente HTTP para daemon API\""
    echo "  $0 MODIFY \"App.tsx - Eliminado mock data, conectado a API real\""
}

# Validar argumentos
if [ $# -lt 2 ]; then
    show_help
    exit 1
fi

TYPE=$(echo "$1" | tr '[:lower:]' '[:upper:]')
DESCRIPTION="$2"
DETAILS="${3:-}"

# Obtener emoji
EMOJI=$(get_emoji "$TYPE")
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
DATE_FILE=$(date '+%Y%m%d')

# Determinar archivo de log (paginación)
LOG_FILE="$LOG_DIR/changes-${DATE_FILE}.log"

# Crear directorio si no existe
mkdir -p "$LOG_DIR"

# Obtener hash de git si está disponible
GIT_HASH=$(git rev-parse --short HEAD 2>/dev/null || echo 'no-git')

# Construir entrada de log
LOG_ENTRY="[$TIMESTAMP] $EMOJI $TYPE: $DESCRIPTION"
if [ -n "$DETAILS" ]; then
    LOG_ENTRY="$LOG_ENTRY
├─ $DETAILS
└─ $GIT_HASH"
else
    LOG_ENTRY="$LOG_ENTRY
└─ $GIT_HASH"
fi

# Escribir al archivo de log
echo -e "$LOG_ENTRY\n" >> "$LOG_FILE"

# Output al usuario
echo -e "${CYAN}════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ Log entry registrado${NC}"
echo -e "${CYAN}════════════════════════════════════════${NC}"
echo -e "$LOG_ENTRY"
echo -e "${CYAN}════════════════════════════════════════${NC}"
echo -e "Archivo: ${YELLOW}$LOG_FILE${NC}"
