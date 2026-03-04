# DiskCortex 🧠

**Intelligent Disk Cleanup Automation with Multi-Platform GUI**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Coverage Status](https://img.shields.io/badge/coverage-0%25-red)]()

---

## 🎯 Overview

DiskCortex is a professional-grade disk cleanup automation tool with intelligent tool discovery, safe deletion protocols, and cross-platform GUI support.

### Key Features

- 🔍 **Auto-Discovery**: Automatically detects programming tools (Goose, VSCode, Claude, etc.)
- 🛡️ **Safe Cleanup**: Never deletes without explicit user confirmation
- 🐳 **Docker Integration**: Smart Docker cleanup with user approval
- 📊 **Real-time Visualization**: GUI with disk usage analytics
- 🔄 **Auto-Scheduling**: Integrates with launchd/systemd/cron
- 📦 **Tool Registry**: SQLite database of all known tools
- 🌍 **Multi-Platform**: macOS, Windows, Linux support

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│           DiskCortex GUI                │
│  (Electron/Tauri - TBD by OpenSpec)    │
└───────────────┬─────────────────────────┘
                │
    ┌───────────┼───────────┐
    │           │           │
┌───▼───┐   ┌───▼───┐   ┌───▼───┐
│Scanner│   │ Core  │   │Scheduler│
│ Engine │   │ Logic │   │  Agent  │
└───┬───┘   └───┬───┘   └───┬───┘
    │           │           │
    └───────────┼───────────┘
                │
        ┌───────▼────────┐
        │  Tool Registry │
        │    (SQLite)    │
        └────────────────┘
```

---

## 🚀 Development Status

**Current Phase**: Planning & Specification

**Methodology**: Scrum + TDD + OpenSpec

**Sprint Duration**: 1 week

**Target**: 6 sprints to MVP

---

## 📚 Documentation

- [OpenAPI Specification](./docs/specs/openapi.yaml)
- [Architecture Decision Records](./docs/architecture/)
- [API Documentation](./docs/api/)

---

## 🤝 Team

See [Team Structure](./plan/team.yaml) *(internal - not in repo)*

---

## 📋 Backlog

See [Product Backlog](./plan/backlog.yaml) *(internal - not in repo)*

---

## 🧪 Testing

```bash
# Run unit tests
npm test

# Run integration tests
npm run test:integration

# Run E2E tests
npm run test:e2e

# Generate coverage
npm run coverage
```

---

## 📦 Installation

*Coming in Sprint 6*

---

## 📄 License

MIT © 2026 Ruben-Alvarez-Dev

---

## 🙏 Acknowledgments

Built with ❤️ using modern development practices and AI-assisted planning.

