# Server GraphQL Module Restructure — Implementation Sequencing Plan

This plan sequences the tickets in `.prompts/server-graphql-module-restructure/implementation` to maximize parallel execution while respecting explicit dependencies from each ticket. It presents clear “waves” (stages) of work, a dependency graph, and checklists so it’s obvious which tickets are next once others are completed.

Reference spec: `.prompts/server-graphql-module-restructure/docs/project-spec.md`

## Overview of Tickets
- 001_create_types_directory.md (Deps: —)
- 002_create_shared_directory.md (Deps: 001)
- 003_extract_remaining_queries_from_mod.md (Deps: 001, 002)
- 004_extract_auth_mutations.md (Deps: 001, 002)
- 005_restructure_takenlijst_existing_mutations.md (Deps: 001, 003, 004)
- 006_update_main_module_schema_building.md (Deps: 001, 002, 003, 004, 005)
- 007_migrate_and_organize_tests.md (Deps: 001, 002, 003, 004, 005, 006)
- 008_create_placeholder_app_structure.md (Deps: 001, 002, 006)
- 009_final_verification_and_cleanup.md (Deps: 001–008)

## Dependency Graph (Adjacency) 
Edges are A → B meaning A must complete before B can begin.
- 001 → 002, 003, 004, 005, 006, 007, 008, 009
- 002 → 003, 004, 006, 007, 008, 009
- 003 → 005, 006, 007, 009
- 004 → 005, 006, 007, 009
- 005 → 006, 007, 009
- 006 → 007, 008, 009
- 007 → 009
- 008 → 009

Equivalent grouped dependencies per ticket:
- 001: —
- 002: 001
- 003: 001, 002
- 004: 001, 002
- 005: 001, 003, 004
- 006: 001, 002, 003, 004, 005
- 007: 001, 002, 003, 004, 005, 006
- 008: 001, 002, 006
- 009: 001, 002, 003, 004, 005, 006, 007, 008

## Parallel Execution Waves
Each wave can be run fully in parallel by unlimited agents. Begin a wave when all tickets in prior waves are done. Check off tickets as they complete; the “Next work-ready tickets” section below updates accordingly.

Wave 0
- [x] .prompts/server-graphql-module-restructure/implementation/001_create_types_directory.md

Wave 1 (after 001)
- [x] .prompts/server-graphql-module-restructure/implementation/002_create_shared_directory.md

Wave 2 (after 001, 002) — fully parallel
- [x] .prompts/server-graphql-module-restructure/implementation/003_extract_remaining_queries_from_mod.md
- [x] .prompts/server-graphql-module-restructure/implementation/004_extract_auth_mutations.md

Wave 3 (after 001, 003, 004)
- [x] .prompts/server-graphql-module-restructure/implementation/005_restructure_takenlijst_existing_mutations.md

Wave 4 (after 001, 002, 003, 004, 005)
- [x] .prompts/server-graphql-module-restructure/implementation/006_update_main_module_schema_building.md

Wave 5 (after 006; note each ticket’s full deps are satisfied here) — fully parallel
- [ ] .prompts/server-graphql-module-restructure/implementation/007_migrate_and_organize_tests.md
- [x] .prompts/server-graphql-module-restructure/implementation/008_create_placeholder_app_structure.md

Wave 6 (final, after 001–008)
- [ ] .prompts/server-graphql-module-restructure/implementation/009_final_verification_and_cleanup.md

## “Next work-ready tickets” logic
Use this section during execution to know what to pick up next. At any moment, the next tickets are those whose dependencies are all checked off.

- If Wave 0 not complete: start 001.
- After 001 is checked: start 002.
- After 002 is checked: start 003 and 004 in parallel.
- After 003 and 004 are checked: start 005.
- After 005 is checked: start 006.
- After 006 is checked: start 007 and 008 in parallel.
- After 007 and 008 are checked: start 009.

## Notes for Coordination
- Tickets 003 and 004 are explicitly parallel-safe and should be executed concurrently to minimize critical path length.
- Ticket 005 gates on both 003 and 004 because it relies on queries (003) and mutations extraction (004).
- Ticket 006 integrates all prior restructure work; it is the critical consolidation step before tests (007) and placeholder app integration (008).
- Ticket 008 intentionally waits for 006 so that the main schema’s structure is stable before adding the placeholder app constructs.
- With unlimited agents, the longest path (critical path) is: 001 → 002 → (003,004 in parallel) → 005 → 006 → (007,008 in parallel) → 009.

## Progress Checklist (master)
Mark tickets done here as work proceeds.
- [ ] 001_create_types_directory.md
- [x] 002_create_shared_directory.md
- [x] 003_extract_remaining_queries_from_mod.md
- [x] 004_extract_auth_mutations.md
- [x] 005_restructure_takenlijst_existing_mutations.md
- [x] 006_update_main_module_schema_building.md
- [ ] 007_migrate_and_organize_tests.md
- [x] 008_create_placeholder_app_structure.md
- [ ] 009_final_verification_and_cleanup.md
