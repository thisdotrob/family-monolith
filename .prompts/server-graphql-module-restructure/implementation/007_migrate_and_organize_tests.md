# Ticket 007: Migrate and Organize Tests

## ID: 007
**Dependencies**: 001, 002, 003, 004, 005, 006
**Parallel-safe**: No (depends on all module restructuring)

## Objective
Migrate and organize test files to match the new module structure, updating imports and ensuring all tests continue to pass.

## Tasks
1. Create test directory structure to match new organization
2. Move existing integration tests to appropriate locations
3. Create unit test files for individual resolvers
4. Update all test imports to use new module structure
5. Verify all tests pass with new organization

## Test Organization Structure

### Keep at Top Level (Integration Tests):
- `tests_integration.rs` - Keep at `server/src/graphql/tests_integration.rs`
- `tests_history.rs` - Keep at `server/src/graphql/tests_history.rs`  
- `tests_recurring_series.rs` - Keep at `server/src/graphql/tests_recurring_series.rs`
- `tests_saved_views.rs` - Keep at `server/src/graphql/tests_saved_views.rs`

### Create Unit Test Structure:
```
server/src/graphql/
├── shared/tests/
│   ├── login.rs           # Tests for login mutation
│   ├── refresh_token.rs   # Tests for refresh token mutation
│   ├── logout.rs          # Tests for logout mutation
│   └── me.rs              # Tests for me query
├── takenlijst/tests/
│   ├── create_project.rs  # Tests for create project mutation
│   ├── update_project.rs  # Tests for update project mutation
│   ├── projects_query.rs  # Tests for projects query
│   ├── create_tag.rs      # Tests for create tag mutation
│   ├── tags_query.rs      # Tests for tags query
│   └── ...                # Tests for all other resolvers
```

## Implementation Details

### Update Integration Tests
1. Update imports in all top-level test files:
   - Replace imports from old structure
   - Use new `crate::graphql::types::*` for types
   - Use new `crate::graphql::{QueryRoot, CombinedMutation}` for schema

### Create Unit Test Templates
Each unit test file should follow this pattern:
```rust
// Example: shared/tests/login.rs
use crate::graphql::types::{LoginInput, LoginPayload};
use crate::graphql::shared::LoginMutation;
use async_graphql::{Context, Object};
use sqlx::SqlitePool;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_login_success() {
        // Test implementation
    }
    
    #[tokio::test] 
    async fn test_login_invalid_credentials() {
        // Test implementation
    }
}
```

### Update Module Test Organization
1. Create `shared/tests/mod.rs` to organize shared tests
2. Create `takenlijst/tests/mod.rs` to organize app-specific tests
3. Update parent module files to include test modules properly

### Migration Strategy
1. **Phase 1**: Update imports in existing integration tests
2. **Phase 2**: Create unit test directory structure
3. **Phase 3**: Move relevant test code from integration tests to unit tests where appropriate
4. **Phase 4**: Create additional unit tests for resolvers that lack coverage

## Verification
- All existing tests pass with updated imports
- New unit test structure is in place
- Test coverage is maintained or improved
- Tests can be run individually by module/resolver
- Integration tests still verify end-to-end functionality

## Files Created
- `server/src/graphql/shared/tests/mod.rs`
- `server/src/graphql/shared/tests/login.rs`
- `server/src/graphql/shared/tests/refresh_token.rs`
- `server/src/graphql/shared/tests/logout.rs`
- `server/src/graphql/shared/tests/me.rs`
- `server/src/graphql/takenlijst/tests/mod.rs`
- Unit test files for each takenlijst resolver

## Files Modified
- `server/src/graphql/tests_integration.rs` - Update imports
- `server/src/graphql/tests_history.rs` - Update imports
- `server/src/graphql/tests_recurring_series.rs` - Update imports
- `server/src/graphql/tests_saved_views.rs` - Update imports
- `server/src/graphql/shared/mod.rs` - Include tests module
- `server/src/graphql/takenlijst/mod.rs` - Include tests module

## Testing Commands
After implementation, verify with:
```bash
cd server
cargo test graphql::tests_integration
cargo test graphql::shared::tests
cargo test graphql::takenlijst::tests
cargo test graphql # Run all GraphQL tests
```