# Ticket 014: Daily Highlights Backend Implementation

## Overview
Implement the backend infrastructure for daily highlights tracking, including support for tags and quick notable moment captures.

## Acceptance Criteria
- [ ] Database migration created for daily highlights table
- [ ] GraphQL schema extended with daily highlight types
- [ ] Mutations implemented for creating daily highlights
- [ ] Queries implemented for retrieving daily highlights
- [ ] Tags stored and retrieved as JSON array
- [ ] All operations properly namespaced under `champTracker` GraphQL field
- [ ] User authentication and association working correctly

## Implementation Steps

### 1. Create Database Migration
Create new migration file: `server/migrations/YYYYMMDD_create_daily_highlights.sql`
```sql
CREATE TABLE daily_highlights (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    content TEXT,
    tags TEXT, -- JSON array of strings
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### 2. Add GraphQL Types
Extend `server/src/graphql/champ_tracker.rs`:
```rust
#[derive(InputObject)]
pub struct DailyHighlightInput {
    pub timestamp: DateTime<Utc>,
    pub content: Option<String>,
    pub tags: Vec<String>,
}

#[derive(SimpleObject)]
pub struct DailyHighlight {
    pub id: i32,
    pub user_id: i32,
    pub timestamp: DateTime<Utc>,
    pub content: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}
```

### 3. Implement Database Operations
Add to `server/src/db/helpers.rs`:
- `create_daily_highlight(pool, user_id, input)` - serialize tags to JSON
- `get_daily_highlights(pool, user_id, limit, offset)` - deserialize tags from JSON
- `get_daily_highlight_by_id(pool, id)` - deserialize tags from JSON

### 4. Implement GraphQL Resolvers
Extend the existing ChampTrackerQueries and add mutation:
```rust
// In MutationRoot
async fn create_daily_highlight(
    &self,
    ctx: &Context<'_>,
    input: DailyHighlightInput,
) -> Result<DailyHighlight> {
    // Implementation with user auth check
    // Serialize tags to JSON before storing
}

// Add to ChampTrackerQueries
async fn daily_highlights(
    &self,
    ctx: &Context<'_>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<DailyHighlight>> {
    // Implementation
    // Deserialize tags from JSON when retrieving
}
```

### 5. Handle JSON Serialization
Add helper functions for tags:
- Serialize `Vec<String>` to JSON string for database storage
- Deserialize JSON string to `Vec<String>` for GraphQL response
- Handle empty tags array gracefully

### 6. Update GraphQL Schema Registration
Ensure new types are properly registered in the GraphQL schema and mutations are available.

## Files to Create/Modify
- `server/migrations/YYYYMMDD_create_daily_highlights.sql`
- `server/src/graphql/champ_tracker.rs` (extend existing)
- `server/src/db/helpers.rs` (add daily highlight functions)

## Testing Notes
- Test migration runs successfully
- Verify GraphQL schema compiles with new types
- Test mutation creates daily highlight records correctly
- Test query returns proper daily highlight data with tags
- Test tags serialization/deserialization works correctly
- Test with empty tags array
- Test with multiple tags (e.g. ["funny", "milestone"])
- Confirm user association works

## Dependencies
- Completed tickets 002, 004, 006, 008, 010, 012 (established backend pattern)
- Existing user authentication system
- SQLx for database operations
- async-graphql for GraphQL
- serde_json for JSON serialization

## Estimated Effort
Small - Following established pattern with minor JSON handling complexity.