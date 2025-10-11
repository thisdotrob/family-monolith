# Ticket 003: Extract Remaining Queries from mod.rs

## ID: 003
**Dependencies**: 001, 002
**Parallel-safe**: No (depends on types and shared structure)

## Objective
Extract all remaining queries from the large `QueryRoot` implementation in `mod.rs` and organize them into appropriate app-specific or shared directories.

## Tasks
1. Analyze remaining queries in `QueryRoot` implementation
2. Categorize queries as either:
   - **Shared**: Cross-app queries → move to `shared/`
   - **Takenlijst-specific**: App-specific queries → move to `takenlijst/`
3. Extract each query into individual files
4. Update module structure to use `MergedObject` pattern

## Queries to Extract from mod.rs QueryRoot
Based on analysis, extract the following queries:

### Takenlijst-specific queries → `takenlijst/`:
- `projects` → `takenlijst/projects_query.rs` 
- `tags` → `takenlijst/tags_query.rs`
- `tasks` → `takenlijst/tasks_query.rs`
- `history` → `takenlijst/history_query.rs`
- `saved_views` → `takenlijst/saved_views_query.rs`
- `project_default_saved_view` → `takenlijst/project_default_saved_view_query.rs`

## Implementation Details

### Individual Query Files
Each query file should:
- Define its own struct (e.g., `ProjectsQuery`, `TagsQuery`)
- Import required types from `crate::graphql::types`
- Contain only the single query resolver
- Use `#[derive(Default)]` and `#[Object]` patterns

### Example Structure for `takenlijst/projects_query.rs`:
```rust
use crate::graphql::types::Project;
use crate::auth::Claims;
use async_graphql::{Context, Object};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct ProjectsQuery;

#[Object]
impl ProjectsQuery {
    async fn projects(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = false)] include_archived: bool,
        #[graphql(default = 0)] offset: i32,
        #[graphql(default = 50)] limit: i32,
    ) -> async_graphql::Result<Vec<Project>> {
        // Move existing projects query logic here
    }
}
```

### Update Takenlijst Module (`takenlijst/mod.rs`):
```rust
use async_graphql::MergedObject;

// Existing modules
pub mod projects;
pub mod tags;

// New query modules
mod projects_query;
mod tags_query;
mod tasks_query;
mod history_query;
mod saved_views_query;
mod project_default_saved_view_query;

pub use projects_query::ProjectsQuery;
pub use tags_query::TagsQuery;
pub use tasks_query::TasksQuery;
pub use history_query::HistoryQuery;
pub use saved_views_query::SavedViewsQuery;
pub use project_default_saved_view_query::ProjectDefaultSavedViewQuery;

#[derive(MergedObject, Default)]
pub struct TakenlijstQuery(
    ProjectsQuery,
    TagsQuery,
    TasksQuery,
    HistoryQuery,
    SavedViewsQuery,
    ProjectDefaultSavedViewQuery,
);

// Keep existing mutation exports
pub use projects::ProjectsMutation;
pub use tags::TagsMutation;

#[derive(MergedObject, Default)]
pub struct TakenlijstMutation(ProjectsMutation, TagsMutation);
```

## Verification
- Code compiles successfully
- All queries are in individual files
- `TakenlijstQuery` struct combines all app queries
- No changes to GraphQL schema structure
- All existing query functionality preserved

## Files Created
- `takenlijst/projects_query.rs`
- `takenlijst/tags_query.rs`
- `takenlijst/tasks_query.rs`
- `takenlijst/history_query.rs`
- `takenlijst/saved_views_query.rs`
- `takenlijst/project_default_saved_view_query.rs`

## Files Modified
- `server/src/graphql/mod.rs` - Remove QueryRoot implementation
- `server/src/graphql/takenlijst/mod.rs` - Add query organization