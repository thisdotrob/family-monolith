# 035 — Mobile: Project Members List and Add by Username

Spec refs: §§5,8

## Summary
Show list of members and allow owner to add a member by exact username.

## Scope
- Query projectMembers(projectId)
- Mutation addProjectMemberByUsername(projectId, username)

## Acceptance Criteria
- Adding existing member shows error; non-existent username shows error

## Dependencies
- 003, 033

## Implementation Steps
1) Display members including owner
2) Add form to add member; show errors from server


Note: When you complete this ticket, update todo-app-implementation-sequencing-plan.md to check off .rovodev/todo-app-035_mobile_project_members.md in the appropriate wave.
