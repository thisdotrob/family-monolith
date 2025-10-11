# 052 — Shared: Offline Detection and Storage Keys

Spec refs: §9, §16

## Summary
Provide hooks/adapters for offline detection and define storage keys for last-selected project and saved view.

## Scope
- Hook `useOffline()` based on NetInfo or Navigator.onLine (platform-aware)
- Define storage key constants and helper functions
- Ensure integration points with AuthContext are respected

## Acceptance Criteria
- Offline hook works on mobile; returns stable boolean and emits changes
- Storage helpers read/write keys without leaking tokens

## Dependencies
- None; used by mobile tickets (037, 041, 045)

## Implementation Steps
1) Implement hook and keys under `shared/`
2) Minimal tests or manual usage sample


Note: When you complete this ticket, update todo-app-implementation-sequencing-plan.md to check off .rovodev/todo-app-052_shared_offline_and_storage.md in the appropriate wave.
