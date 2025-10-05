# 009_1_6 — Server: Split saved views mutations

Spec refs: §§5,6,8,10,13

## Summary
Move saved views mutations (CRUD and default setter) into a dedicated module. No behavior changes.

## Scope
- Extract saved views mutations and helpers into `server/src/graphql/auth/saved_views.rs` (or similar name).
- Keep public GraphQL shapes and return types identical.
- Continue to expose via `AuthenticatedMutation` re-exported in `auth::mod` and used by `CombinedMutation`.

## Acceptance Criteria
- Saved views mutations live under a dedicated module.
- `AuthenticatedMutation` exposure remains stable for `CombinedMutation`.
- Code compiles and all tests pass.

## Dependencies
- Depends on: 009, 009_1_1 — Module wiring and re-exports

## Implementation Steps
1) Move impl blocks and helpers for saved views mutations.
2) Fix `use` paths and visibility.
3) Run `cargo fmt` and `cargo test`.

## Tests
- No new tests required. Ensure existing unit/integration tests compile and pass.
