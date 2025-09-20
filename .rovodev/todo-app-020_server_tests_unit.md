# 020 — Server: Unit Tests (Validation, Permissions, Concurrency, Recurrence)

Spec refs: §17

## Summary
Add unit tests to cover validation, permissions, concurrency, and recurrence behaviors.

## Scope
- Validation: tag normalization/uniqueness; project and saved view name rules; RRULE validity and first occurrence constraints; deadline offset bounds
- Concurrency: stale-write detection using updatedAt
- Permissions: owner vs member actions; archived project read-only enforcement
- Recurrence: top-up policy on completion/abandon; regeneration behavior on update

## Acceptance Criteria
- Tests run in CI and are deterministic
- Cover both success and failure paths

## Dependencies
- 006, 003, 009, 016, 017

## Implementation Steps
1) Add unit tests under `server/src/...` or `tests/` as appropriate
2) Mock or seed minimal data for each scenario
3) Assert error codes via GraphQL extensions where relevant
