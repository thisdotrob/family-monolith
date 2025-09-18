# Ticket 008: Vet Visits Backend Implementation

## Overview
Implement the backend infrastructure for vet visit tracking, following the established pattern from previous activity types.

## Acceptance Criteria
- [ ] Database migration created for vet visits table
- [ ] GraphQL schema extended with vet visit types
- [ ] Mutations implemented for creating vet visits
- [ ] Queries implemented for retrieving vet visits
- [ ] All operations properly namespaced under `champTracker` GraphQL field
- [ ] User authentication and association working correctly

## Implementation Steps

### 1. Create Database Migration
Create new migration file: `server/migrations/YYYYMMDD_create_vet_visits.sql`
```sql
CREATE TABLE vet_visits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    date_time DATETIME NOT NULL,
    reason TEXT NOT NULL,
    weight_kg REAL,
    treatments_procedures TEXT,
    cost_amount REAL,
    cost_currency TEXT DEFAULT 'USD',
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### 2. Add GraphQL Types
Extend `server/src/graphql/champ_tracker.rs`:
```rust
#[derive(InputObject)]
pub struct VetVisitInput {
    pub date_time: DateTime<Utc>,
    pub reason: String,
    pub weight_kg: Option<f64>,
    pub treatments_procedures: Option<String>,
    pub cost_amount: Option<f64>,
    pub cost_currency: Option<String>,
    pub notes: Option<String>,
}

#[derive(SimpleObject)]
pub struct VetVisit {
    pub id: i32,
    pub user_id: i32,
    pub date_time: DateTime<Utc>,
    pub reason: String,
    pub weight_kg: Option<f64>,
    pub treatments_procedures: Option<String>,
    pub cost_amount: Option<f64>,
    pub cost_currency: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

### 3. Implement Database Operations
Add to `server/src/db/helpers.rs`:
- `create_vet_visit(pool, user_id, input)`
- `get_vet_visits(pool, user_id, limit, offset)`
- `get_vet_visit_by_id(pool, id)`

### 4. Implement GraphQL Resolvers
Extend the existing ChampTrackerQueries and add mutation:
```rust
// In MutationRoot
async fn create_vet_visit(
    &self,
    ctx: &Context<'_>,
    input: VetVisitInput,
) -> Result<VetVisit> {
    // Implementation with user auth check
}

// Add to ChampTrackerQueries
async fn vet_visits(
    &self,
    ctx: &Context<'_>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<VetVisit>> {
    // Implementation
}
```

### 5. Update GraphQL Schema Registration
Ensure new types are properly registered in the GraphQL schema and mutations are available.

## Files to Create/Modify
- `server/migrations/YYYYMMDD_create_vet_visits.sql`
- `server/src/graphql/champ_tracker.rs` (extend existing)
- `server/src/db/helpers.rs` (add vet visit functions)

## Testing Notes
- Test migration runs successfully
- Verify GraphQL schema compiles with new types
- Test mutation creates vet visit records correctly
- Test query returns proper vet visit data
- Confirm user association works
- Test with GraphQL playground/client
- Test cost and weight fields handle decimal values properly

## Dependencies
- Completed tickets 002, 004, 006 (established backend pattern)
- Existing user authentication system
- SQLx for database operations
- async-graphql for GraphQL

## Estimated Effort
Small - Following established pattern from previous activities.