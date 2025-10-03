use sqlx::{Row, SqlitePool};
mod auth;
mod tests_recurring_series;

pub use crate::graphql::auth::{AuthenticatedMutation, RecurringSeries, UnauthenticatedMutation};
use async_graphql::{Context, EmptySubscription, Schema};
use async_graphql::{MergedObject, Object, SimpleObject};

use crate::auth::Claims;
use crate::tasks::{Task, TaskBucket, TaskStatus, time_utils};
use std::sync::Arc;

#[derive(SimpleObject)]
struct User {
    username: String,
    #[graphql(name = "firstName")]
    first_name: Option<String>,
}

#[derive(SimpleObject)]
struct Project {
    id: String,
    name: String,
    #[graphql(name = "ownerId")]
    owner_id: String,
    #[graphql(name = "archivedAt")]
    archived_at: Option<String>,
    #[graphql(name = "createdAt")]
    created_at: String,
    #[graphql(name = "updatedAt")]
    updated_at: String,
}

#[derive(SimpleObject)]
pub struct Tag {
    id: String,
    name: String,
    #[graphql(name = "createdAt")]
    created_at: String,
    #[graphql(name = "updatedAt")]
    updated_at: String,
}

#[derive(SimpleObject)]
pub struct PagedTasks {
    items: Vec<Task>,
    #[graphql(name = "totalCount")]
    total_count: i32,
}

// Define a query root
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        let username = &claims.sub;

        let user_data = sqlx::query_as::<_, (String, Option<String>)>(
            "SELECT username, first_name FROM users WHERE username = ?1",
        )
        .bind(username)
        .fetch_one(pool)
        .await?;

        Ok(User {
            username: user_data.0,
            first_name: user_data.1,
        })
    }

    async fn projects(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = false)] include_archived: bool,
        #[graphql(default = 0)] offset: i32,
        #[graphql(default = 50)] limit: i32,
    ) -> async_graphql::Result<Vec<Project>> {
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let username = &claims.sub;

        // First get the user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(username)
            .fetch_one(pool)
            .await?
            .0;

        // Build the query to get projects where user is owner or member
        let mut query = String::from(
            "SELECT DISTINCT p.id, p.name, p.owner_id, p.archived_at, p.created_at, p.updated_at 
             FROM projects p 
             LEFT JOIN project_members pm ON p.id = pm.project_id 
             WHERE (p.owner_id = ?1 OR pm.user_id = ?1)",
        );

        if !include_archived {
            query.push_str(" AND p.archived_at IS NULL");
        }

        query.push_str(" ORDER BY p.created_at DESC LIMIT ?2 OFFSET ?3");

        let projects =
            sqlx::query_as::<_, (String, String, String, Option<String>, String, String)>(&query)
                .bind(&user_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?;

        Ok(projects
            .into_iter()
            .map(
                |(id, name, owner_id, archived_at, created_at, updated_at)| Project {
                    id,
                    name,
                    owner_id,
                    archived_at,
                    created_at,
                    updated_at,
                },
            )
            .collect())
    }

    async fn tags(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] offset: i32,
        #[graphql(default = 200)] limit: i32,
    ) -> async_graphql::Result<Vec<Tag>> {
        // Require authentication
        let _claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        let tags = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, name, created_at, updated_at FROM tags ORDER BY name LIMIT ?1 OFFSET ?2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(tags
            .into_iter()
            .map(|(id, name, created_at, updated_at)| Tag {
                id,
                name,
                created_at,
                updated_at,
            })
            .collect())
    }

    async fn tasks(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        timezone: String,
        statuses: Option<Vec<TaskStatus>>,
        assignee: Option<String>,
        include_unassigned: Option<bool>,
        assigned_to_me: Option<bool>,
        tag_ids: Option<Vec<String>>,
        search: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> async_graphql::Result<PagedTasks> {
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let username = &claims.sub;

        // Parse timezone
        let tz = time_utils::parse_timezone(&timezone).map_err(|e| async_graphql::Error::new(e))?;

        // First get the user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(username)
            .fetch_one(pool)
            .await?
            .0;

        // Check if user has access to this project
        let has_access = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM projects p 
             LEFT JOIN project_members pm ON p.id = pm.project_id 
             WHERE p.id = ?1 AND (p.owner_id = ?2 OR pm.user_id = ?2)",
        )
        .bind(&project_id)
        .bind(&user_id)
        .fetch_one(pool)
        .await?
        .0;

        if has_access == 0 {
            return Err(async_graphql::Error::new(
                "Project not found or access denied",
            ));
        }

        // Start with a simple query for basic functionality
        let mut base_query = String::from(
            "SELECT t.id, t.project_id, t.author_id, t.assignee_id, t.series_id, t.title, t.description, 
                    t.status, t.scheduled_date, t.scheduled_time_minutes, t.deadline_date, t.deadline_time_minutes,
                    t.completed_at, t.completed_by, t.abandoned_at, t.abandoned_by, t.created_at, t.updated_at
             FROM tasks t WHERE t.project_id = ?1"
        );

        // Handle default values
        let statuses = statuses.unwrap_or_else(|| vec![TaskStatus::Todo]);
        let include_unassigned = include_unassigned.unwrap_or(false);
        let assigned_to_me = assigned_to_me.unwrap_or(false);
        let tag_ids = tag_ids.unwrap_or_default();
        let offset = offset.unwrap_or(0);
        let limit = limit.unwrap_or(20);

        // Add status filter for todo by default
        if statuses.len() == 1 && statuses[0] == TaskStatus::Todo {
            base_query.push_str(" AND t.status = 'todo'");
        }

        // Add ordering according to spec: Scheduled, then Deadline, then Created
        // NULL values go last in SQLite with ASC, so we need to handle this properly
        base_query.push_str(
            " ORDER BY 
            CASE WHEN t.scheduled_date IS NULL THEN 1 ELSE 0 END,
            t.scheduled_date ASC,
            t.scheduled_time_minutes ASC,
            CASE WHEN t.deadline_date IS NULL THEN 1 ELSE 0 END,
            t.deadline_date ASC,
            t.deadline_time_minutes ASC,
            t.created_at ASC",
        );

        base_query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

        // Count total items - simplified for now
        let total_count =
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tasks WHERE project_id = ?1")
                .bind(&project_id)
                .fetch_one(pool)
                .await?
                .0;

        // Execute the main query
        let rows = sqlx::query(&base_query)
            .bind(&project_id)
            .fetch_all(pool)
            .await?;

        let mut tasks = Vec::new();
        for row in rows {
            let id: String = row.get("id");
            let project_id: String = row.get("project_id");
            let author_id: String = row.get("author_id");
            let assignee_id: Option<String> = row.get("assignee_id");
            let series_id: Option<String> = row.get("series_id");
            let title: String = row.get("title");
            let description: Option<String> = row.get("description");
            let status_str: String = row.get("status");
            let scheduled_date: Option<String> = row.get("scheduled_date");
            let scheduled_time_minutes: Option<i32> = row.get("scheduled_time_minutes");
            let deadline_date: Option<String> = row.get("deadline_date");
            let deadline_time_minutes: Option<i32> = row.get("deadline_time_minutes");
            let completed_at: Option<String> = row.get("completed_at");
            let completed_by: Option<String> = row.get("completed_by");
            let abandoned_at: Option<String> = row.get("abandoned_at");
            let abandoned_by: Option<String> = row.get("abandoned_by");
            let created_at: String = row.get("created_at");
            let updated_at: String = row.get("updated_at");

            let status = match status_str.as_str() {
                "todo" => TaskStatus::Todo,
                "done" => TaskStatus::Done,
                "abandoned" => TaskStatus::Abandoned,
                _ => TaskStatus::Todo,
            };

            // Compute derived fields
            let is_overdue = time_utils::is_task_overdue(
                scheduled_date.as_deref(),
                scheduled_time_minutes,
                deadline_date.as_deref(),
                deadline_time_minutes,
                tz,
            );

            let bucket = time_utils::get_task_bucket(
                scheduled_date.as_deref(),
                scheduled_time_minutes,
                deadline_date.as_deref(),
                deadline_time_minutes,
                tz,
            );

            tasks.push(Task {
                id,
                project_id,
                author_id,
                assignee_id,
                series_id,
                title,
                description,
                status,
                scheduled_date,
                scheduled_time_minutes,
                deadline_date,
                deadline_time_minutes,
                completed_at,
                completed_by,
                abandoned_at,
                abandoned_by,
                created_at,
                updated_at,
                is_overdue,
                bucket,
            });
        }

        Ok(PagedTasks {
            items: tasks,
            total_count: total_count as i32,
        })
    }
}

#[derive(MergedObject, Default)]
pub struct CombinedMutation(UnauthenticatedMutation, AuthenticatedMutation);

pub type AppSchema = Schema<QueryRoot, CombinedMutation, EmptySubscription>;

pub fn build(pool: SqlitePool) -> AppSchema {
    Schema::build(QueryRoot, CombinedMutation::default(), EmptySubscription)
        .data(pool)
        .limit_depth(5)
        .limit_complexity(50)
        .disable_introspection()
        .finish()
}
