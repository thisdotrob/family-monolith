# Ticket 006: Update Main Module and Schema Building

## ID: 006
**Dependencies**: 001, 002, 003, 004, 005
**Parallel-safe**: No (depends on all previous restructuring)

## Objective
Update the main `mod.rs` file to use the new modular structure, implement the final schema building logic, and remove obsolete code.

## Tasks
1. Remove all extracted code from `mod.rs` (types, queries, mutations)
2. Update module imports to use new structure
3. Implement new schema building using `MergedObject` pattern
4. Create clean `QueryRoot` and `CombinedMutation` structures
5. Remove obsolete files (`unauthenticated.rs`)

## Implementation Details

### New Main Module Structure (`mod.rs`):
```rust
use sqlx::SqlitePool;
use async_graphql::{EmptySubscription, Schema, MergedObject};

// Import organized modules
mod types;
mod shared;
mod takenlijst;

// Import organized test modules
mod tests_history;
mod tests_integration;
mod tests_recurring_series;
mod tests_saved_views;

// Re-export types for convenience
pub use types::*;

// Import app-level query and mutation structs
use shared::{SharedQuery, SharedMutation};
use takenlijst::{TakenlijstQuery, TakenlijstMutation};

// Build final schema structures
#[derive(MergedObject, Default)]
pub struct QueryRoot(SharedQuery, TakenlijstQuery);

#[derive(MergedObject, Default)]
pub struct CombinedMutation(SharedMutation, TakenlijstMutation);

pub type AppSchema = Schema<QueryRoot, CombinedMutation, EmptySubscription>;

pub fn build(pool: SqlitePool) -> AppSchema {
    Schema::build(QueryRoot::default(), CombinedMutation::default(), EmptySubscription)
        .data(pool)
        .limit_depth(5)
        .limit_complexity(50)
        .disable_introspection()
        .finish()
}
```

### Module Import Updates
- Remove imports for old `auth`, `unauthenticated` modules
- Add imports for new `types`, `shared`, `takenlijst` modules
- Update any remaining references to use new structure

### Clean Up Tasks
1. Remove all type definitions from `mod.rs` (moved to `types/`)
2. Remove `QueryRoot` implementation (replaced with queries in individual files)
3. Remove old `CombinedMutation` structure (replaced with new one)
4. Remove `pub use` statements for old mutation structs
5. Remove imports that are no longer needed

## Files to Remove
- `server/src/graphql/unauthenticated.rs` (functionality moved to `shared/`)
- Any other obsolete files identified during implementation

## Verification
- Code compiles successfully
- Schema builds correctly with `build()` function
- All GraphQL queries and mutations accessible through new structure
- No changes to external GraphQL API
- All tests continue to pass
- Clean module structure with no circular dependencies

## Files Modified
- `server/src/graphql/mod.rs` - Complete restructure to use new organization

## Files Removed
- `server/src/graphql/unauthenticated.rs`
- Any other identified obsolete files

## Integration Points
- Ensure `server/src/lib.rs` or other files importing GraphQL module work correctly
- Verify that the schema building in server startup code functions properly
- Check that all authentication and authorization flows remain intact