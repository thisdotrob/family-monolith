# 040 — Mobile: Saved Views UI and Default Setter

Spec refs: §§5,8,15

## Summary
Implement Saved Views tab: list, apply, create/rename/delete, and set/clear project default.

## Scope
- Queries: savedViews(projectId), projectDefaultSavedView(projectId)
- Mutations: createSavedView, updateSavedView, deleteSavedView, setProjectDefaultSavedView

## Acceptance Criteria
- Applying a view updates task filters state
- Default setter persists and is reflected after refresh

## Dependencies
- 012, 033, 037

## Implementation Steps
1) Build saved views list and editor dialogs
2) Wire default setter control


Note: When you complete this ticket, update todo-app-implementation-sequencing-plan.md to check off .rovodev/todo-app-040_mobile_saved_views_ui.md in the appropriate wave.
