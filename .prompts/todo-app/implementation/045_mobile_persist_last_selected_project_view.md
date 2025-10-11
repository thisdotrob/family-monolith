# 045 — Mobile: Persist Last-Selected Project and View

Spec refs: §4, §16

## Summary
Remember last-selected project and saved view in storage; restore them on app start.

## Scope
- Keys defined in shared; use in Projects and Saved Views tabs

## Acceptance Criteria
- On launch, app opens to last selected project Tasks screen if available; otherwise to Projects

## Dependencies
- 052, 033, 040

## Implementation Steps
1) Save on selection; load on startup
2) Add basic loading state while restoring


Note: When you complete this ticket, update todo-app-implementation-sequencing-plan.md to check off .rovodev/todo-app-045_mobile_persist_last_selected_project_view.md in the appropriate wave.
