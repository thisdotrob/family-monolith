# Ticket 005: Restructure Existing Takenlijst Mutations

## ID: 005
**Dependencies**: 001, 003, 004
**Parallel-safe**: No (depends on types and query extraction)

## Objective
Restructure the existing `takenlijst/projects.rs` and `takenlijst/tags.rs` files to follow the file-per-resolver pattern, extracting individual mutations into separate files.

## Tasks
1. Analyze existing `takenlijst/projects.rs` and `takenlijst/tags.rs` files
2. Extract each mutation into individual files
3. Maintain existing `ProjectsMutation` and `TagsMutation` struct exports for backwards compatibility
4. Ensure proper integration with the updated module structure

## Current Files to Restructure

### From `takenlijst/projects.rs`:
Extract individual mutations (need to examine file to identify specific mutations):
- Likely: `create_project`, `update_project`, `delete_project`, etc.
- Each mutation → separate file: `takenlijst/create_project.rs`, etc.

### From `takenlijst/tags.rs`:  
Extract individual mutations (need to examine file to identify specific mutations):
- Likely: `create_tag`, `update_tag`, `delete_tag`, etc.
- Each mutation → separate file: `takenlijst/create_tag.rs`, etc.

## Implementation Details

### Analysis Phase
1. Examine `takenlijst/projects.rs` to identify all mutations in `ProjectsMutation`
2. Examine `takenlijst/tags.rs` to identify all mutations in `TagsMutation`
3. Create individual files for each mutation

### Individual Mutation Files
Each mutation file should:
- Define its own struct (e.g., `CreateProjectMutation`, `UpdateTagMutation`)
- Import required types from `crate::graphql::types`
- Import required utilities like `crate::auth::Claims`, `require_member`
- Contain only the single mutation resolver
- Use `#[derive(Default)]` and `#[Object]` patterns

### Example Structure for `takenlijst/create_project.rs`:
```rust
use crate::graphql::types::{CreateProjectInput, Project};
use crate::auth::Claims;
use async_graphql::{Context, Object};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct CreateProjectMutation;

#[Object]
impl CreateProjectMutation {
    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> async_graphql::Result<Project> {
        // Move existing create_project logic here
    }
}
```

### Maintain Backward Compatibility
Update `takenlijst/mod.rs` to:
- Import all individual mutation structs
- Re-export legacy `ProjectsMutation` and `TagsMutation` using `MergedObject`
- Integrate with the overall `TakenlijstMutation` structure

### Example Updated `takenlijst/mod.rs`:
```rust
use async_graphql::MergedObject;

// Individual mutation modules
mod create_project;
mod update_project;
mod delete_project;
mod create_tag;
mod update_tag;
mod delete_tag;
// ... other mutations

// Individual query modules (from ticket 003)
mod projects_query;
mod tags_query;
// ... other queries

// Re-export individual structs
pub use create_project::CreateProjectMutation;
pub use update_project::UpdateProjectMutation;
// ... all other exports

// Legacy compatibility exports
#[derive(MergedObject, Default)]
pub struct ProjectsMutation(CreateProjectMutation, UpdateProjectMutation, DeleteProjectMutation);

#[derive(MergedObject, Default)]
pub struct TagsMutation(CreateTagMutation, UpdateTagMutation, DeleteTagMutation);

// Overall app structure
#[derive(MergedObject, Default)]
pub struct TakenlijstMutation(ProjectsMutation, TagsMutation, /* other mutations from ticket 004 */);

#[derive(MergedObject, Default)]
pub struct TakenlijstQuery(/* queries from ticket 003 */);
```

## Verification
- Code compiles successfully
- All mutations are in individual files
- Legacy `ProjectsMutation` and `TagsMutation` still work
- Integration with overall module structure is clean
- All existing mutation functionality preserved

## Files Created
- Individual mutation files based on analysis of existing projects.rs and tags.rs
- e.g., `takenlijst/create_project.rs`, `takenlijst/update_project.rs`, etc.

## Files Modified
- `server/src/graphql/takenlijst/mod.rs` - Updated to import and organize all mutations

## Files Removed
- `server/src/graphql/takenlijst/projects.rs` (after mutations extracted)
- `server/src/graphql/takenlijst/tags.rs` (after mutations extracted)