# Ticket 012: Play Activities Backend Implementation

## Overview
Implement the backend infrastructure for play activity tracking, following the established pattern from previous activity types.

## Acceptance Criteria
- [ ] Database migration created for play activities table
- [ ] GraphQL schema extended with play activity types
- [ ] Mutations implemented for creating play activities
- [ ] Queries implemented for retrieving play activities
- [ ] All operations properly namespaced under `champTracker` GraphQL field
- [ ] User authentication and association working correctly

## Implementation Steps

### 1. Create Database Migration
Create new migration file: `server/migrations/YYYYMMDD_create_play_activities.sql`
```sql
CREATE TABLE play_activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    start_time DATETIME NOT NULL,
    duration_minutes INTEGER NOT NULL,
    play_type TEXT NOT NULL,
    participants TEXT NOT NULL,
    location TEXT,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### 2. Add GraphQL Types
Extend `server/src/graphql/champ_tracker.rs`:
```rust
#[derive(InputObject)]
pub struct PlayActivityInput {
    pub start_time: DateTime<Utc>,
    pub duration_minutes: i32,
    pub play_type: String,
    pub participants: String,
    pub location: Option<String>,
    pub notes: Option<String>,
}

#[derive(SimpleObject)]
pub struct PlayActivity {
    pub id: i32,
    pub user_id: i32,
    pub start_time: DateTime<Utc>,
    pub duration_minutes: i32,
    pub play_type: String,
    pub participants: String,
    pub location: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

### 3. Implement Database Operations
Add to `server/src/db/helpers.rs`:
- `create_play_activity(pool, user_id, input)`
- `get_play_activities(pool, user_id, limit, offset)`
- `get_play_activity_by_id(pool, id)`

### 4. Implement GraphQL Resolvers
Extend the existing ChampTrackerQueries and add mutation:
```rust
// In MutationRoot
async fn create_play_activity(
    &self,
    ctx: &Context<'_>,
    input: PlayActivityInput,
) -> Result<PlayActivity> {
    // Implementation with user auth check
}

// Add to ChampTrackerQueries
async fn play_activities(
    &self,
    ctx: &Context<'_>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<PlayActivity>> {
    // Implementation
}
```

### 5. Update GraphQL Schema Registration
Ensure new types are properly registered in the GraphQL schema and mutations are available.

## Files to Create/Modify
- `server/migrations/YYYYMMDD_create_play_activities.sql`
- `server/src/graphql/champ_tracker.rs` (extend existing)
- `server/src/db/helpers.rs` (add play activity functions)

## Testing Notes
- Test migration runs successfully
- Verify GraphQL schema compiles with new types
- Test mutation creates play activity records correctly
- Test query returns proper play activity data
- Confirm user association works
- Test with GraphQL playground/client
- Test duration field handles positive integers

## Dependencies
- Completed tickets 002, 004, 006, 008, 010 (established backend pattern)
- Existing user authentication system
- SQLx for database operations
- async-graphql for GraphQL

## Estimated Effort
Small - Following established pattern from previous activities.