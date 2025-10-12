# 009 — Server: GraphQL Task Mutations with Concurrency

Spec refs: §§5,6,8,10,13

## Summary

Implement mutations: createTask, updateTask, completeTask, abandonTask, restoreTask, with validation, permissions, and concurrency checks

## Scope

Module placement and structure
- Place resolvers under `server/src/graphql/takenlijst/` using a file-per-resolver pattern (e.g., `task_create_mutation.rs`, `task_update_mutation.rs`, etc.).
- Put shared GraphQL types/inputs in `server/src/graphql/types/` where appropriate.
- Ensure `server/src/graphql/mod.rs` merges takenlijst mutations via `MergedObject` into the root schema.
- Follow the standardized error codes: INVALID_CREDENTIALS, TOKEN_EXPIRED, VALIDATION_FAILED, PERMISSION_DENIED, NOT_FOUND, CONFLICT_STALE_WRITE, INTERNAL_ERROR.
- Tests: add unit tests under `server/src/graphql/takenlijst/tests/` and any integration tests under `server/src/graphql/tests_*.rs`.

Note: When you complete this ticket, update todo-app-implementation-sequencing-plan.md to check off .rovodev/todo-app-009_server_graphql_tasks_mutations.md in the appropriate wave.
