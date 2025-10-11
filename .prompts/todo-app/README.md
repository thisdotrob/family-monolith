# .rovodev — Planning Tickets for Family Takenlijst

This folder contains the blueprint and implementation tickets for the Family Takenlijst mobile app. Each ticket is an incremental, independently testable unit of work with explicit dependencies, allowing parallel development across backend, shared code, and mobile.

How to use:
- Start with the blueprint: `todo-app-blueprint.md`.
- Pick tickets whose dependencies are satisfied. Prefer backend APIs first, then shared docs, then mobile surfaces.
- Ensure no ticket leaves orphaned code: placeholders are integrated into navigation and guarded behind feature flags until server support lands.
