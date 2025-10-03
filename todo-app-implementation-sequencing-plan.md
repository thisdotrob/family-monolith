# Todo App — Implementation Sequencing Plan (Parallelizable Waves)

This plan sequences all implementation tickets for the `todo-app` project into parallelizable “waves” based solely on their explicit dependencies. Each ticket is referenced by its filename and presented with a checkbox so you can mark completion and always see what can start next in parallel.

How to use this plan
- Start with Wave 1. All items in a wave can be worked on in parallel once their dependencies are complete.
- When a wave is fully checked, begin the next wave. If a single item in a wave finishes early and all of its dependents in the next wave are already unblocked, those dependents can also start.
- Keep this document updated by checking off tickets as they complete. The “Next Up” guidance beneath each wave clarifies when to move forward.

Legend
- [ ] Not started
- [x] Done

Source of dependencies: .rovodev/todo-app-blueprint.md (Ticket Map), plus notes within the individual ticket files.

Note on a few tickets with soft/rolling dependencies
- .rovodev/todo-app-020_server_tests_unit.md can be developed as features land; this plan schedules it after core backend features are ready to maximize coverage.
- .rovodev/todo-app-051_shared_timezone_and_apollo.md can start immediately, but finalization depends on Tasks list/derived fields. It appears early here and will be referenced by mobile pieces later.
- .rovodev/todo-app-042_mobile_recurrence_fields_in_form.md can start after the Task Form, with full enablement once recurrence backend (014,015) is done.


Wave 1 — Foundations (independent kickoff)
Backend
- [x] .rovodev/todo-app-001_server_migrations_projects_memberships.md
- [x] .rovodev/todo-app-005_server_migrations_tags.md
- [x] .rovodev/todo-app-019_server_placeholder.md (reserved / gap filler)
Shared
- [x] .rovodev/todo-app-051_shared_timezone_and_apollo.md (can start; finalize after 008,010)
- [x] .rovodev/todo-app-052_shared_offline_and_storage.md
Mobile
- [x] .rovodev/todo-app-030_mobile_app_module_skeleton.md

Next up when Wave 1 items complete:
- Projects/Tasks dependent migrations and queries/mutations (Wave 2 and 3)
- Mobile wiring (031) and early navigation (032)


Wave 2 — Early schema and mobile wiring
Backend
- [x] .rovodev/todo-app-002_server_graphql_projects_queries.md (depends on 001)
- [x] .rovodev/todo-app-006_server_graphql_tags_crud.md (depends on 005)
- [x] .rovodev/todo-app-007_server_migrations_tasks_task_tags.md (depends on 001,005)
- [x] .rovodev/todo-app-011_server_migrations_saved_views.md (depends on 001)
- [x] .rovodev/todo-app-013_server_migrations_recurring_series.md (depends on 001,005)
Mobile
- [x] .rovodev/todo-app-031_mobile_app_wiring_selector_app_config.md (depends on 030)
- [ ] .rovodev/todo-app-044_mobile_timezone_plumbing.md (depends on 051)
- [x] .rovodev/todo-app-045_mobile_persist_last_selected_project_view.md (depends on 052)

Next up when Wave 2 items complete:
- Project mutations and permissions (003,004)
- Derived fields and Tasks list (010,008)
- Navigation scaffolding (032)


Wave 3 — Mutations, derived fields, navigation, and docs
Backend
- [ ] .rovodev/todo-app-003_server_graphql_projects_mutations.md (depends on 001,002)
- [ ] .rovodev/todo-app-010_server_tasks_derived_fields.md (depends on 007)
- [x] .rovodev/todo-app-014_server_graphql_create_series_validation.md (depends on 013,006)
Docs
- [ ] .rovodev/todo-app-060_docs_update_database.md (after migrations: 001,005,007,011,013)
Mobile
- [ ] .rovodev/todo-app-032_mobile_navigation_tabs_placeholders.md (depends on 031)
- [ ] .rovodev/todo-app-036_mobile_tag_manager.md (depends on 006)

Next up when Wave 3 items complete:
- Permissions (004), Tasks list (008), and History (018)


Wave 4 — Permissions and read queries finalized
Backend
- [ ] .rovodev/todo-app-004_server_permissions_projects.md (depends on 003)
- [ ] .rovodev/todo-app-008_server_graphql_tasks_list.md (depends on 007,010,006)
- [ ] .rovodev/todo-app-018_server_graphql_history_query.md (depends on 007,010)

Next up when Wave 4 items complete:
- Task mutations (009), Saved Views GraphQL (012), Integration tests (021)


Wave 5 — Mutations and saved views
Backend
- [ ] .rovodev/todo-app-009_server_graphql_tasks_mutations.md (depends on 008)
- [ ] .rovodev/todo-app-012_server_graphql_saved_views.md (depends on 011,004)
- [ ] .rovodev/todo-app-021_server_tests_integration.md (depends on 010,018)

Next up when Wave 5 items complete:
- Recurrence: generation, updates, top-up (015,016,017)


Wave 6 — Recurrence generation enabling + Mobile stale-write UX
Backend
- [ ] .rovodev/todo-app-015_server_generation_series_occurrences.md (depends on 014,007,009)
Mobile
- [ ] .rovodev/todo-app-046_mobile_stale_write_retry_ux.md (depends on 009,004)

Next up when Wave 6 items complete:
- Recurrence update/regeneration and top-up triggers (016,017) and begin backend unit tests (020)


Wave 7 — Recurrence update/top-up and backend unit tests
Backend
- [ ] .rovodev/todo-app-016_server_update_series_regeneration.md (depends on 015)
- [ ] .rovodev/todo-app-017_server_series_topup_on_completion.md (depends on 015,009)
- [ ] .rovodev/todo-app-020_server_tests_unit.md (depends on features as delivered; schedule here to cover recurrence, permissions, concurrency, derivations)

Next up when Wave 7 items complete:
- Shared GraphQL documents, series management UI (043)


Wave 8 — Shared GraphQL docs and series UI
Shared
- [ ] .rovodev/todo-app-050_shared_graphql_documents.md (depends on 002,006,008,012,014,016,018)
Mobile
- [ ] .rovodev/todo-app-043_mobile_series_management.md (depends on 016)

Next up when Wave 8 items complete:
- Projects list (033) and History tab (041) unblock; then Tasks list (037) and Task form (038)


Wave 9 — Projects UI and History (unblocks Tasks screens)
Mobile
- [ ] .rovodev/todo-app-033_mobile_projects_list.md (depends on 050,002,031)
- [ ] .rovodev/todo-app-041_mobile_history_tab.md (depends on 018,050,051,052)

Next up when Wave 9 items complete:
- Tasks list (037), Task form (038), and docs (061) can proceed; Project settings/members (034,035) also unlock


Wave 10 — Tasks list and task form; project settings/members; docs
Mobile
- [ ] .rovodev/todo-app-037_mobile_tasks_list_grouping_polling_tz.md (depends on 010,050,051,052,033)
- [ ] .rovodev/todo-app-038_mobile_task_form.md (depends on 009,006,033)
- [ ] .rovodev/todo-app-034_mobile_project_settings.md (depends on 003,033)
- [ ] .rovodev/todo-app-035_mobile_project_members.md (depends on 003,033)
Docs
- [ ] .rovodev/todo-app-061_docs_update_architecture_and_mobile_readme.md (depends on 031,033)

Next up when Wave 10 items complete:
- Swipe actions (039), Saved Views UI (040), and Recurrence fields (042)


Wave 11 — Saved Views UI, swipe actions, recurrence fields
Mobile
- [ ] .rovodev/todo-app-039_mobile_task_swipe_actions.md (depends on 009,037)
- [ ] .rovodev/todo-app-040_mobile_saved_views_ui.md (depends on 012,033,037)
- [ ] .rovodev/todo-app-042_mobile_recurrence_fields_in_form.md (depends on 038; fully enabled by 014,015)

Next up when Wave 11 items complete:
- Final QA pass; ensure 020/021 test coverage is green; any open TODOs are tracked

Critical path (for awareness)
1) 001 → 002 → 003 → 004 → 012 → 050 → 033 → 037/038 → 039/040/042
2) 005 → 006
3) 001+005 → 007 → 010 → 008 → 009 → 015 → 016/017
4) 007 → 010 → 018 → 021 → 050 → 041


Dependency recap (adjacency form)
- Projects: 001 → 002 → 003 → 004 → 012
- Tags: 005 → 006
- Tasks: (001,005) → 007 → 010 → 008 → 009
- Saved Views: 001 → 011 → 012
- Recurrence: (001,005) → 013 → 014 → (with 007,009) → 015 → 016,017
- History: (007,010) → 018
- Shared: 002,006,008,012,014,016,018 → 050; 051 (start early, finalize after 008,010); 052 independent
- Mobile: 030 → 031 → 032; 033 (needs 050,002,031); 034/035 (needs 003,033); 036 (needs 006); 037 (needs 010,050,051,052,033); 038 (needs 009,006,033); 039 (needs 009,037); 040 (needs 012,033,037); 041 (needs 018,050,051,052); 042 (needs 038; fully enabled by 014,015); 043 (needs 016); 044 (needs 051); 045 (needs 052); 046 (needs 009,004)


Operating guidance
- At any time, pick the earliest wave with remaining unchecked items; all items in that wave are safe to start in parallel.
- If an item in a later wave becomes unblocked early because all its dependencies are checked, you may start it without waiting for its peers in the same wave.
- Keep the “Critical path” in mind to minimize end-to-end duration while exploiting parallelism.
