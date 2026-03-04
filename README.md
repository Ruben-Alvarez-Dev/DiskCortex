# DiskCortex

**Enterprise-grade disk cleanup automation with intelligent tool discovery and safe deletion protocols.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## What is DiskCortex?

DiskCortex is a desktop application that automatically discovers developer tools on your system, analyzes their disk usage, and safely cleans up caches, logs, and build artifacts—with explicit user confirmation before any deletion.

### The Problem

Development tools accumulate massive amounts of data:
- AI tools store models, caches, and logs (Goose: 47GB, Claude: 8GB+)
- IDEs keep build artifacts and indexing data
- Package managers cache thousands of dependencies
- Docker images and volumes grow unchecked

### The Solution

DiskCortex provides:
- **Auto-discovery** of 50+ development tools (AI, IDEs, runtimes, containers)
- **Safe cleanup** with explicit confirmation—nothing is deleted without your approval
- **Docker integration** with smart detection of unused images/containers/volumes
- **Scheduling** for automated maintenance (launchd, systemd, Task Scheduler)
- **Audit logging** for enterprise compliance
- **Multi-platform** support (macOS, Linux, Windows)

---

## Features

| Feature | Description |
|---------|-------------|
| 🔍 **Tool Discovery** | Automatically detects installed development tools |
| 📊 **Disk Analysis** | Real-time visualization of disk usage per tool |
| 🛡️ **Safe Deletion** | Plans require explicit confirmation before execution |
| 🐳 **Docker Support** | Preview and clean unused images, containers, volumes |
| 📅 **Scheduling** | Set up automated cleanups with cron-style scheduling |
| 📝 **Audit Logs** | Full operation history for compliance |
| 🔐 **User Management** | Role-based access (admin, operator, viewer) |
| ✅ **Compliance** | GDPR, SOC2, HIPAA, ISO27001, PCI-DSS reporting |

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    DiskCortex GUI                       │
│              (Electron + React + TypeScript)            │
└─────────────────────────┬───────────────────────────────┘
                          │ HTTP (localhost:7331)
┌─────────────────────────▼───────────────────────────────┐
│                   DiskCortex Daemon                     │
│                     (Rust Backend)                      │
│  ┌─────────┐ ┌──────────┐ ┌─────────┐ ┌─────────────┐  │
│  │ Scanner │ │ Registry │ │ Cleanup │ │ Scheduler   │  │
│  │ Engine  │ │  (SQLite)│ │ Engine  │ │ (launchd)   │  │
│  └─────────┘ └──────────┘ └─────────┘ └─────────────┘  │
└─────────────────────────────────────────────────────────┘
```

---

## Installation

### Prerequisites

- macOS 10.15+, Windows 10+, or Linux (systemd)
- 50MB free disk space

### Download

*Releases coming soon*

### Build from Source

```bash
git clone https://github.com/Ruben-Alvarez-Dev/DiskCortex.git
cd DiskCortex

# Build daemon
cargo build --release

# Build GUI
npm install
npm run build
```

---

## Usage

### Quick Start

1. Launch DiskCortex
2. Click **Scan** to discover tools and analyze disk usage
3. Review the cleanup plan
4. **Confirm** items to delete (nothing is removed without your approval)
5. Execute cleanup

### CLI Usage

```bash
# Scan for tools
diskcortex scan

# Show disk usage
diskcortex usage

# Create cleanup plan
diskcortex plan --tool goose --category cache

# Execute plan (requires confirmation)
diskcortex execute --plan-id <uuid> --confirm
```

---

## API

DiskCortex exposes a local HTTP API at `http://127.0.0.1:7331`

Full API documentation: [OpenAPI Specification](./docs/specs/openapi.yaml)

### Key Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/registry/tools` | List registered tools |
| POST | `/discovery/scan` | Run tool discovery |
| POST | `/scans/full` | Full disk scan |
| POST | `/cleanup/plans` | Create cleanup plan |
| POST | `/cleanup/plans/{id}/confirm` | **Confirm and execute** |
| GET | `/docker/status` | Docker daemon status |
| GET | `/compliance/report` | Generate compliance report |

---

## Supported Tools

### AI Tools
- Goose, Claude Desktop, ChatGPT Desktop, OpenCode
- VSCode Copilot, Cursor, Windsurf
- Qwen, Gemini, Codex

### IDEs
- VSCode, VSCode Insiders
- JetBrains IDEs (IntelliJ, PyCharm, GoLand, etc.)
- Sublime Text, Vim/Neovim

### Runtimes & Package Managers
- Node.js (npm, yarn, pnpm)
- Python (pip, uv, poetry)
- Go, Rust, Java, .NET

### Containers
- Docker (images, containers, volumes, build cache)

---

## Development

### Tech Stack

- **Backend**: Rust, SQLite, tokio, axum
- **Frontend**: Electron, React, TypeScript, Vite
- **API**: OpenAPI 3.1, REST
- **Testing**: Jest, Playwright, cargo test

### Project Structure

```
DiskCortex/
├── src/
│   ├── core/           # Core cleanup logic
│   ├── daemon/         # Rust HTTP server
│   ├── scanner/        # Tool detection
│   └── gui/            # Electron + React
├── docs/
│   ├── specs/          # OpenAPI specification
│   └── architecture/   # ADRs
├── tests/
│   ├── unit/
│   ├── integration/
│   └── e2e/
└── plan/               # Internal planning (gitignored)
```

### Contributing

See [CONTRIBUTING.md](./.github/CONTRIBUTING.md)

---

## License

MIT © 2026 Ruben-Alvarez-Dev

---

## Acknowledgments

Built with modern development practices and a focus on safety—because losing data is not an option.
