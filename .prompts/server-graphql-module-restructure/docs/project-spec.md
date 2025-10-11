# GraphQL Module Restructure Specification

## Overview
Restructure the server's GraphQL module organization from the current messy structure to a clean, file-per-resolver architecture with proper app-based and shared organization.

## Current State Analysis
- Mixed mutations and queries in single files (`auth.rs`, `mod.rs`)
- App-specific logic under `takenlijst/` subdirectory
- Shared authentication logic scattered across multiple files
- Test files mixed in with implementation files
- Large `mod.rs` file containing inline resolvers

## Target Architecture

### Directory Structure
```
server/src/graphql/
├── mod.rs                          # Main module file, imports and combines everything
├── types/                          # Shared GraphQL types and input objects
│   ├── mod.rs
│   ├── user.rs                     # User types
│   ├── loginInput.rs               # LoginInput type
│   ├── loginPayload.rs             # LoginPayload type
│   ├── refreshInput.rs             # RefreshInput type
│   ├── refreshPayload.rs           # RefreshPayload type
│   └── ...                         # One file per type/object
├── shared/                         # Cross-app mutations and queries
│   ├── mod.rs
│   ├── tests/                      # Tests for shared resolvers
│   │   ├── login.rs
│   │   ├── refreshToken.rs
│   │   └── me.rs
│   ├── login.rs                    # Login mutation
│   ├── refreshToken.rs             # Refresh token mutation  
│   ├── me.rs                       # Me query
│   └── logout.rs                   # Logout mutation
├── takenlijst/                     # Takenlijst app-specific resolvers
│   ├── mod.rs
│   ├── tests/                      # Tests for takenlijst resolvers
│   │   ├── createProject.rs
│   │   ├── updateProject.rs
│   │   └── ...
│   ├── createProject.rs            # Individual mutations
│   ├── updateProject.rs
│   ├── deleteProject.rs
│   ├── projects.rs                 # Projects query
│   ├── createTag.rs
│   ├── updateTag.rs
│   ├── deleteTag.rs
│   ├── tags.rs                     # Tags query
│   └── ...
├── placeholder/                    # Placeholder app-specific resolvers
│   ├── mod.rs
│   ├── tests/
│   └── ...
└── tests_integration.rs            # Integration tests stay at top level
└── tests_history.rs               # Integration tests stay at top level
```

## File Organization Rules

### 1. File Naming Convention
- Each mutation/query gets its own file named exactly after the resolver function
- Files containing mutations: `createProject.rs`, `updateProject.rs`, `login.rs`
- Files containing queries: `projects.rs`, `tags.rs`, `me.rs`
- If duplicate names exist across apps, suffix with app name: `createProjectMobile.rs`, `createProjectWeb.rs`

### 2. Directory Organization
- **Shared**: Mutations/queries used by multiple apps go in `shared/` (flat structure)
- **App-specific**: Directory names must match app IDs exactly (`takenlijst`, `placeholder`)
- **Types**: All shared GraphQL types and input objects in `types/` (one file per type)
- **Tests**: App-specific tests in `{app}/tests/`, shared tests in `shared/tests/`

### 3. App-Level Grouping
- Maintain separate `Query` and `Mutation` structs per app
- Example: `TakenlijstQuery`, `TakenlijstMutation`, `SharedQuery`, `SharedMutation`
- Each app's `mod.rs` exports these grouped structs

### 4. Schema Building
- Main `mod.rs` imports app-level Query/Mutation structs
- Use `MergedObject` to combine into final schema
- Clean break - no backwards compatibility needed

## Implementation Requirements

### 1. Code Changes (Minimal)
- Pure module restructuring - avoid changing resolver logic
- Keep current error handling patterns (`success: bool` + `errors: Vec<String>`)
- Keep current validation logic embedded in resolvers
- Maintain existing GraphQL schema structure

### 2. Type Organization
- Move all shared types to `types/` directory
- One file per type or input object
- Import types where needed in individual resolver files

### 3. Module Structure
Each app directory (`shared/`, `takenlijst/`, etc.) should have:
```rust
// mod.rs example for takenlijst/
use async_graphql::MergedObject;

mod createProject;
mod updateProject;
mod projects;
// ... other modules

#[derive(MergedObject, Default)]
pub struct TakenlijstMutation(
    createProject::CreateProjectMutation,
    updateProject::UpdateProjectMutation,
    // ... other mutations
);

#[derive(MergedObject, Default)]
pub struct TakenlijstQuery(
    projects::ProjectsQuery,
    // ... other queries
);
```

### 4. Individual Resolver Files
Each resolver file should:
- Define its own resolver struct (e.g., `LoginMutation`, `ProjectsQuery`)
- Import required types from `../types/`
- Export the resolver struct for use in app-level grouping
- Keep existing resolver logic unchanged

## Implementation Phases

### Phase 1: Types Extraction
1. Create `server/src/graphql/types/` directory
2. Extract all shared types and input objects to individual files
3. Update imports in existing files
4. Verify compilation

### Phase 2: Shared Module Restructure  
1. Create `server/src/graphql/shared/` directory
2. Extract authentication-related mutations/queries to individual files
3. Create `SharedMutation` and `SharedQuery` structs
4. Move related tests to `shared/tests/`
5. Update main `mod.rs` imports

### Phase 3: App-Specific Restructure
1. Restructure `takenlijst/` directory to individual files
2. Create `TakenlijstMutation` and `TakenlijstQuery` structs
3. Move related tests to `takenlijst/tests/`
4. Handle any other app directories similarly

### Phase 4: Main Module Update
1. Update main `mod.rs` to import and combine all app-level structs
2. Update schema building logic
3. Remove old file references
4. Final compilation and testing

### Phase 5: Integration Test Updates
1. Update imports in top-level integration tests
2. Verify all tests pass
3. Clean up any temporary files

## Success Criteria
- [x] Each mutation/query in its own file
- [x] Files named after the resolver function
- [x] App-specific organization by app ID
- [x] Shared functionality in flat `shared/` directory
- [x] Types organized in `types/` directory
- [x] Tests organized to match new structure
- [x] App-level Query/Mutation grouping maintained
- [x] Schema builds and functions identically
- [x] All existing tests pass with updated imports
- [x] Clean module structure with no circular dependencies

## Error Handling Strategy
- Maintain existing error patterns throughout restructure
- Any import errors should be resolved by updating file paths
- Compilation errors indicate missing exports or incorrect grouping
- Test failures indicate incorrect import paths or missing functionality

## Notes
- This is a pure refactor - no functional changes to GraphQL schema
- External clients are unaffected as GraphQL endpoint remains the same
- Focus on clean module organization and maintainability
- File-per-resolver approach improves code navigation and maintenance