# Family Takenlijst (Mobile Todo App) — Developer-Ready Specification

## 1) Overview
- App: Mobile-only (Expo/React Native) todo app for a single-family deployment.
- Deployment: Reuse repo’s existing auth and backend GraphQL server. App ID (slug): `takenlijst`. Display name: "Family Takenlijst".
- Multi-app setup: Add a new mobile app under the existing pattern. No web app required.
- Sync model: Online only. Polling fetch for tasks of the currently selected project every 10s; immediate read sync after any write. No server push/subscriptions.

## 2) Goals and Non-Goals
- Goals: Simple, fast shared task management with project-level sharing, recurrence, tags, assignee, and saved views. Clear Done vs Abandoned history. Light UX, dependable data rules.
- Non-Goals (MVP): No notifications, no invites, no comments/attachments, no web UI, no offline edits, no household/organization entity, no cross-project task moves.

## 3) Primary Personas
- Family members with their own login (existing username/password). All actions within family context (single-tenant deployment).

## 4) Navigation (Tabs)
- Projects
- Tasks (for selected project; open last-selected project on launch if any, otherwise Projects)
- Saved Views (project-shared views)
- History (toggle between Done and Abandoned)

## 5) Core Concepts & Permissions
- Projects
  - Each project has an owner and members. Members are added by exact username (no invite flow).
  - Membership is permanent: owners cannot remove members; members cannot leave; ownership is permanent (no transfer).
  - Owner-only actions: rename, archive/unarchive. Archived projects hidden by default (toggle to include archived); tasks read-only while archived.
  - Project creation: any user can create; starts private to creator until members are added.
- Tasks
  - Status: `todo`, `done`, `abandoned`. Completing sets `done`. Abandoning sets `abandoned`. Only abandoned tasks can be restored (to `todo`). Once done/abandoned, tasks are otherwise immutable.
  - Author (creator) and Completer (who marked done/abandoned). Optional single assignee; defaults to creator; any member can change.
  - No task moves between projects (MVP).
  - Actions allowed by any project member: create/edit (todo only), complete, abandon, restore abandoned.
- Tags
  - Global, shared across all family users. Name only; case-insensitive uniqueness with normalization (trim/collapse spaces). Store without leading `#` (accept `#` on input).
  - Any member can create/rename/delete tags. Delete is prevented if in use. Rename collisions merge usages into the existing tag.
- Saved Views (project-shared)
  - Stored per project. Any member can create/rename/delete.
  - Views capture only: Status filter, Assignee filter, Tags (AND) filter. No grouping/sort/visibility stored.
  - A project-wide default Saved View can be set/cleared by any member. If deleted, fallback to base default.

## 6) Scheduling & Recurrence
- Floating times (no timezone on stored values). Client supplies its timezone on each relevant request. Server uses that for validation, expansion, and derived fields.
- Fields:
  - scheduledDate (YYYY-MM-DD, nullable)
  - scheduledTimeMinutes (0–1439, nullable)
  - deadlineDate (YYYY-MM-DD, nullable)
  - deadlineTimeMinutes (0–1439, nullable)
- Grouping/overdue rules (server-derived, based on client timezone):
  - Overdue if scheduled datetime < now (for datetime) or scheduled date < today (for date-only).
  - If Scheduled absent, use Deadline to derive overdue/bucket and ordering.
  - Default grouping on Tasks screen: Overdue / Today / Tomorrow / Upcoming / No date
  - No-date bucket is sorted alphabetically by title. Overall default sort is: Scheduled, then Deadline, then Created.
- Recurrence
  - Representation: RFC 5545 RRULE string + floating DTSTART (date + optional minutes). No COUNT/UNTIL (we maintain 5 future occurrences).
  - Anchor: scheduled date/time. Deadline on generated occurrences uses a non-negative offset from scheduled (minutes granularity; range 0 to +365 days).
  - Creation constraints: first occurrence must be in the future or today. If time present, first datetime must be >= now in client timezone. No past occurrences allowed on creation.
  - Generation strategy: Pre-generate 5 future occurrences per series. Top-up only occurs when an occurrence is marked done/abandoned, if the future count drops below 5.
  - Occurrence model: Pre-generated occurrences are independent tasks linked to a series (fully editable tasks, but see propagation below).
  - Editing behavior:
    - You can only edit occurrences with status `todo`.
    - Updating one todo occurrence propagates core content (title, description, tags, assignee) to all todo occurrences in the same series and updates the series template. Schedule/deadline remain per-occurrence.
    - Editing the series RRULE or deadline offset regenerates all not-yet-completed/abandoned occurrences from now onward (update their schedule/deadline to match new rule). Past and done/abandoned remain unchanged.
  - No pause/resume feature for series.

## 7) Data Model (Conceptual)
- User (existing)
  - id, username, firstName (existing in repo)
- Project
  - id, name (required, max 60, trim/collapse, duplicates allowed), ownerId, archivedAt (nullable), createdAt, updatedAt
  - Membership: join table (projectId, userId, createdAt)
- Tag
  - id, name (normalized unique, case-insensitive), createdAt, updatedAt
- Task
  - id, projectId, authorId, assigneeId (nullable), seriesId (nullable)
  - title (required, max 120), description (optional, max 5000)
  - status enum('todo','done','abandoned')
  - scheduledDate, scheduledTimeMinutes, deadlineDate, deadlineTimeMinutes (all nullable)
  - completedAt, completedBy (nullable)
  - abandonedAt, abandonedBy (nullable)
  - createdAt, updatedAt
  - tags: many-to-many (task_tags: taskId, tagId)
- RecurringSeries
  - id, projectId, createdBy
  - title, description, tags (defaults for new occurrences)
  - assigneeId (nullable default)
  - rrule (text), dtstartDate, dtstartTimeMinutes (nullable)
  - deadlineOffsetMinutes (non-negative)
  - createdAt, updatedAt
- SavedView (project-shared)
  - id, projectId, name (required, max 60, unique per project case-insensitive, trim/collapse)
  - filters: status subset, assignee (memberId | Unassigned | Me), tags (AND list)
  - createdBy, createdAt, updatedAt
- ProjectDefaultView
  - projectId -> savedViewId (nullable)

## 8) API (GraphQL) Design
- General
  - Auth: reuse existing login/refresh/logout.
  - Errors: Native GraphQL errors with standardized extensions.code values: VALIDATION_FAILED, PERMISSION_DENIED, NOT_FOUND, CONFLICT_STALE_WRITE, RATE_LIMITED, BAD_REQUEST, INTERNAL_SERVER_ERROR.
  - Concurrency: Stale-write protection via updatedAt. Mutations take lastKnownUpdatedAt; server rejects if stale with extensions.code=CONFLICT_STALE_WRITE.
  - Pagination: Offset + limit with totalCount.
  - Client sends timezone (IANA tz name) on queries/mutations that need validation/derived fields.

- Types (suggested)
  - enum TaskStatus { todo, done, abandoned }
  - enum TaskBucket { Overdue, Today, Tomorrow, Upcoming, NoDate }
  - type Task {
    id: ID!
    projectId: ID!
    authorId: ID!
    assigneeId: ID
    seriesId: ID
    title: String!
    description: String
    status: TaskStatus!
    scheduledDate: String
    scheduledTimeMinutes: Int
    deadlineDate: String
    deadlineTimeMinutes: Int
    completedAt: String
    completedBy: ID
    abandonedAt: String
    abandonedBy: ID
    createdAt: String!
    updatedAt: String!
    // Derived (requires timezone input on list/query):
    isOverdue: Boolean!
    bucket: TaskBucket!
  }
  - type Tag { id: ID!, name: String! }
  - type Project { id: ID!, name: String!, ownerId: ID!, archivedAt: String, createdAt: String!, updatedAt: String! }
  - type SavedView { id: ID!, projectId: ID!, name: String!, filters: SavedViewFilters!, createdBy: ID!, createdAt: String!, updatedAt: String! }
  - input SavedViewFilters { statuses: [TaskStatus!]!, assignee: ID, includeUnassigned: Boolean, assignedToMe: Boolean, tagIds: [ID!]! }
  - type RecurringSeries {
    id: ID!
    projectId: ID!
    createdBy: ID!
    title: String!
    description: String
    assigneeId: ID
    rrule: String!
    dtstartDate: String!
    dtstartTimeMinutes: Int
    deadlineOffsetMinutes: Int!
    createdAt: String!
    updatedAt: String!
    // Defaults for generation:
    defaultTagIds: [ID!]!
  }
  - type PagedTasks { items: [Task!]!, totalCount: Int! }

- Queries (key ones)
  - me: User
  - projects(includeArchived: Boolean = false, offset: Int = 0, limit: Int = 50): [Project!]!
  - projectMembers(projectId: ID!): [User!]!
  - tags(offset: Int = 0, limit: Int = 200): [Tag!]!
  - savedViews(projectId: ID!): [SavedView!]!
  - projectDefaultSavedView(projectId: ID!): SavedView
  - tasks(
      projectId: ID!,
      statuses: [TaskStatus!] = [todo],
      assignee: ID,
      includeUnassigned: Boolean,
      assignedToMe: Boolean,
      tagIds: [ID!],
      search: String,
      // Dates are optional here per MVP; grouping/derived requires tz
      offset: Int = 0,
      limit: Int = 20,
      timezone: String!
    ): PagedTasks
  - history(
      statuses: [TaskStatus!]!, // expect [done] or [abandoned]
      projectId: ID,
      tagIds: [ID!],
      completerId: ID,
      fromDate: String, // default last 7 days
      toDate: String,
      offset: Int = 0,
      limit: Int = 20,
      timezone: String!
    ): PagedTasks // items come pre-sorted and suitable for day grouping

- Mutations (key ones)
  - createProject(name: String!): Project
  - renameProject(projectId: ID!, name: String!, lastKnownUpdatedAt: String!): Project
  - archiveProject(projectId: ID!, lastKnownUpdatedAt: String!): Project
  - unarchiveProject(projectId: ID!, lastKnownUpdatedAt: String!): Project
  - addProjectMemberByUsername(projectId: ID!, username: String!): Boolean // errors if not found/already member
  - setProjectDefaultSavedView(projectId: ID!, savedViewId: ID): Boolean // null clears; any member allowed

  - createTag(name: String!): Tag // normalize, dedupe; may return existing if collision
  - renameTag(tagId: ID!, newName: String!): Tag // merge on collision
  - deleteTag(tagId: ID!): Boolean // prevent if in use

  - createTask(input: CreateTaskInput!): Task
  - updateTask(id: ID!, input: UpdateTaskInput!, lastKnownUpdatedAt: String!): Task
  - completeTask(id: ID!, lastKnownUpdatedAt: String!): Task // sets done, records completedAt/By
  - abandonTask(id: ID!, lastKnownUpdatedAt: String!): Task // sets abandoned, records abandonedAt/By
  - restoreTask(id: ID!, lastKnownUpdatedAt: String!): Task // only from abandoned -> todo (clears abandoned fields)

  - createSavedView(projectId: ID!, name: String!, filters: SavedViewFiltersInput!): SavedView
  - updateSavedView(id: ID!, name: String, filters: SavedViewFiltersInput, lastKnownUpdatedAt: String!): SavedView
  - deleteSavedView(id: ID!): Boolean

  - createRecurringSeries(input: CreateSeriesInput!): RecurringSeries // validates RRULE and start vs now
  - updateRecurringSeries(id: ID!, input: UpdateSeriesInput!, lastKnownUpdatedAt: String!): RecurringSeries // may regenerate future occurrences

- Inputs (high level)
  - input CreateTaskInput {
      projectId: ID!,
      title: String!,
      description: String,
      assigneeId: ID,
      scheduledDate: String,
      scheduledTimeMinutes: Int,
      deadlineDate: String,
      deadlineTimeMinutes: Int,
      tagIds: [ID!]
    }
  - input UpdateTaskInput { title, description, assigneeId, scheduledDate, scheduledTimeMinutes, deadlineDate, deadlineTimeMinutes, tagIds }
  - input SavedViewFiltersInput { statuses: [TaskStatus!]!, assignee: ID, includeUnassigned: Boolean, assignedToMe: Boolean, tagIds: [ID!]! }
  - input CreateSeriesInput {
      projectId: ID!,
      title: String!,
      description: String,
      assigneeId: ID,
      defaultTagIds: [ID!],
      rrule: String!,
      dtstartDate: String!,
      dtstartTimeMinutes: Int,
      deadlineOffsetMinutes: Int!
    }
  - input UpdateSeriesInput { title, description, assigneeId, defaultTagIds, rrule, dtstartDate, dtstartTimeMinutes, deadlineOffsetMinutes }

## 9) Sync, Polling, and Offline
- Polling: Every 10s while app is active, fetch the Tasks list for the currently selected project using the active filters (usually todo only). Do not poll other entities by default; fetch them when screens need them or on pull-to-refresh.
- After any successful mutation, immediately refetch the affected list/details.
- Backgrounding: Pause polling while app is backgrounded; resume on foreground.
- Offline: Read-only with an “Offline” banner; all write actions disabled.

## 10) Validation Rules
- Title: required, max 120 chars.
- Description: optional, max 5000 chars.
- Tags: name only, case-insensitive unique (normalized). Max length 30. Allowed characters: letters, numbers, spaces, - _ / + # and emojis. Trim, collapse spaces. Leading `#` stripped on save. Rename collisions merge.
- Projects: name required, max 60, trim/collapse spaces, duplicates allowed.
- Saved Views: name required, max 60, trim/collapse, unique per project (case-insensitive). Filters limited to statuses, assignee, tags.
- Recurrence: RRULE must be valid; first occurrence must be in the future or today; with time present, first datetime must be >= now in client timezone. Deadline offset: minutes, 0..+525600 (365 days).

## 11) Sorting, Grouping, and Derived Fields
- Server computes and returns `isOverdue` and `bucket` (Overdue/Today/Tomorrow/Upcoming/NoDate) per task using the client-provided timezone.
- Default sort order for Tasks: Scheduled (date/time), then Deadline (date/time), then Created time. Within NoDate, sort by title (A→Z).
- History (Done/Abandoned): default window last 7 days, paginated; grouped by day; aggregate across all projects with a project filter.

## 12) Pagination
- Offset+limit for Task lists and History. Default limit 20. Return `totalCount` with each list.

## 13) Concurrency & Conflict Handling
- Clients send `lastKnownUpdatedAt` for updates; server compares with current `updatedAt`.
- If mismatch, reject with GraphQL error code `CONFLICT_STALE_WRITE`. Client refreshes and retries.
- For list refreshes, server sorts and derives fields based on provided timezone.

## 14) Security & Permissions Summary
- Auth: Existing JWT + refresh token flow and guards.
- Project owner: rename, archive/unarchive. Cannot remove members; cannot transfer ownership (permanent). Members cannot leave.
- Any member: create/edit todo tasks, complete, abandon, restore abandoned; manage tags and saved views; set/clear project default view.
- Archived project: tasks read-only; project hidden by default from selectors unless “Include archived” is toggled.

## 15) UX Details
- Project Tasks screen default:
  - Filter: todo only by default.
  - Grouping: Overdue / Today / Tomorrow / Upcoming / No date.
  - Quick filters: tag chips in header.
  - Item quick actions: tap opens edit; swipe right = Complete; swipe left = Abandon; long-press menu (Restore only visible in Abandoned list within History).
- Task creation: full-screen form (title, description, assignee, tags, schedule/deadline, recurrence toggle + raw RRULE input fields).
- Saved Views tab: list of project views; apply by tap. Header overflow: open Tag Manager.
- History tab: toggle between Done and Abandoned. Default shows last 7 days; infinite scroll/pagination to go back further. Grouped by day. Filters available: project selector, tags (AND), completer, date range.
- Project Settings: accessed from Tasks screen header menu; actions include rename, archive/unarchive, add member by exact username, view members, set/clear default saved view.
- Onboarding: minimal empty-state prompts (e.g., Create Project, New Task).
- Appearance: Light mode only.

## 16) Implementation Notes (Repo Integration)
- Mobile app module
  - App slug: `takenlijst`; display name: "Family Takenlijst".
  - Follow `mobileapp` patterns (Expo SDK 53). Add app entry module under `apps/mobile/takenlijst` (HomePage.tsx + index.ts) and wire selection (currently hard-coded to `placeholder`; extend selector to allow `takenlijst`).
- Backend
  - Extend Rust GraphQL schema with new entities/tables/migrations (projects, members, tags, tasks, task_tags, recurring_series, saved_views, project_default_view, etc.).
  - Use server-side derived fields for Tasks (isOverdue/bucket) using client timezone from query args.
  - Enforce validation and concurrency rules via GraphQL errors with standardized extensions.code.
  - Apply migrations automatically on startup (matches existing server behavior).
- Shared code
  - Reuse shared Apollo client and AuthContext. Add storage keys for project selection and saved view where helpful.

## 17) Testing & QA Criteria
- Unit tests (server):
  - Validation: tag normalization/uniqueness; project and saved view name rules; RRULE validation and first occurrence constraints; deadline offset bounds.
  - Concurrency: stale-write rejection for tasks/projects.
  - Permissions: owner vs member actions; archived project read-only enforcement.
  - Recurrence: top-up on done/abandoned only; correct regeneration on RRULE/offset changes.
- Integration tests (server):
  - Tasks listing derives isOverdue/bucket correctly per timezone.
  - Pagination returns totalCount and correct ordering.
- Mobile manual tests:
  - Polling refresh every 10s; immediate refresh after mutations.
  - Offline banner and disabled write actions.
  - History grouping and filters; per-project default saved view behavior.

## 18) Data Retention
- Keep completed and abandoned tasks forever (no auto-cleanup).

## 19) Open Extensions (Post-MVP)
- Notifications (local or push), comments/attachments, invites, moving tasks, manual ordering, advanced recurrence editor UI with preview, web parity, analytics/telemetry.

---

This specification consolidates all decisions from our iterative Q&A and is ready for implementation planning in this repository’s architecture.
