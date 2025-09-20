# 043 — Mobile: Recurring Series Management Screen

Spec refs: §6, §15

## Summary
Add screen to view and edit recurring series for a project.

## Scope
- List series; edit title/desc/assignee/tags/rrule/dtstart/deadline offset
- Use create/update series mutations; reflect regeneration behavior

## Acceptance Criteria
- Editing propagates to todo occurrences as per spec

## Dependencies
- 016

## Implementation Steps
1) Build series list and edit screen
2) Wire mutations and refresh related tasks
