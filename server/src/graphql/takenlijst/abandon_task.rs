use std::sync::Arc;

use async_graphql::{Context, ErrorExtensions, Object};
use sqlx::{Row, SqlitePool};

use super::series_topup::top_up_series;
use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::error_codes::ErrorCode;
use crate::graphql::types::Task;
use crate::tasks::{TaskStatus, time_utils};

#[derive(Default)]
pub struct AbandonTaskMutation;

#[Object]
impl AbandonTaskMutation {
    async fn abandon_task(
        &self,
        ctx: &Context<'_>,
        id: String,
        last_known_updated_at: String,
        #[graphql(default = "UTC")] timezone: String,
    ) -> async_graphql::Result<Task> {
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let tz = time_utils::parse_timezone(&timezone).map_err(async_graphql::Error::new)?;

        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        let task_row =
            sqlx::query("SELECT id, project_id, updated_at, status FROM tasks WHERE id = ?1")
                .bind(&id)
                .fetch_one(pool)
                .await
                .map_err(|_| {
                    async_graphql::Error::new("Task not found")
                        .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()))
                })?;
        let project_id: String = task_row.get("project_id");
        let current_updated_at: String = task_row.get("updated_at");
        let status_str: String = task_row.get("status");

        require_member(pool, &user_id, &project_id).await?;

        let archived = sqlx::query_as::<_, (Option<String>,)>(
            "SELECT archived_at FROM projects WHERE id = ?1",
        )
        .bind(&project_id)
        .fetch_one(pool)
        .await?;
        if archived.0.is_some() {
            let error = async_graphql::Error::new("Project is archived; tasks are read-only")
                .extend_with(|_, e| e.set("code", ErrorCode::PermissionDenied.as_str()));
            return Err(error);
        }

        if current_updated_at != last_known_updated_at {
            let error = async_graphql::Error::new("Task has been modified by another user")
                .extend_with(|_, e| e.set("code", ErrorCode::ConflictStaleWrite.as_str()));
            return Err(error);
        }

        if status_str == "done" {
            let error = async_graphql::Error::new("Cannot abandon a completed task")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        sqlx::query("UPDATE tasks SET status = 'abandoned', abandoned_at = (strftime('%Y-%m-%d %H:%M:%f','now')), abandoned_by = ?1 WHERE id = ?2")
            .bind(&user_id)
            .bind(&id)
            .execute(pool)
            .await?;

        let row = sqlx::query("SELECT id, project_id, author_id, assignee_id, series_id, title, description, status, scheduled_date, scheduled_time_minutes, deadline_date, deadline_time_minutes, completed_at, completed_by, abandoned_at, abandoned_by, created_at, updated_at FROM tasks WHERE id = ?1")
            .bind(&id)
            .fetch_one(pool)
            .await?;

        // If this task belongs to a recurring series, attempt a top-up to ensure 5 future TODOs remain
        if let Some(series_id) = row.get::<Option<String>, _>("series_id") {
            let _ = top_up_series(pool, &series_id, &timezone).await;
        }

        let status: String = row.get("status");
        let status = match status.as_str() {
            "todo" => TaskStatus::Todo,
            "done" => TaskStatus::Done,
            "abandoned" => TaskStatus::Abandoned,
            _ => TaskStatus::Todo,
        };
        let scheduled_date: Option<String> = row.get("scheduled_date");
        let scheduled_time_minutes: Option<i32> = row.get("scheduled_time_minutes");
        let deadline_date: Option<String> = row.get("deadline_date");
        let deadline_time_minutes: Option<i32> = row.get("deadline_time_minutes");

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

        Ok(Task {
            id: row.get("id"),
            project_id: row.get("project_id"),
            author_id: row.get("author_id"),
            assignee_id: row.get("assignee_id"),
            series_id: row.get("series_id"),
            title: row.get("title"),
            description: row.get("description"),
            status,
            scheduled_date,
            scheduled_time_minutes,
            deadline_date,
            deadline_time_minutes,
            completed_at: row.get("completed_at"),
            completed_by: row.get("completed_by"),
            abandoned_at: row.get("abandoned_at"),
            abandoned_by: row.get("abandoned_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            is_overdue,
            bucket,
        })
    }
}
