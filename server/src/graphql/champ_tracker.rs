use crate::auth::Claims;
use async_graphql::{Context, InputObject, Object, SimpleObject, Result};
use chrono::{DateTime, Utc};
use sqlx::{SqlitePool, Row};
use std::sync::Arc;

// Use the async-graphql chrono scalar for DateTime
type DateTimeUtc = DateTime<Utc>;

// Bathroom Activity Types
#[derive(InputObject)]
pub struct BathroomActivityInput {
    pub timestamp: DateTimeUtc,
    pub consistency: Option<String>,
    pub observations: Option<String>,
    pub litter_changed: bool,
}

#[derive(SimpleObject)]
pub struct BathroomActivity {
    pub id: i32,
    pub user_id: String,
    pub timestamp: DateTimeUtc,
    pub consistency: Option<String>,
    pub observations: Option<String>,
    pub litter_changed: bool,
    pub created_at: DateTimeUtc,
}

// Database helpers for bathroom activities
pub async fn create_bathroom_activity(
    pool: &SqlitePool,
    user_id: &str,
    input: BathroomActivityInput,
) -> Result<BathroomActivity> {
    // First insert the record
    let result = sqlx::query(
        r#"
        INSERT INTO bathroom_activities (user_id, timestamp, consistency, observations, litter_changed)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#
    )
    .bind(user_id)
    .bind(input.timestamp)
    .bind(&input.consistency)
    .bind(&input.observations)
    .bind(input.litter_changed)
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid();

    // Then fetch the created record
    let row = sqlx::query(
        r#"
        SELECT id, user_id, timestamp, consistency, observations, litter_changed, created_at
        FROM bathroom_activities WHERE id = ?1
        "#
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(BathroomActivity {
        id: row.get::<i64, _>("id") as i32,
        user_id: row.get("user_id"),
        timestamp: row.get("timestamp"),
        consistency: row.get("consistency"),
        observations: row.get("observations"),
        litter_changed: row.get("litter_changed"),
        created_at: row.get("created_at"),
    })
}

pub async fn get_bathroom_activities(
    pool: &SqlitePool,
    user_id: Option<&str>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<BathroomActivity>> {
    let limit = limit.unwrap_or(50).min(100); // Max 100 items
    let offset = offset.unwrap_or(0);

    let rows = if let Some(user_id) = user_id {
        sqlx::query(
            r#"
            SELECT id, user_id, timestamp, consistency, observations, litter_changed, created_at
            FROM bathroom_activities 
            WHERE user_id = ?1
            ORDER BY timestamp DESC 
            LIMIT ?2 OFFSET ?3
            "#
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT id, user_id, timestamp, consistency, observations, litter_changed, created_at
            FROM bathroom_activities 
            ORDER BY timestamp DESC 
            LIMIT ?1 OFFSET ?2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?
    };

    let activities = rows
        .into_iter()
        .map(|row| BathroomActivity {
            id: row.get::<i64, _>("id") as i32,
            user_id: row.get("user_id"),
            timestamp: row.get("timestamp"),
            consistency: row.get("consistency"),
            observations: row.get("observations"),
            litter_changed: row.get("litter_changed"),
            created_at: row.get("created_at"),
        })
        .collect();

    Ok(activities)
}

// ChampTracker queries namespace
#[derive(Default)]
pub struct ChampTrackerQueries;

#[Object]
impl ChampTrackerQueries {
    async fn bathroom_activities(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<BathroomActivity>> {
        let pool = ctx.data::<SqlitePool>()?;
        
        // For now, return all activities (multi-user shared data as specified)
        // Could filter by user if needed in the future
        get_bathroom_activities(pool, None, limit, offset).await
    }
}

// ChampTracker mutations
#[derive(Default)]
pub struct ChampTrackerMutations;

#[Object]
impl ChampTrackerMutations {
    async fn create_bathroom_activity(
        &self,
        ctx: &Context<'_>,
        input: BathroomActivityInput,
    ) -> Result<BathroomActivity> {
        // Get authenticated user
        let claims = ctx.data_opt::<Arc<Claims>>()
            .ok_or_else(|| async_graphql::Error::new("Authentication required"))?;
        
        let pool = ctx.data::<SqlitePool>()?;
        
        // Get user ID from database using username from claims
        let user_result = sqlx::query_as::<_, (String,)>(
            "SELECT id FROM users WHERE username = ?1"
        )
        .bind(&claims.sub)
        .fetch_one(pool)
        .await?;
        
        create_bathroom_activity(pool, &user_result.0, input).await
    }
}