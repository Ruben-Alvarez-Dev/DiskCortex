# DiskCortex Development Workflow

## Sprint Cadence

**Duration:** 1 week  
**Ceremonies:**
- Monday: Sprint Planning
- Daily: Standup (async)
- Friday: Sprint Review + Retrospective

## Branch Strategy

### Protected Branches
- `main` - Production-ready code
- `develop` - Integration branch

### Feature Workflow
```
develop → feature/XXX → develop (PR) → main (release)
```

## Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short description>

# Examples:
feat(scanner): add Claude Desktop detector
test(core): add unit tests for disk analyzer
docs(api): document cleanup API endpoints
refactor(gui): improve dashboard component
fix(docker): handle permission errors
```

## Automation

### Pre-commit Hooks
- Linting
- Type checking
- Test execution (affected files)

### CI/CD Pipeline
1. Run linters
2. Execute tests
3. Build artifacts
4. Deploy (on main)

---

**Questions?** Open an issue or contact the team.
