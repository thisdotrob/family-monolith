# Ticket 004: Extract Auth.rs Mutations to Individual Files

## ID: 004
**Dependencies**: 001, 002
**Parallel-safe**: Yes (can work in parallel with 003)

## Objective
Extract all mutations from `auth.rs` into individual resolver files, organizing them by app specificity (shared vs takenlijst-specific).

## Tasks
1. Analyze all mutations in `AuthenticatedMutation` implementation in `auth.rs`
2. Categorize mutations as either:
   - **Shared**: Cross-app mutations → move to `shared/`
   - **Takenlijst-specific**: App-specific mutations → move to `takenlijst/`
3. Extract each mutation into individual files
4. Update `auth.rs` to be removed entirely

## Mutations to Extract from auth.rs

### Shared mutations → `shared/`:
- `logout` → Already handled in ticket 002

### Takenlijst-specific mutations → `takenlijst/`:
- `create_recurring_series` → `takenlijst/create_recurring_series.rs`
- All other mutations found in `AuthenticatedMutation` (need to identify by examining full file)

## Implementation Details

### Analysis Required
First examine the complete `auth.rs` file to identify all mutations in `AuthenticatedMutation`:
- Check the full implementation to catalog all resolver methods
- Determine which are app-specific vs cross-app
- Some mutations may need to be determined by business logic context

### Individual Mutation Files
Each mutation file should:
- Define its own struct (e.g., `CreateRecurringSeriesMutation`)
- Import required types from `crate::graphql::types`
- Import required utilities like `crate::auth::Claims`, `require_member`
- Contain only the single mutation resolver
- Use `#[derive(Default)]` and `#[Object]` patterns

### Example Structure for `takenlijst/create_recurring_series.rs`:
```rust
use crate::graphql::types::{CreateSeriesInput, RecurringSeries};
use crate::auth::Claims;
use crate::auth::guard::require_member;
use async_graphql::{Context, Object, ErrorExtensions};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct CreateRecurringSeriesMutation;

#[Object]
impl CreateRecurringSeriesMutation {
    async fn create_recurring_series(
        &self,
        ctx: &Context<'_>,
        input: CreateSeriesInput,
    ) -> async_graphql::Result<RecurringSeries> {
        // Move existing create_recurring_series logic here
    }
}
```

### Update Takenlijst Module
Add the new mutations to the `TakenlijstMutation` struct in `takenlijst/mod.rs`.

## Verification
- Code compiles successfully
- All mutations are in individual files
- `auth.rs` file can be removed
- All mutation functionality preserved
- Proper error handling maintained

## Files Created
- `takenlijst/create_recurring_series.rs`
- Additional mutation files based on analysis of complete `auth.rs`

## Files Modified
- `server/src/graphql/takenlijst/mod.rs` - Add new mutation imports and struct updates
- `server/src/graphql/mod.rs` - Remove auth.rs import

## Files Removed
- `server/src/graphql/auth.rs` (after all mutations extracted)