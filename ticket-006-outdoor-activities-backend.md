# Ticket 006: Outdoor Activities Backend Implementation

## Overview
Implement the backend infrastructure for outdoor activity tracking, following the established pattern from previous activity types.

## Acceptance Criteria
- [ ] Database migration created for outdoor activities table
- [ ] GraphQL schema extended with outdoor activity types
- [ ] Mutations implemented for creating outdoor activities
- [ ] Queries implemented for retrieving outdoor activities
- [ ] All operations properly namespaced under `champTracker` GraphQL field
- [ ] User authentication and association working correctly

## Implementation Steps

### 1. Create Database Migration
Create new migration file: `server/migrations/YYYYMMDD_create_outdoor_activities.sql`
```sql
CREATE TABLE outdoor_activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    start_timestamp DATETIME NOT NULL,
    duration_minutes INTEGER NOT NULL,
    activity_type TEXT NOT NULL,
    behavior TEXT,
    location TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### 2. Add GraphQL Types
Extend `server/src/graphql/champ_tracker.rs`:
```rust
#[derive(InputObject)]
pub struct OutdoorActivityInput {
    pub start_timestamp: DateTime<Utc>,
    pub duration_minutes: i32,
    pub activity_type: String,
    pub behavior: Option<String>,
    pub location: Option<String>,
}

#[derive(SimpleObject)]
pub struct OutdoorActivity {
    pub id: i32,
    pub user_id: i32,
    pub start_timestamp: DateTime<Utc>,
    pub duration_minutes: i32,
    pub activity_type: String,
    pub behavior: Option<String>,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

### 3. Implement Database Operations
Add to `server/src/db/helpers.rs`:
- `create_outdoor_activity(pool, user_id, input)`
- `get_outdoor_activities(pool, user_id, limit, offset)`
- `get_outdoor_activity_by_id(pool, id)`

### 4. Implement GraphQL Resolvers
Extend the existing ChampTrackerQueries and add mutation:
```rust
// In MutationRoot
async fn create_outdoor_activity(
    &self,
    ctx: &Context<'_>,
    input: OutdoorActivityInput,
) -> Result<OutdoorActivity> {
    // Implementation with user auth check
}

// Add to ChampTrackerQueries
async fn outdoor_activities(
    &self,
    ctx: &Context<'_>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<OutdoorActivity>> {
    // Implementation
}
```

### 5. Update GraphQL Schema Registration
Ensure new types are properly registered in the GraphQL schema and mutations are available.

## Files to Create/Modify
- `server/migrations/YYYYMMDD_create_outdoor_activities.sql`
- `server/src/graphql/champ_tracker.rs` (extend existing)
- `server/src/db/helpers.rs` (add outdoor activity functions)

## Testing Notes
- Test migration runs successfully
- Verify GraphQL schema compiles with new types
- Test mutation creates outdoor records correctly
- Test query returns proper outdoor data
- Confirm user association works
- Test with GraphQL playground/client

## Dependencies
- Completed tickets 002 and 004 (established backend pattern)
- Existing user authentication system
- SQLx for database operations
- async-graphql for GraphQL

## Estimated Effort
Small - Following established pattern from previous activities.