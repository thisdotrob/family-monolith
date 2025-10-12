# 050 — Shared: GraphQL Documents for Projects/Tags/Tasks

Spec refs: §8, §16

## Summary
Add typed GraphQL documents for Projects, Tags, Tasks, Saved Views, History; export from shared package for reuse by web/mobile (mobile first).

## Scope
- Queries: me, projects, tags, tasks, savedViews, projectDefaultSavedView, history
  - Note: `projectMembers` was removed from the API during the GraphQL module restructure; exclude it from shared docs until a replacement surface exists.
- Mutations: project CRUD, tag CRUD, task CRUD, saved view CRUD/default, series create/update
- Add TypeScript types via codegen or manual typings aligned with existing pattern

## Acceptance Criteria
- All documents compile and can be imported from `@shared/graphql`
- Documents include required variables for timezone where needed

## Dependencies
- 002, 006, 008, 012, 014, 016, 018

## Implementation Steps
1) Create/extend files under `shared/graphql/`
2) Ensure exports map mirrors names in spec
3) Add minimal unit tests to check variables shape

## Out of Scope
- Apollo client setup (covered by 051)


Note: When you complete this ticket, update todo-app-implementation-sequencing-plan.md to check off .rovodev/todo-app-050_shared_graphql_documents.md in the appropriate wave.
