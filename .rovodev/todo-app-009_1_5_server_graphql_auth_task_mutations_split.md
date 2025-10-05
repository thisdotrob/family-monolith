# 009_1_5 — Server: Split task-related mutations

Spec refs: §§5,6,8,10,13

## Summary
Move task-related mutations (create, update, complete, abandon, restore, and any read-after-write helpers) into a dedicated module. No behavior changes.

## Scope
- Extract task mutations and helpers into `server/src/graphql/auth/tasks.rs` (or similar name).
- Keep public GraphQL shapes and return types identical.
- Continue to expose via `AuthenticatedMutation` re-exported in `auth::mod` and used by `CombinedMutation`.

## Acceptance Criteria
- Task mutations live under a dedicated module.
- `AuthenticatedMutation` exposure remains stable for `CombinedMutation`.
- Code compiles and all tests pass.

## Dependencies
- Depends on: 009, 009_1_1 — Module wiring and re-exports

## Implementation Steps
1) Move impl blocks and helpers for task mutations.
2) Fix `use` paths and visibility.
3) Run `cargo fmt` and `cargo test`.

## Tests
- No new tests required. Ensure existing unit/integration tests compile and pass.
