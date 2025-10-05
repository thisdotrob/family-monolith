# 009_1 — Server: Refactor `server/src/graphql/auth.rs` into smaller modules

Spec refs: §§5,6,8,10,13

## Summary
Break down the oversized `server/src/graphql/auth.rs` (currently >2000 LOC) into cohesive, smaller modules to improve maintainability and testability. No behavior changes.

## Scope
- Extract responsibilities into clearly named files under `server/src/graphql/` (or submodules):
  - `auth_unauth_mutations.rs` (login/refresh)
  - `auth_project_mutations.rs` (project create/rename/archive/unarchive/members)
  - `auth_tag_mutations.rs` (tags CRUD/merge semantics)
  - `auth_task_mutations.rs` (create/update/complete/abandon/restore + helper read-after-write)
  - `auth_saved_views_mutations.rs` (saved views CRUD + default setter)
  - `auth_recurring_series_mutations.rs` (series create; future: update)
  - Keep/adjust a `mod.rs` to re-export the combined `AuthenticatedMutation` and `UnauthenticatedMutation` used by `graphql::CombinedMutation`.
- Preserve all public GraphQL shapes and schema; strictly a mechanical refactor.
- Ensure the new module boundaries compile and pass existing tests.

## Acceptance Criteria
- `auth.rs` is reduced to a thin module that composes submodules; no large impl blocks remain.
- All mutations compile and schema remains unchanged (except reordering).
- Existing tests compile and pass.

## Dependencies
- Depends on: 009 — Server: GraphQL Task Mutations with Concurrency
- Unblocks: 009_2 .. 009_5 (follow-up fixes)

## Implementation Steps
1) Create submodule files and move the corresponding impl blocks and helpers.
2) Adjust `use` paths and re-exports so `graphql::AuthenticatedMutation` and `UnauthenticatedMutation` remain the external types used in `CombinedMutation`.
3) Run `cargo fmt` and `cargo test`.

## Tests
- No new tests required. Ensure existing unit/integration tests compile and pass after refactor.
