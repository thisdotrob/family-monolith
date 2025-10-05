# 009_5 — Server: Add Task Mutation Tests and Fix Integration Suite for async-graphql v7

Spec refs: §§5,6,8,10,13

## Summary
Add unit tests covering task mutation behavior (status transitions, concurrency conflicts, archived project rejection) and update the integration test harness to async-graphql v7 APIs so tests compile and run.

## Scope
- Unit tests for:
  - complete: only from todo; sets done, completedAt/By.
  - abandon: only from todo; sets abandoned, abandonedAt/By.
  - restore: only from abandoned; clears abandoned fields.
  - concurrency: update/complete/abandon/restore reject with `CONFLICT_STALE_WRITE` on stale `lastKnownUpdatedAt`.
  - archived: all task writes rejected.
- Integration tests fixes (async-graphql v7):
  - Request variables: use `Variables::from_json`.
  - Response data shape: `Response.data` is `Value` (remove `.expect`).
  - Seed test arrays: wrap string literals with `Some("...")` to satisfy `Option<&str>` types.

## Acceptance Criteria
- New unit tests compile and pass.
- Integration tests compile and pass locally.

## Dependencies
- Depends on: 009_1 — refactor modules
- May be done in parallel with 009_2–009_4 once the refactor merges.

## Implementation Steps
1) Add unit tests under `server/src/graphql/` (or tasks module) to cover behavior.
2) Modernize `tests_integration.rs` to async-graphql v7.
3) Run `cargo test` until green.

## Tests
- Covered within Scope; ensure meaningful assertions and error codes.
