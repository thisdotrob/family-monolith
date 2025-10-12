# 015 — Server: Generate Series Occurrences and Link to Tasks

Spec refs: §§6,8

## Summary

On series creation, generate 5 future task occurrences and link them to the series.

## Scope

Module placement and structure


- Put any shared GraphQL types/inputs in `server/src/graphql/types/`.
- Ensure `server/src/graphql/mod.rs` merges takenlijst objects via `MergedObject`.
- Tests: add unit tests under `server/src/graphql/takenlijst/tests/` and any integration tests under `server/src/graphql/tests_*.rs`.
- Standardized error codes: INVALID_CREDENTIALS, TOKEN_EXPIRED, VALIDATION_FAILED, PERMISSION_DENIED, NOT_FOUND, CONFLICT_STALE_WRITE, INTERNAL_ERROR.\n- Generate tasks with seriesId referencing the series\n- Apply template defaults: title, description, tags, assignee\n- Set scheduled and deadline using offset\n\n## Acceptance Criteria\n- Exactly 5 future todo occurrences created\n- No past occurrence created\n\n## Dependencies\n- 014, 007, 009\n\n## Implementation Steps\n1) Generation utility that returns next N occurrences after now\n2) Persist tasks and tag associations in a transaction\n3) Return created series\n4) Tests for generation count and scheduling\n

Note: When you complete this ticket, update todo-app-implementation-sequencing-plan.md to check off .rovodev/todo-app-015_server_generation_series_occurrences.md in the appropriate wave.
