# 009_4 — Server: Validate Task Date Fields in Mutations

Spec refs: §§6,8,10

## Summary
Validate `scheduledDate` and `deadlineDate` format (YYYY-MM-DD) when provided to `createTask` and `updateTask`. Reject invalid dates with `VALIDATION_FAILED`.

## Scope
- Use `chrono::NaiveDate::parse_from_str(..., "%Y-%m-%d")` to validate when Option<String> is Some.
- Keep time-minute bounds validation as-is.

## Acceptance Criteria
- Invalid date strings are rejected with `VALIDATION_FAILED` and clear error messages.
- Valid inputs are accepted; no change to persisted data format.

## Dependencies
- Depends on: 009_1 — refactor modules

## Implementation Steps
1) Add date validation branches in create/update paths.
2) Ensure error codes and messages match the existing validation pattern.

## Tests
- Unit tests: a few cases for invalid dates (bad month/day), and valid dates.
