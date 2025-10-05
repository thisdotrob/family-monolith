# 009_3 — Server: Enforce Archived Read-only on Status Mutations

Spec refs: §§5,8,14

## Summary
For archived projects, all task writes must be rejected. Add archived guards to `completeTask`, `abandonTask`, and `restoreTask` (already present in `createTask`/`updateTask`).

## Scope
- In each of the 3 resolvers, fetch `archived_at` for the task’s project and return a GraphQL error with `extensions.code = VALIDATION_FAILED` if archived.
- Keep existing permission and concurrency checks as-is.

## Acceptance Criteria
- Attempting to complete/abandon/restore a task in an archived project is rejected with `VALIDATION_FAILED`.
- Behavior for non-archived projects remains unchanged.

## Dependencies
- Depends on: 009_1 — refactor modules

## Implementation Steps
1) Add archived check (same SQL as used in create/update) early in each status mutation.
2) Build and test locally.

## Tests
- Unit tests: one per resolver asserting rejection in archived project context.
