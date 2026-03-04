# DiskCortex Development Workflow

## Sprint Cadence

**Duration:** 1 week  
**Ceremonies:**
- Monday: Sprint Planning
- Daily: Standup (async via GitHub)
- Friday: Sprint Review + Retrospective

## Commit Schedule

### Organic GitHub Activity Simulation

To maintain realistic development patterns:

**Daily Commits:** 1-5 commits
**Pattern:**
- Morning (9-11h): 1-2 commits (planning, docs)
- Afternoon (14-17h): 2-3 commits (implementation)
- Evening (19-21h): 0-1 commits (fixes, refactors)

**Commit Distribution:**
- 40% Feature commits
- 25% Test commits
- 20% Refactor commits
- 15% Documentation commits

### Weekly Schedule

**Week 1-2:** Foundation (Sprint 1-2)
- Project setup
- Core architecture
- Basic scanning

**Week 3-4:** Features (Sprint 3-4)
- GUI development
- Tool registry
- Cleanup logic

**Week 5-6:** Polish (Sprint 5-6)
- Testing
- Documentation
- Release prep

## GitHub Push Strategy

### Protected Branches
- `main` - Requires PR + 1 approval
- `develop` - Requires PR

### Feature Workflow
```
develop → feature/XXX → develop (PR) → main (release)
```

### Commit Message Format

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

**Note:** This workflow ensures professional, sustainable development pace.

