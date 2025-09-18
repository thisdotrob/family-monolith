# Ticket 004: Eating Activities Backend Implementation

## Overview
Implement the backend infrastructure for eating activity tracking, following the same pattern established in ticket 002 for bathroom activities.

## Acceptance Criteria
- [ ] Database migration created for eating activities table
- [ ] GraphQL schema extended with eating activity types
- [ ] Mutations implemented for creating eating activities
- [ ] Queries implemented for retrieving eating activities
- [ ] All operations properly namespaced under `champTracker` GraphQL field
- [ ] User authentication and association working correctly

## Implementation Steps

### 1. Create Database Migration
Create new migration file: `server/migrations/YYYYMMDD_create_eating_activities.sql`
```sql
CREATE TABLE eating_activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    quantity_eaten TEXT NOT NULL,
    leftovers_thrown_away TEXT,
    food_type TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### 2. Add GraphQL Types
Extend `server/src/graphql/champ_tracker.rs`:
```rust
#[derive(InputObject)]
pub struct EatingActivityInput {
    pub timestamp: DateTime<Utc>,
    pub quantity_eaten: String,
    pub leftovers_thrown_away: Option<String>,
    pub food_type: String,
}

#[derive(SimpleObject)]
pub struct EatingActivity {
    pub id: i32,
    pub user_id: i32,
    pub timestamp: DateTime<Utc>,
    pub quantity_eaten: String,
    pub leftovers_thrown_away: Option<String>,
    pub food_type: String,
    pub created_at: DateTime<Utc>,
}
```

### 3. Implement Database Operations
Add to `server/src/db/helpers.rs`:
- `create_eating_activity(pool, user_id, input)`
- `get_eating_activities(pool, user_id, limit, offset)`
- `get_eating_activity_by_id(pool, id)`

### 4. Implement GraphQL Resolvers
Extend the existing ChampTrackerQueries and add mutation:
```rust
// In MutationRoot
async fn create_eating_activity(
    &self,
    ctx: &Context<'_>,
    input: EatingActivityInput,
) -> Result<EatingActivity> {
    // Implementation with user auth check
}

// Add to ChampTrackerQueries
async fn eating_activities(
    &self,
    ctx: &Context<'_>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<EatingActivity>> {
    // Implementation
}
```

### 5. Update GraphQL Schema Registration
Ensure new types are properly registered in the GraphQL schema and mutations are available.

## Files to Create/Modify
- `server/migrations/YYYYMMDD_create_eating_activities.sql`
- `server/src/graphql/champ_tracker.rs` (extend existing)
- `server/src/db/helpers.rs` (add eating activity functions)

## Testing Notes
- Test migration runs successfully
- Verify GraphQL schema compiles with new types
- Test mutation creates eating records correctly
- Test query returns proper eating data
- Confirm user association works
- Test with GraphQL playground/client

## Dependencies
- Completed ticket 002 (bathroom activities backend)
- Existing user authentication system
- SQLx for database operations
- async-graphql for GraphQL

## Estimated Effort
Small - Following established pattern from bathroom activities.