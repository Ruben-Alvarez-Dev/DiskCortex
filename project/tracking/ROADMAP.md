# 🗺️ DiskCortex - Roadmap

> Última actualización: 2026-03-04
> Estado: Desarrollo Activo - Fase 1 (Core API)

---

## 📊 Estado General

| Componente | Progreso | Estado |
|------------|----------|--------|
| **OpenAPI Spec** | 100% | ✅ Completado |
| **Daemon API** | 5% | 🚧 En Progreso |
| **TUI** | 40% | 🚧 Básico Funcional |
| **GUI** | 20% | 🚧 Estructura + API Client |
| **Tests** | 5% | ⚠️ Crítico - Necesita TDD |

---

## 🎯 Épicas Actuales

### EPIC-001: Core API Implementation
**Prioridad:** CRÍTICA | **Sprint:** Actual

| ID | Tarea | Estado | Notas |
|----|-------|--------|-------|
| API-001 | Health endpoint | ✅ Done | /health funcionando |
| API-002 | Auth endpoints | ❌ Todo | /auth/login, /auth/logout |
| API-003 | Registry endpoints | ❌ Todo | CRUD de herramientas |
| API-004 | Discovery scan | ❌ Todo | Auto-detección de tools |
| API-005 | Cleanup execution | ❌ Todo | Borrado seguro |

### EPIC-002: GUI React Application
**Prioridad:** ALTA | **Sprint:** Actual

| ID | Tarea | Estado | Notas |
|----|-------|--------|-------|
| GUI-001 | Eliminar mock data | ✅ Done | Conectado a API real |
| GUI-002 | API Client (Axios) | ✅ Done | apiClient.ts creado |
| GUI-003 | Tool Service | ✅ Done | toolService.ts creado |
| GUI-004 | Loading/Error states | ✅ Done | UX mejorada |
| GUI-005 | Auth UI | ❌ Todo | Login/logout |
| GUI-006 | Cleanup UI | 🚧 WIP | Placeholder existe |

### EPIC-003: Testing & Quality
**Prioridad:** CRÍTICA | **Sprint:** Próximo

| ID | Tarea | Estado | Notas |
|----|-------|--------|-------|
| TEST-001 | Unit tests daemon | ❌ Todo | Target: 90% coverage |
| TEST-002 | Integration tests | ❌ Todo | Target: 80% coverage |
| TEST-003 | E2E tests GUI | ❌ Todo | Target: 70% coverage |
| TEST-004 | TDD workflow | ❌ Todo | RED-GREEN-REFACTOR |

---

## 🚨 Deuda Técnica Identificada

| ID | Problema | Severidad | Acción |
|----|----------|-----------|--------|
| DEBT-001 | Mock data en producción | 🔴 CRÍTICO | ✅ RESUELTO |
| DEBT-002 | println! en producción | 🟡 HIGH | ✅ RESUELTO (tracing) |
| DEBT-003 | Funciones >20 líneas | 🟡 HIGH | commands.rs, settings.rs |
| DEBT-004 | Sin CI/CD | 🟡 HIGH | Crear pipeline |
| DEBT-005 | Coverage <10% | 🔴 CRÍTICO | Implementar TDD |

---

## 📅 Timeline

### Sprint Actual (Marzo 2026)
- [x] Eliminar mock data
- [x] Crear API Client
- [x] Implementar tracing
- [ ] Completar auth endpoints
- [ ] Tests básicos

### Próximo Sprint
- [ ] Discovery scan implementation
- [ ] Registry CRUD
- [ ] Aumentar coverage a 50%

### Backlog
- [ ] Docker integration
- [ ] Scheduled cleanups
- [ ] Compliance features

---

## 🔗 Referencias

- [OpenAPI Spec](../docs/specs/openapi.yaml)
- [ADR-001: Tech Stack](../docs/architecture/adr/001-tech-stack.md)
- [ADR-002: Safe Deletion](../docs/architecture/adr/002-safe-deletion-protocol.md)
- [AUDIT_REPORT.md](../AUDIT_REPORT.md)
