# Family Takenlijst (Mobile Todo App) — Delivery Blueprint

This blueprint translates the specification in `todo-app-spec.md` into an actionable implementation plan for this repository. It outlines architecture, milestones, and a right-sized breakdown into iterative tickets with clear dependencies to enable parallel work. All tickets are markdown files under `.rovodev/`.

## Architecture Overview (Repo Integration)
- Mobile: New Expo app module `apps/mobile/takenlijst` (slug `takenlijst`, display name "Family Takenlijst"), integrated via `mobileapp/src/selectMobileApp.ts` and `mobileapp/app.config.ts`.
- Backend: Extend Rust server (Axum + async-graphql + sqlx/SQLite) with schema, resolvers, and migrations:
  - Projects + Memberships
  - Tags + normalization/merge + in-use delete guard
  - Tasks + task_tags + derived fields (isOverdue, bucket)
  - Saved Views + project default
  - RecurringSeries + occurrence generation/top-up
  - History query
  - Concurrency via `updatedAt` + GraphQL errors with standardized `extensions.code`
- Shared: Add GraphQL documents and helpers for timezone and offline; reuse `shared/apollo` and `AuthContext`.

## Delivery Strategy & Milestones
1) Foundations
- Server: Projects + Tags + Tasks MVP (schema + migrations + permissions + pagination + derived fields) with tests
- Mobile: App shell, navigation, project selection, Tasks list (placeholder until API), timezone wiring
- Shared: GraphQL documents for Projects/Tags/Tasks

2) Saved Views + History
- Server: Saved Views (CRUD + default); History query
- Mobile: Saved Views UI + History tab

3) Recurrence
- Server: RecurringSeries (validation + generation + top-up + update/regeneration)
- Mobile: Raw RRULE UI + series management

4) Polish & QA
- Server: Broader tests for validation, permissions, concurrency, derivations
- Mobile: Polling, offline banner, error states, stale-write retry UX
- Docs: Update DB/architecture/mobile README

## Cross-Cutting Policies
- Validation: As per spec (titles, tags, views, RRULE, deadline offsets)
- Permissions: Owner-only project settings; members manage tasks/tags/views
- Concurrency: Update mutations accept `lastKnownUpdatedAt`; reject stale writes
- Timezone: Client provides tz on queries; server computes `isOverdue` and `bucket`
- Polling: 10s on Tasks while app active; immediate refetch after mutations

## Ticket Map (IDs and Dependencies)

Backend
- 001 Migrations: Projects/Memberships
- 002 GraphQL: Project types/queries — depends on 001
- 003 GraphQL: Project mutations (create/rename/archive/unarchive/addMember) — depends on 001,002
- 004 Permissions: Enforce owner/member rules — depends on 003
- 005 Migrations: Tags + unique normalization
- 006 GraphQL: Tags CRUD with merge & delete guard — depends on 005
- 007 Migrations: Tasks + task_tags — depends on 001,005
- 008 GraphQL: Task types + list with derived fields + pagination — depends on 007,010,006
- 009 GraphQL: Task mutations (create/update/complete/abandon/restore) + concurrency — depends on 008
- 010 Derived fields: isOverdue & bucket computation — depends on 007
- 011 Migrations: Saved Views + ProjectDefaultView — depends on 001
- 012 GraphQL: Saved Views CRUD + default setter — depends on 011,004
- 013 Migrations: RecurringSeries — depends on 001,005
- 014 GraphQL: CreateSeries with validation — depends on 013,006
- 015 Generation: Occurrence creation & linkage, 5-future policy — depends on 014,007,009
- 016 GraphQL: UpdateSeries with regeneration rules — depends on 015
- 017 Top-up: Trigger on done/abandoned — depends on 015,009
- 018 GraphQL: History query — depends on 007,010
- 019 Reserved (gap filler)
- 020 Tests: Unit (validation, permissions, concurrency, recurrence) — depends on features as delivered
- 021 Tests: Integration (derived fields, pagination, history) — depends on 010,018

Shared
- 050 GraphQL documents (Projects/Tags/Tasks/Saved Views/History/Series) — depends on 002,006,008,012,014,016,018
- 051 Timezone helper & Apollo integration patterns — can start early; finalize after 008,010
- 052 Hooks/adapters: Offline detection + storage keys — independent, used by Mobile

Mobile
- 030 App module skeleton (`takenlijst`) — independent
- 031 App wiring in selector + app.config — depends on 030
- 032 Navigation tabs & placeholders (Projects/Tasks/Saved Views/History) — depends on 031
- 033 Projects list (live) + archived toggle — depends on 050,002,031
- 034 Project settings (rename/archive/unarchive) — depends on 003,033
- 035 Members list + add-by-username — depends on 003,033
- 036 Tag manager UI — depends on 006
- 037 Tasks list (live) with grouping + polling + tz — depends on 010,050,051,052,033
- 038 Task create/edit form + validation + tags/assignee — depends on 009,006,033
- 039 Swipe actions complete/abandon/restore — depends on 009,037
- 040 Saved Views UI + default setter — depends on 012,033,037
- 041 History tab UI + filters — depends on 018,050,051,052
- 042 Recurrence fields in task form (raw RRULE) — depends on 038; fully enabled by 014,015
- 043 Series management screen (list/edit series) — depends on 016
- 044 Timezone plumbing — depends on 051; used by 037/041
- 045 Persist last-selected project & view — depends on 052; used by 033/040
- 046 Stale-write retry UX & error states — depends on 009,004; used by 034/038/039/040

Docs
- 060 Update DATABASE.md with new tables & relationships — after migrations
- 061 Update ARCHITECTURE.MD & mobile README for `takenlijst` — after 031/033

## Parallelization Notes
- Server: 001/005/007/011/013 can be developed in parallel; merge in the order above.
- Mobile: 030–032 can proceed immediately; 033–046 unblock as server endpoints land.
- Shared: 051/052 can start immediately; 050 follows server schema.

## Risks and Mitigations
- RRULE/DST correctness: use a mature RRULE crate and unit-test edge cases.
- Derived fields accuracy: snapshot tests across multiple timezones.
- Concurrency: ensure `updatedAt` precision and consistent serialization format.

## Definition of Done (overall)
- All GraphQL surfaces implemented with validation, permissions, concurrency.
- Mobile app provides UX for Projects, Tasks, Tags, Views, History, Recurrence per spec.
- Polling and immediate refetches verified; offline banner disables writes.
- Documentation updated.
