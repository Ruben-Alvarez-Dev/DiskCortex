# ADR-001: Technology Stack

## Status

Accepted

## Context

DiskCortex needs a technology stack that supports:
- Cross-platform desktop GUI (macOS, Linux, Windows)
- High-performance disk scanning
- Local HTTP API for GUI/CLI communication
- SQLite for tool registry and audit logs
- Safe, concurrent operations

## Decision

We will use:

### Backend
- **Rust** - Memory safety, performance, excellent async support
- **Axum** - Modern async web framework
- **SQLite (via sqlx)** - Embedded database, no external dependencies
- **Tokio** - Async runtime

### Frontend
- **Electron** - Cross-platform desktop with native OS integration
- **React + TypeScript** - Type-safe UI development
- **Vite** - Fast build tooling

### API
- **REST over HTTP** (localhost:7331)
- **OpenAPI 3.1** specification
- **JWT + Session tokens** for authentication

## Consequences

### Positive
- Single backend binary, easy distribution
- SQLite means no database server setup
- Rust provides memory safety and performance
- Electron allows code reuse across platforms

### Negative
- Larger bundle size due to Electron
- Rust learning curve for contributors
- Two build systems (Cargo + npm)

## Alternatives Considered

1. **Tauri instead of Electron** - Smaller bundle, but less mature ecosystem
2. **Go instead of Rust** - Simpler, but less memory-safe guarantees
3. **PostgreSQL instead of SQLite** - Overkill for local-only data
