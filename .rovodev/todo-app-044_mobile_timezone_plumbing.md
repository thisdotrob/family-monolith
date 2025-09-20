# 044 — Mobile: Timezone Plumbing

Spec refs: §§6,8,11,16

## Summary
Ensure device timezone is passed into tasks and history queries/mutations that need validation/derived fields.

## Scope
- Use shared timezone helper
- Audit calls in Tasks and History screens

## Acceptance Criteria
- tz variable is present and valid for all relevant requests

## Dependencies
- 051; used by 037, 041

## Implementation Steps
1) Inject tz from `useTimezone()` into Apollo hooks
2) Add regression tests where feasible
