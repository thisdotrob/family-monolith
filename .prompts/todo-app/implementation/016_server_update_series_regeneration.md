# 016 — Server: UpdateSeries with Regeneration of Future Occurrences

Spec refs: §§6,8

## Summary

Implement `updateRecurringSeries` mutation to update template and regenerate not-yet-completed/abandoned occurrences from now onward.

## Scope

Module placement and structure


- Put any shared GraphQL types/inputs in `server/src/graphql/types/`.
- Ensure `server/src/graphql/mod.rs` merges takenlijst mutations via `MergedObject`.
- Tests: add unit tests under `server/src/graphql/takenlijst/tests/` and any integration tests under `server/src/graphql/tests_*.rs`.
- Standardized error codes: INVALID_CREDENTIALS, TOKEN_EXPIRED, VALIDATION_FAILED, PERMISSION_DENIED, NOT_FOUND, CONFLICT_STALE_WRITE, INTERNAL_ERROR.
- Update series fields (title/desc/assignee/tags/rrule/dtstart/deadlineOffset)
- Propagation: core content to all todo occurrences; schedule/deadline recomputed for future incomplete occurrences

## Acceptance Criteria

- Past and completed/abandoned occurrences unchanged
- Todo occurrences updated correctly

## Dependencies

- 015

## Implementation Steps
1) Identify future occurrences to regenerate based on now
2) Apply template updates; recompute schedule/deadline
3) Transactional update
4) Tests for propagation and regeneration

Note: When you complete this ticket, update todo-app-implementation-sequencing-plan.md to check off .rovodev/todo-app-016_server_update_series_regeneration.md in the appropriate wave.
