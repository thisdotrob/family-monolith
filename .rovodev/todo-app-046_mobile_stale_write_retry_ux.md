# 046 — Mobile: Stale-Write Retry UX and Error States

Spec refs: §13, §15

## Summary
Provide consistent UX for `CONFLICT_STALE_WRITE` errors: refetch and prompt user to retry or auto-retry; show offline and permission errors distinctly.

## Scope
- Intercept GraphQL errors; detect `extensions.code`
- For stale-write, refetch entity and offer retry
- For permission denied, show explanation

## Acceptance Criteria
- User can successfully resolve conflicts without data loss

## Dependencies
- 009, 004; used by 034, 038, 039, 040

## Implementation Steps
1) Add error handling helper/hook
2) Integrate in mutation flows
