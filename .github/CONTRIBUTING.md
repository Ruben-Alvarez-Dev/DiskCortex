# Contributing to DiskCortex

## Development Workflow

### Branch Strategy
- `main` - Production-ready code
- `develop` - Integration branch
- `feature/*` - Feature branches
- `bugfix/*` - Bug fixes
- `release/*` - Release preparation

### Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

#### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

#### Scopes
- `core`: Core cleanup logic
- `gui`: GUI components
- `scanner`: Tool scanning
- `scheduler`: Scheduling system
- `registry`: Tool registry
- `docker`: Docker integration

#### Examples
```
feat(scanner): add VSCode extension detector

- Scan .vscode/extensions directory
- Parse extension manifests
- Add to tool registry

Closes #42
```

### Pull Request Process

1. Create feature branch from `develop`
2. Write tests first (TDD)
3. Implement feature
4. Ensure all tests pass
5. Update documentation
6. Create PR with description
7. Wait for code review
8. Squash and merge

### Code Style

- **TypeScript/JavaScript**: ESLint + Prettier
- **Python**: Black + isort
- **Rust**: rustfmt + clippy
- **General**: SOLID, DRY, Clean Code

### Testing

- Unit tests: Jest/Vitest
- Integration tests: Supertest
- E2E tests: Playwright
- Coverage: >80%

---

**Questions?** Open an issue or contact the team.

