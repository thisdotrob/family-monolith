# 009_1_4 — Server: Split tag-related mutations

Spec refs: §§5,6,8,10,13

## Summary
Move tag-related mutations (CRUD, merge semantics) into a dedicated module. No behavior changes.

## Scope
- Extract tag mutations and helpers into `server/src/graphql/auth/tags.rs` (or similar name).
- Keep public GraphQL shapes and return types identical.
- Continue to expose via `AuthenticatedMutation` re-exported in `auth::mod` and used by `CombinedMutation`.

## Acceptance Criteria
- Tag mutations live under a dedicated module.
- `AuthenticatedMutation` exposure remains stable for `CombinedMutation`.
- Code compiles and all tests pass.

## Dependencies
- Depends on: 009, 009_1_1 — Module wiring and re-exports

## Implementation Steps
1) Move impl blocks and helpers for tag mutations.
2) Fix `use` paths and visibility.
3) Run `cargo fmt` and `cargo test`.

## Tests
- No new tests required. Ensure existing unit/integration tests compile and pass.
