# 021 — Server: Integration Tests (Derived Fields, Pagination, History)

Spec refs: §17, §§11–12

## Summary
Add integration tests to validate end-to-end behavior for derived fields, ordering, pagination, and history query.

## Scope
- Tasks list: verify isOverdue/bucket with provided timezone, and default ordering
- Pagination: verify offset/limit behavior and totalCount
- History query: verify filters and day-grouping-friendly ordering

## Acceptance Criteria
- Tests run in CI and pass reliably
- Cover at least two timezones (e.g., Europe/Amsterdam, America/New_York)

## Dependencies
- 010, 018, 008

## Implementation Steps
1) Seed test data per scenario
2) Execute GraphQL queries with tz variations
3) Assert ordering and derived flags

## Out of Scope
- Unit-level validation (covered by 020)


Note: When you complete this ticket, update todo-app-implementation-sequencing-plan.md to check off .rovodev/todo-app-021_server_tests_integration.md in the appropriate wave.
