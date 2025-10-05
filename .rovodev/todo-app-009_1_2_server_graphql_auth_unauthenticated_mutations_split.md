# 009_1_2 — Server: Split unauthenticated auth mutations (login/refresh)

Spec refs: §§5,6,8,10,13

## Summary
Move unauthenticated auth mutations (`login`, `refreshToken`) into a dedicated module (e.g., `auth/unauth.rs`). No behavior changes.

## Scope
- Extract `login` and `refreshToken` mutations and any local helpers into `server/src/graphql/auth/unauth.rs` (name may vary but should be clear).
- Keep public GraphQL shapes and return types identical.
- Ensure `UnauthenticatedMutation` continues to be exposed via `auth::mod` and used by `graphql::CombinedMutation`.

## Acceptance Criteria
- `login` and `refreshToken` live under a dedicated unauth module.
- `UnauthenticatedMutation` is re-exported from the `auth` module and used by `CombinedMutation` unchanged.
- Code compiles and all tests pass.

## Dependencies
- Depends on: 009, 009_1_1 — Module wiring and re-exports

## Implementation Steps
1) Move impl blocks and helpers for unauthenticated mutations into the new unauth module.
2) Adjust imports and visibility as needed.
3) Run `cargo fmt` and `cargo test`.

## Tests
- No new tests required. Ensure existing unit/integration tests compile and pass.
