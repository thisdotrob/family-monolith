# Ticket 001: Create Types Directory Structure

## ID: 001
**Dependencies**: None
**Parallel-safe**: Yes

## Objective
Create the `server/src/graphql/types/` directory and extract all GraphQL types and input objects from `mod.rs` into individual files.

## Tasks
1. Create `server/src/graphql/types/` directory
2. Create `server/src/graphql/types/mod.rs` with proper exports
3. Extract the following types from `mod.rs` into individual files:
   - `User` → `types/user.rs`
   - `Project` → `types/project.rs` 
   - `Tag` → `types/tag.rs`
   - `Task` → `types/task.rs`
   - `SavedView` → `types/saved_view.rs`
   - `SavedViewFilters` → `types/saved_view_filters.rs`
   - `RecurringSeries` → `types/recurring_series.rs`
   - All input objects from `unauthenticated.rs`:
     - `LoginInput` → `types/login_input.rs`
     - `LoginPayload` → `types/login_payload.rs`
     - `RefreshInput` → `types/refresh_input.rs` 
     - `RefreshPayload` → `types/refresh_payload.rs`
   - All input objects from `auth.rs`:
     - `LogoutInput` → `types/logout_input.rs`
     - `LogoutPayload` → `types/logout_payload.rs`
     - `CreateSeriesInput` → `types/create_series_input.rs`
     - And all other input/payload types found in auth.rs

## Implementation Details
- Each type file should contain only the type definition and necessary imports
- Use `pub` visibility for all types that need to be exported
- Update `types/mod.rs` to re-export all types with proper module declarations
- Keep all existing GraphQL derive macros and field annotations
- Preserve all serde derives where present

## Verification
- Code compiles successfully
- All type definitions are accessible via `use crate::graphql::types::*`
- No functionality changes - pure code organization
- All tests continue to pass

## Files Created
- `server/src/graphql/types/mod.rs`
- `server/src/graphql/types/user.rs`
- `server/src/graphql/types/project.rs`
- `server/src/graphql/types/tag.rs`
- `server/src/graphql/types/task.rs`
- `server/src/graphql/types/saved_view.rs`
- `server/src/graphql/types/saved_view_filters.rs`
- `server/src/graphql/types/recurring_series.rs`
- `server/src/graphql/types/login_input.rs`
- `server/src/graphql/types/login_payload.rs`
- `server/src/graphql/types/refresh_input.rs`
- `server/src/graphql/types/refresh_payload.rs`
- `server/src/graphql/types/logout_input.rs`
- `server/src/graphql/types/logout_payload.rs`
- `server/src/graphql/types/create_series_input.rs`
- Additional input/payload type files as discovered in auth.rs

## Files Modified
- `server/src/graphql/mod.rs` - Remove type definitions, add import from types
- `server/src/graphql/unauthenticated.rs` - Remove type definitions, add imports
- `server/src/graphql/auth.rs` - Remove type definitions, add imports