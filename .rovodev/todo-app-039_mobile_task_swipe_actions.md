# 039 — Mobile: Task Swipe Actions (Complete/Abandon/Restore)

Spec refs: §§5–6,8,15

## Summary
Add swipe gestures: right=Complete, left=Abandon on todo; Restore on abandoned in History.

## Scope
- Mutations: completeTask, abandonTask, restoreTask
- Immediate refetch after success

## Acceptance Criteria
- Disabled while offline
- Confirmations or undo toast (optional)

## Dependencies
- 009, 037

## Implementation Steps
1) Add swipeable list items
2) Wire mutations and refresh
