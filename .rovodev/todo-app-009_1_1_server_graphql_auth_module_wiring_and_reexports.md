# 009_1_1 — Server: GraphQL auth module wiring and re-exports

Spec refs: §§5,6,8,10,13

## Summary
Establish the new module structure for auth GraphQL mutations by introducing a `mod.rs` (or equivalent module composition) that re-exports `AuthenticatedMutation` and `UnauthenticatedMutation`. This is the scaffolding step to enable subsequent mechanical splits of responsibilities.

## Scope
- Create `server/src/graphql/auth/` submodule folder (or equivalent module namespace) and add a `mod.rs` that:
  - Declares submodules for responsibilities (unauth, projects, tags, tasks, saved_views, recurring_series).
  - Re-exports `AuthenticatedMutation` and `UnauthenticatedMutation` types so `graphql::CombinedMutation` remains unchanged externally.
- Reduce the original `server/src/graphql/auth.rs` to a thin delegator or move it entirely under the new folder as needed.
- No behavioral changes; strictly structural.

## Acceptance Criteria
- A new `auth` module layout exists with a `mod.rs` that composes submodules and re-exports `AuthenticatedMutation` and `UnauthenticatedMutation`.
- Code compiles and existing tests pass.
- No changes to public GraphQL schema or mutation shapes (excluding reordering).

## Dependencies
- Depends on: 009 — Server: GraphQL Task Mutations with Concurrency
- Unblocks: 009_2 .. 009_5 (follow-up fixes)

## Implementation Steps
1) Create the `auth` folder/module and `mod.rs` wiring with placeholders for each submodule.
2) Keep type names and visibility consistent so external references remain valid.
3) Run `cargo fmt` and `cargo test`.

## Tests
- No new tests required. Ensure existing unit/integration tests compile and pass after scaffolding.
