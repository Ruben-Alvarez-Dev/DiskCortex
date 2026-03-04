# 🗺️ DiskCortex - Roadmap

> Última actualización: 2026-03-04 10:04
> Estado: Desarrollo Activo - Fase 1 (Core API)
> **Fuente única de verdad:** docs/specs/openapi.yaml

---

## 📊 Estado General

| Componente | Progreso | Estado |
|------------|----------|--------|
| **OpenAPI Spec** | 100% | ✅ Completado - CONTRATO DEFINIDO |
| **Daemon API** | 5% | 🚧 Solo /health |
| **GUI** | 20% | 🚧 Conectada a API (sin mock data) |
| **Tests** | 5% | ⚠️ Crítico |

---

## 🎯 ROADMAP CRISTALINO (basado en OpenAPI)

### FASE 1: Auth & Users (PRIORIDAD 1)
**Por qué:** Sin auth no hay seguridad, todo lo demás depende

| Endpoint | Spec | Impl | Tests |
|----------|------|------|-------|
| `POST /auth/login` | ✅ | ❌ | ❌ |
| `POST /auth/logout` | ✅ | ❌ | ❌ |
| `POST /auth/refresh` | ✅ | ❌ | ❌ |
| `GET /auth/session` | ✅ | ❌ | ❌ |
| `GET /users` | ✅ | ❌ | ❌ |
| `POST /users` | ✅ | ❌ | ❌ |
| `GET /users/{id}` | ✅ | ❌ | ❌ |

### FASE 2: Registry & Discovery (PRIORIDAD 2)
**Por qué:** Core del sistema - registro de herramientas

| Endpoint | Spec | Impl | Tests |
|----------|------|------|-------|
| `GET /registry/tools` | ✅ | ❌ | ❌ |
| `POST /registry/tools` | ✅ | ❌ | ❌ |
| `GET /registry/tools/{id}` | ✅ | ❌ | ❌ |
| `PUT /registry/tools/{id}` | ✅ | ❌ | ❌ |
| `DELETE /registry/tools/{id}` | ✅ | ❌ | ❌ |
| `POST /discovery/scan` | ✅ | ❌ | ❌ |
| `GET /discovery/jobs/{id}` | ✅ | ❌ | ❌ |

### FASE 3: Scans & Cleanup (PRIORIDAD 3)
**Por qué:** Funcionalidad principal del sistema

| Endpoint | Spec | Impl | Tests |
|----------|------|------|-------|
| `POST /scans/full` | ✅ | ❌ | ❌ |
| `POST /scans/tools` | ✅ | ❌ | ❌ |
| `GET /scans/tools/usage` | ✅ | ❌ | ❌ |
| `POST /cleanup/plan` | ✅ | ❌ | ❌ |
| `POST /cleanup/execute` | ✅ | ❌ | ❌ |
| `GET /cleanup/jobs/{id}` | ✅ | ❌ | ❌ |

### FASE 4: Docker & Schedules (PRIORIDAD 4)
**Por qué:** Features avanzadas

| Endpoint | Spec | Impl | Tests |
|----------|------|------|-------|
| `GET /docker/images` | ✅ | ❌ | ❌ |
| `POST /docker/prune` | ✅ | ❌ | ❌ |
| `GET /schedules` | ✅ | ❌ | ❌ |
| `POST /schedules` | ✅ | ❌ | ❌ |

---

## 📅 Sprints Planificados

### Sprint 1 (Actual): Auth + Users
```
🔴 RED:   Escribir tests para /auth/*
🟢 GREEN: Implementar endpoints auth
🔵 REFACTOR: Limpiar código
```

### Sprint 2: Registry + Discovery
```
🔴 RED:   Tests para /registry/* y /discovery/*
🟢 GREEN: Implementar CRUD de tools
🔵 REFACTOR: Optimizar queries
```

### Sprint 3: Scans + Cleanup
```
🔴 RED:   Tests para /scans/* y /cleanup/*
🟢 GREEN: Implementar lógica de limpieza
🔵 REFACTOR: Mejorar rendimiento
```

---

## 🚨 Deuda Técnica (RESUELTA)

| ID | Problema | Estado |
|----|----------|--------|
| DEBT-001 | Mock data en producción | ✅ RESUELTO |
| DEBT-002 | println! en producción | ✅ RESUELTO |
| DEBT-003 | Sin estructura tracking | ✅ RESUELTO |

---

## 🔗 Referencias

- **CONTRATO API:** [docs/specs/openapi.yaml](../docs/specs/openapi.yaml)
- **HANDOFF:** [CONTEXT_HANDOFF.md](./CONTEXT_HANDOFF.md)
- **AUDITORÍA:** [AUDIT_REPORT.md](../../AUDIT_REPORT.md)
