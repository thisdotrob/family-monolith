# Ticket 002: Bathroom Activities Backend Implementation

## Overview
Implement the backend infrastructure for bathroom activity tracking, including database schema, GraphQL mutations/queries, and basic CRUD operations.

## Acceptance Criteria
- [ ] Database migration created for bathroom activities table
- [ ] GraphQL schema extended with bathroom activity types
- [ ] Mutations implemented for creating bathroom activities
- [ ] Queries implemented for retrieving bathroom activities
- [ ] All operations properly namespaced under `champTracker` GraphQL field
- [ ] User authentication and association working correctly

## Implementation Steps

### 1. Create Database Migration
Create new migration file: `server/migrations/YYYYMMDD_create_bathroom_activities.sql`
```sql
CREATE TABLE bathroom_activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    consistency TEXT,
    observations TEXT,
    litter_changed BOOLEAN NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### 2. Create GraphQL Types
Add to `server/src/graphql/` (create new file `champ_tracker.rs`):
```rust
// Input types
#[derive(InputObject)]
pub struct BathroomActivityInput {
    pub timestamp: DateTime<Utc>,
    pub consistency: Option<String>,
    pub observations: Option<String>,
    pub litter_changed: bool,
}

// Object types
#[derive(SimpleObject)]
pub struct BathroomActivity {
    pub id: i32,
    pub user_id: i32,
    pub timestamp: DateTime<Utc>,
    pub consistency: Option<String>,
    pub observations: Option<String>,
    pub litter_changed: bool,
    pub created_at: DateTime<Utc>,
}
```

### 3. Implement Database Operations
Create database helper functions for:
- `create_bathroom_activity(pool, user_id, input)`
- `get_bathroom_activities(pool, user_id, limit, offset)`
- `get_bathroom_activity_by_id(pool, id)`

### 4. Implement GraphQL Resolvers
Add to mutation and query resolvers:
```rust
// In MutationRoot
async fn create_bathroom_activity(
    &self,
    ctx: &Context<'_>,
    input: BathroomActivityInput,
) -> Result<BathroomActivity> {
    // Implementation with user auth check
}

// In QueryRoot under champTracker field
async fn bathroom_activities(
    &self,
    ctx: &Context<'_>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<BathroomActivity>> {
    // Implementation
}
```

### 5. Namespace Under champTracker
Create a nested GraphQL object for all champ tracker functionality:
```rust
#[derive(Default)]
pub struct ChampTrackerQueries;

#[Object]
impl ChampTrackerQueries {
    async fn bathroom_activities(&self, /* params */) -> Result<Vec<BathroomActivity>> {
        // Implementation
    }
}

// Add to main QueryRoot
async fn champ_tracker(&self) -> ChampTrackerQueries {
    ChampTrackerQueries::default()
}
```

## Files to Create/Modify
- `server/migrations/YYYYMMDD_create_bathroom_activities.sql`
- `server/src/graphql/champ_tracker.rs` (new file)
- `server/src/graphql/mod.rs` (add module reference)
- `server/src/db/helpers.rs` (add bathroom activity functions)

## Testing Notes
- Test migration runs successfully
- Verify GraphQL schema compiles
- Test mutation creates records correctly
- Test query returns proper data
- Confirm user association works
- Test with GraphQL playground/client

## Dependencies
- Existing user authentication system
- SQLx for database operations
- async-graphql for GraphQL

## Estimated Effort
Medium - Database and GraphQL setup with proper namespacing.