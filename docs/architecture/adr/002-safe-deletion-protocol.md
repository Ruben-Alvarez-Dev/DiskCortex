# ADR-002: Safe Deletion Protocol

## Status

Accepted

## Context

DiskCortex deletes user files. This is inherently dangerous. We need a protocol that:
- Prevents accidental data loss
- Provides clear audit trail
- Requires explicit user consent
- Allows cancellation before execution

## Decision

We implement a **Plan → Confirm → Execute** workflow:

### 1. Plan Creation
```
POST /cleanup/plans
→ Returns: planId, confirmationToken
```
- System analyzes disk usage
- Generates list of items to delete
- Creates a unique confirmation token
- Plan is in `draft` state

### 2. User Review
```
GET /cleanup/plans/{planId}
→ Returns: Full plan with all items
```
- User reviews every item
- Each item shows: path, size, risk level, tool
- Risk levels: `safe`, `low`, `medium`, `high`

### 3. Explicit Confirmation
```
POST /cleanup/plans/{planId}/confirm
Body: { confirmationToken: "..." }
→ Returns: executionId
```
- Token must match exactly
- No execution without valid token
- Plan state changes to `executing`

### 4. Execution
```
GET /cleanup/executions/{executionId}
→ Returns: Progress, results
```
- Real-time progress updates
- Per-item success/failure status
- Bytes reclaimed

### 5. Audit
All operations logged to `audit_logs` table with:
- Timestamp, user, action
- Resource affected
- IP address

## Consequences

### Positive
- Zero accidental deletions
- Full audit trail for compliance
- Clear user accountability
- Reversible until confirmation

### Negative
- Extra API calls required
- User must actively confirm

## Risk Categories

| Category | Examples | Default Action |
|----------|----------|----------------|
| `safe` | Cache files, temp files | Auto-include |
| `low` | Build artifacts | Auto-include |
| `medium` | Logs, old downloads | Ask per item |
| `high` | Docker volumes, databases | Never auto-include |

## Docker Special Handling

Docker items require additional consent:
- Images: Check if used by any container
- Volumes: Verify no active mounts
- Containers: Confirm stopped state
