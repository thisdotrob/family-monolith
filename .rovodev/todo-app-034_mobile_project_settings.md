# 034 — Mobile: Project Settings (Rename/Archive/Unarchive)

Spec refs: §§5,8,15

## Summary
Provide a settings screen accessible from Tasks header to rename or archive/unarchive the current project.

## Scope
- Mutations: renameProject, archiveProject, unarchiveProject
- Concurrency handling via `lastKnownUpdatedAt`

## Acceptance Criteria
- Only owner sees actions; others see read-only details
- After mutation, immediate refetch of projects and current project

## Dependencies
- 003, 033

## Implementation Steps
1) Show owner-only actions conditionally (query me + project owner)
2) Wire mutations; handle `CONFLICT_STALE_WRITE` with retry UX placeholder (046 for polish)
