# 009_2 — Server: Expose Task Mutations (GraphQL schema registration)

Spec refs: §§5,6,8,10,13

## Summary
Ensure the Task mutation resolvers (createTask, updateTask, completeTask, abandonTask, restoreTask) are exposed in the GraphQL schema (missing `#[Object]` on the impl that defines them).

## Scope
- Add `#[Object]` to the `impl AuthenticatedMutation` block that contains the task mutation functions or move them into an existing `#[Object]`-annotated impl.
- Verify the schema contains the 5 mutations with the correct argument names and types per spec.

## Acceptance Criteria
- Schema exposes: `createTask`, `updateTask`, `completeTask`, `abandonTask`, `restoreTask` under `AuthenticatedMutation`.
- Introspection (in dev) or SDL print includes the 5 mutations with correct inputs.
- All existing tests compile and pass.

## Dependencies
- Depends on: 009_1 — refactor modules

## Implementation Steps
1) After the refactor, ensure the `auth_task_mutations.rs` impl is annotated with `#[Object]` and linked into `CombinedMutation`.
2) Build and validate the schema compiles.

## Tests
- Add a small schema-level test or reuse integration tests to assert the mutations are callable (e.g., a smoke test calling `createTask`).
