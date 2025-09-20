# 038 — Mobile: Task Create/Edit Form

Spec refs: §§5–6,8,10,15

## Summary
Full-screen form to create and edit todo tasks with validation.

## Scope
- Fields: title, description, assignee, tags, schedule/deadline
- Validation as per spec
- Use createTask/updateTask mutations

## Acceptance Criteria
- Validation errors shown inline
- Successful submit triggers immediate list refetch

## Dependencies
- 009, 006, 033

## Implementation Steps
1) Build form with input components
2) Populate members and tags dropdowns
3) Wire mutations and navigate back on success
