# 042 — Mobile: Recurrence Fields in Task Form (Raw RRULE)

Spec refs: §6, §15

## Summary
Add a collapsible section in the task form for recurrence input (raw RRULE and DTSTART), hidden behind a toggle until server feature available.

## Scope
- UI fields: RRULE string, DTSTART date/time and deadline offset
- For MVP, display but disable submit until series support lands; or wire to create series when 014/015 are ready

## Acceptance Criteria
- UI does not break task creation/update for non-recurring tasks

## Dependencies
- 038; fully enabled by 014, 015

## Implementation Steps
1) Add fields and local validation hints
2) Gate by feature flag or check for server field availability
