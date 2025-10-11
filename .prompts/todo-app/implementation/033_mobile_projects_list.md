# 033 — Mobile: Projects List (Live) with Archived Toggle

Spec refs: §§4–5,8

## Summary
Build Projects tab to list projects (with includeArchived toggle) and select a project.

## Scope
- Query `projects(includeArchived)` using shared docs
- List items show name and archived badge if applicable
- Selecting a project sets current project in storage

## Acceptance Criteria
- Lists only projects where user is owner/member
- Include archived toggle works

## Dependencies
- 050, 002, 031

## Implementation Steps
1) Fetch projects via Apollo
2) Render list with toggle
3) On select, persist to storage and navigate to Tasks tab

## Tests
- Manual happy path; consider basic component test


Note: When you complete this ticket, update todo-app-implementation-sequencing-plan.md to check off .rovodev/todo-app-033_mobile_projects_list.md in the appropriate wave.
