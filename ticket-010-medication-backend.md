# Ticket 010: Medication Tracking Backend Implementation

## Overview
Implement the backend infrastructure for medication tracking, including both medication records and individual dose logging. This is more complex than previous activities due to the two-table relationship.

## Acceptance Criteria
- [ ] Database migrations created for medications and medication_doses tables
- [ ] GraphQL schema extended with medication and dose types
- [ ] Mutations implemented for creating medications and logging doses
- [ ] Queries implemented for retrieving medications and doses
- [ ] All operations properly namespaced under `champTracker` GraphQL field
- [ ] User authentication and association working correctly
- [ ] Proper relationship between medications and doses

## Implementation Steps

### 1. Create Database Migrations
Create new migration file: `server/migrations/YYYYMMDD_create_medications.sql`
```sql
CREATE TABLE medications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    dosage TEXT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    reason TEXT NOT NULL,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE medication_doses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    medication_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    dose_timestamp DATETIME NOT NULL,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (medication_id) REFERENCES medications(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### 2. Add GraphQL Types
Extend `server/src/graphql/champ_tracker.rs`:
```rust
#[derive(InputObject)]
pub struct MedicationInput {
    pub name: String,
    pub dosage: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub reason: String,
    pub notes: Option<String>,
}

#[derive(InputObject)]
pub struct MedicationDoseInput {
    pub medication_id: i32,
    pub dose_timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(SimpleObject)]
pub struct Medication {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub dosage: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub reason: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(SimpleObject)]
pub struct MedicationDose {
    pub id: i32,
    pub medication_id: i32,
    pub user_id: i32,
    pub dose_timestamp: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

### 3. Implement Database Operations
Add to `server/src/db/helpers.rs`:
- `create_medication(pool, user_id, input)`
- `get_medications(pool, user_id, active_only)`
- `get_medication_by_id(pool, id)`
- `create_medication_dose(pool, user_id, input)`
- `get_medication_doses(pool, user_id, medication_id, limit, offset)`
- `get_doses_for_medication(pool, medication_id)`

### 4. Implement GraphQL Resolvers
Extend the existing ChampTrackerQueries and add mutations:
```rust
// In MutationRoot
async fn create_medication(
    &self,
    ctx: &Context<'_>,
    input: MedicationInput,
) -> Result<Medication> {
    // Implementation with user auth check
}

async fn log_medication_dose(
    &self,
    ctx: &Context<'_>,
    input: MedicationDoseInput,
) -> Result<MedicationDose> {
    // Implementation with user auth check and medication ownership verification
}

// Add to ChampTrackerQueries
async fn medications(
    &self,
    ctx: &Context<'_>,
    active_only: Option<bool>,
) -> Result<Vec<Medication>> {
    // Implementation
}

async fn medication_doses(
    &self,
    ctx: &Context<'_>,
    medication_id: Option<i32>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<MedicationDose>> {
    // Implementation
}
```

### 5. Update GraphQL Schema Registration
Ensure new types are properly registered in the GraphQL schema and mutations are available.

## Files to Create/Modify
- `server/migrations/YYYYMMDD_create_medications.sql`
- `server/src/graphql/champ_tracker.rs` (extend existing)
- `server/src/db/helpers.rs` (add medication functions)

## Testing Notes
- Test migrations run successfully
- Verify GraphQL schema compiles with new types
- Test medication creation works correctly
- Test dose logging with valid medication IDs
- Test dose logging fails with invalid medication IDs
- Confirm user association works for both tables
- Test active_only filter for medications
- Test medication_id filter for doses

## Dependencies
- Completed tickets 002, 004, 006, 008 (established backend pattern)
- Existing user authentication system
- SQLx for database operations
- async-graphql for GraphQL

## Estimated Effort
Medium - More complex due to two-table relationship and foreign key constraints.