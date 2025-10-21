use std::sync::Arc;

use async_graphql::{Context, ErrorExtensions, Object};
use sqlx::{Row, SqlitePool};

use crate::auth::Claims;
use crate::auth::guard::{is_member, require_member};
use crate::error_codes::ErrorCode;
use crate::graphql::takenlijst::types::CreateTaskInput;
use crate::graphql::takenlijst::types::Task;
use crate::tasks::{TaskStatus, time_utils};

#[derive(Default)]
pub struct CreateTaskMutation;

#[Object]
impl CreateTaskMutation {
    async fn create_task(
        &self,
        ctx: &Context<'_>,
        input: CreateTaskInput,
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

        // Get current user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Check membership
        require_member(pool, &user_id, &input.project_id).await?;

        // Enforce read-only for archived projects
        let archived = sqlx::query_as::<_, (Option<String>,)>(
            "SELECT archived_at FROM projects WHERE id = ?1",
        )
        .bind(&input.project_id)
        .fetch_one(pool)
        .await?;
        if archived.0.is_some() {
            let error = async_graphql::Error::new("Project is archived; tasks are read-only")
                .extend_with(|_, e| e.set("code", ErrorCode::PermissionDenied.as_str()));
            return Err(error);
        }

        // Validation
        let title_trim = input.title.trim().to_string();
        if title_trim.is_empty() || title_trim.len() > 120 {
            let error =
                async_graphql::Error::new("Title is required and must be <= 120 characters")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }
        if let Some(desc) = &input.description {
            if desc.len() > 5000 {
                let error = async_graphql::Error::new("Description must be <= 5000 characters")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }
        if input.scheduled_time_minutes.is_some() && input.scheduled_date.is_none() {
            let error = async_graphql::Error::new("scheduledTimeMinutes requires scheduledDate")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }
        if input.deadline_time_minutes.is_some() && input.deadline_date.is_none() {
            let error = async_graphql::Error::new("deadlineTimeMinutes requires deadlineDate")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }
        if let Some(m) = input.scheduled_time_minutes {
            if m < 0 || m > 1439 {
                let error = async_graphql::Error::new("scheduledTimeMinutes out of range")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }
        if let Some(m) = input.deadline_time_minutes {
            if m < 0 || m > 1439 {
                let error = async_graphql::Error::new("deadlineTimeMinutes out of range")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }
        if let Some(date) = &input.scheduled_date {
            if chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_err() {
                let error = async_graphql::Error::new("Invalid scheduledDate format")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }
        if let Some(date) = &input.deadline_date {
            if chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_err() {
                let error = async_graphql::Error::new("Invalid deadlineDate format")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }

        // Validate assignee is project member if provided
        if let Some(assignee_id) = &input.assignee_id {
            let is_mem = is_member(pool, assignee_id, &input.project_id).await?;
            if !is_mem {
                let error = async_graphql::Error::new("Assignee must be a project member")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }

        // Validate tag IDs exist if provided
        if let Some(tag_ids) = &input.tag_ids {
            if !tag_ids.is_empty() {
                let placeholders = vec!["?"; tag_ids.len()].join(",");
                let sql = format!("SELECT COUNT(*) FROM tags WHERE id IN ({})", placeholders);
                let mut q = sqlx::query_as::<_, (i64,)>(&sql);
                for id in tag_ids {
                    q = q.bind(id);
                }
                let count = q.fetch_one(pool).await?.0;
                if count != tag_ids.len() as i64 {
                    let error = async_graphql::Error::new("One or more tags not found")
                        .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                    return Err(error);
                }
            }
        }

        // Insert task
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO tasks (id, project_id, author_id, assignee_id, title, description, status, scheduled_date, scheduled_time_minutes, deadline_date, deadline_time_minutes) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'todo', ?7, ?8, ?9, ?10)")
            .bind(&id)
            .bind(&input.project_id)
            .bind(&user_id)
            .bind(&input.assignee_id)
            .bind(&title_trim)
            .bind(&input.description)
            .bind(&input.scheduled_date)
            .bind(&input.scheduled_time_minutes)
            .bind(&input.deadline_date)
            .bind(&input.deadline_time_minutes)
            .execute(pool)
            .await?;

        // Insert tags mapping
        if let Some(tag_ids) = &input.tag_ids {
            for tag_id in tag_ids {
                sqlx::query("INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)")
                    .bind(&id)
                    .bind(tag_id)
                    .execute(pool)
                    .await?;
            }
        }

        // Fetch created
        let row = sqlx::query("SELECT id, project_id, author_id, assignee_id, series_id, title, description, status, scheduled_date, scheduled_time_minutes, deadline_date, deadline_time_minutes, completed_at, completed_by, abandoned_at, abandoned_by, created_at, updated_at FROM tasks WHERE id = ?1")
            .bind(&id)
            .fetch_one(pool)
            .await?;

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
