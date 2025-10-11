# 051 — Shared: Timezone Helper and Apollo Usage Pattern

Spec refs: §§6,8,11,16

## Summary
Provide helpers to obtain device timezone and pass it into relevant queries/mutations. Document Apollo usage patterns for unauthenticated vs authenticated operations.

## Scope
- Utility to resolve IANA timezone string from device (use `expo-localization` or JS Intl API as fallback)
- Helper to attach tz variable to queries that require it
- Document pattern for `context: { unauthenticated: true }` when invoking login/refresh if needed

## Acceptance Criteria
- Helper returns a valid IANA timezone string
- Sample usage integrated in a small example hook

## Dependencies
- None; used by 037/041/044

## Implementation Steps
1) Create `@shared/time` helper for tz
2) Add example `useTimezone()` hook for mobile
3) Update shared README or inline docs


Note: When you complete this ticket, update todo-app-implementation-sequencing-plan.md to check off .rovodev/todo-app-051_shared_timezone_and_apollo.md in the appropriate wave.
