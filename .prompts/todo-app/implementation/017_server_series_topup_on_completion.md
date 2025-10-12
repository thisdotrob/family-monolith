# 017 — Server: Series Top-up on Done/Abandoned

Spec refs: §§6,8

## Summary

When an occurrence is marked done or abandoned, ensure the series maintains 5 future occurrences by generating new ones.

## Scope

Module placement and structure
- Hook logic within takenlijst task mutations under `server/src/graphql/takenlijst/`, file-per-resolver.
- Put any shared GraphQL types/inputs in `server/src/graphql/types/`.
- Ensure `server/src/graphql/mod.rs` merges takenlijst mutations via `MergedObject`.
- Tests: add unit tests under `server/src/graphql/takenlijst/tests/` and any integration tests under `server/src/graphql/tests_*.rs`.



Note: When you complete this ticket, update todo-app-implementation-sequencing-plan.md to check off .rovodev/todo-app-017_server_series_topup_on_completion.md in the appropriate wave.
