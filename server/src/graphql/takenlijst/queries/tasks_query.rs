use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::graphql::takenlijst::types::PagedTasks;
use crate::graphql::takenlijst::types::Task;
use crate::tasks::{TaskStatus, time_utils};
use async_graphql::{Context, Object};
use sqlx::{Row, SqlitePool};
use std::sync::Arc;

#[derive(Default)]
pub struct TasksQuery;

#[Object]
impl TasksQuery {
    async fn tasks(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        timezone: String,
        #[graphql(default_with = "vec![TaskStatus::Todo]")] statuses: Vec<TaskStatus>,
        assignee: Option<String>,
        #[graphql(default = false)] include_unassigned: bool,
        #[graphql(default = false)] assigned_to_me: bool,
        tag_ids: Option<Vec<String>>,
        search: Option<String>,
        #[graphql(default = 0)] offset: i32,
        #[graphql(default = 20)] limit: i32,
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
        let tz = time_utils::parse_timezone(&timezone).map_err(async_graphql::Error::new)?;

        // First get the user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(username)
            .fetch_one(pool)
            .await?
            .0;

        // Check if user has access to this project
        require_member(pool, &user_id, &project_id).await?;

        // Build the base query with conditions
        let mut where_conditions = vec!["t.project_id = ?".to_string()];
        let mut join_clause = String::new();

        // Add join for tag filtering if needed
        let need_tag_join = tag_ids.as_ref().is_some_and(|tags| !tags.is_empty());
        if need_tag_join {
            join_clause = " LEFT JOIN task_tags tt ON t.id = tt.task_id".to_string();
        }

        // Add status filtering
        if !statuses.is_empty() {
            if statuses.len() == 1 {
                where_conditions.push("t.status = ?".to_string());
            } else {
                let placeholders = vec!["?"; statuses.len()].join(",");
                where_conditions.push(format!("t.status IN ({})", placeholders));
            }
        }

        // Add assignee filtering
        if assignee.is_some() {
            where_conditions.push("t.assignee_id = ?".to_string());
        } else if include_unassigned {
            where_conditions.push("t.assignee_id IS NULL".to_string());
        } else if assigned_to_me {
            where_conditions.push("t.assignee_id = ?".to_string());
        }

        // Add tag filtering
        if let Some(tags) = &tag_ids {
            if !tags.is_empty() {
                let placeholders = vec!["?"; tags.len()].join(",");
                where_conditions.push(format!("tt.tag_id IN ({})", placeholders));
            }
        }

        // Add search filtering
        if let Some(search_term) = &search {
            if !search_term.trim().is_empty() {
                where_conditions.push("(t.title LIKE ? OR t.description LIKE ?)".to_string());
            }
        }

        let where_clause = format!(" WHERE {}", where_conditions.join(" AND "));

        let base_query = format!(
            "SELECT DISTINCT t.id, t.project_id, t.author_id, t.assignee_id, t.series_id, t.title, t.description, \
                    t.status, t.scheduled_date, t.scheduled_time_minutes, t.deadline_date, t.deadline_time_minutes,\n                    t.completed_at, t.completed_by, t.abandoned_at, t.abandoned_by, t.created_at, t.updated_at\n             FROM tasks t{}{}\n             ORDER BY \n                CASE WHEN t.scheduled_date IS NULL AND t.deadline_date IS NULL THEN t.title ELSE '' END ASC,\n                CASE WHEN t.scheduled_date IS NULL THEN 1 ELSE 0 END,\n                t.scheduled_date ASC,\n                CASE WHEN t.scheduled_time_minutes IS NULL THEN 1 ELSE 0 END,\n                t.scheduled_time_minutes ASC,\n                CASE WHEN t.deadline_date IS NULL THEN 1 ELSE 0 END,\n                t.deadline_date ASC,\n                CASE WHEN t.deadline_time_minutes IS NULL THEN 1 ELSE 0 END,\n                t.deadline_time_minutes ASC,\n                t.created_at ASC\n             LIMIT {} OFFSET {}",
            join_clause, where_clause, limit, offset
        );

        let count_query = format!(
            "SELECT COUNT(DISTINCT t.id) FROM tasks t{}{}",
            join_clause, where_clause
        );

        // Build the actual queries with parameter binding
        let mut count_stmt = sqlx::query_as::<_, (i64,)>(&count_query).bind(&project_id);
        let mut main_stmt = sqlx::query(&base_query).bind(&project_id);

        // Bind status parameters
        for status in &statuses {
            let status_str = match status {
                TaskStatus::Todo => "todo",
                TaskStatus::Done => "done",
                TaskStatus::Abandoned => "abandoned",
            };
            count_stmt = count_stmt.bind(status_str);
            main_stmt = main_stmt.bind(status_str);
        }

        // Bind assignee parameters
        if let Some(assignee_id) = &assignee {
            count_stmt = count_stmt.bind(assignee_id);
            main_stmt = main_stmt.bind(assignee_id);
        } else if assigned_to_me {
            count_stmt = count_stmt.bind(&user_id);
            main_stmt = main_stmt.bind(&user_id);
        }

        // Bind tag parameters
        if let Some(tags) = &tag_ids {
            for tag_id in tags {
                count_stmt = count_stmt.bind(tag_id);
                main_stmt = main_stmt.bind(tag_id);
            }
        }

        // Bind search parameters
        if let Some(search_term) = &search {
            if !search_term.trim().is_empty() {
                let search_pattern = format!("%{}%", search_term.trim());
                count_stmt = count_stmt.bind(search_pattern.clone());
                main_stmt = main_stmt.bind(search_pattern.clone());
                count_stmt = count_stmt.bind(search_pattern.clone());
                main_stmt = main_stmt.bind(search_pattern);
            }
        }

        // Execute queries
        let total_count = count_stmt.fetch_one(pool).await?.0;
        let rows = main_stmt.fetch_all(pool).await?;

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
