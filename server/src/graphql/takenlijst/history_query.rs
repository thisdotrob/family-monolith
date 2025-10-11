use crate::auth::Claims;
use crate::graphql::types::PagedTasks;
use crate::graphql::types::Task;
use crate::tasks::{TaskStatus, time_utils};
use async_graphql::{Context, Object};
use sqlx::{Row, SqlitePool};
use std::sync::Arc;

#[derive(Default)]
pub struct HistoryQuery;

#[Object]
impl HistoryQuery {
    async fn history(
        &self,
        ctx: &Context<'_>,
        statuses: Vec<TaskStatus>,
        timezone: String,
        project_id: Option<String>,
        tag_ids: Option<Vec<String>>,
        completer_id: Option<String>,
        from_date: Option<String>,
        to_date: Option<String>,
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
        let tz = time_utils::parse_timezone(&timezone).map_err(|e| async_graphql::Error::new(e))?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(username)
            .fetch_one(pool)
            .await?
            .0;

        // Calculate default date range if not provided (last 7 days)
        let (default_from, default_to) = {
            let (today, _) = time_utils::now_in_timezone(tz);
            let from = today
                .checked_sub_days(chrono::Days::new(7))
                .unwrap_or(today);
            let to = today;
            (
                from.format("%Y-%m-%d").to_string(),
                to.format("%Y-%m-%d").to_string(),
            )
        };

        let from_date = from_date.unwrap_or(default_from);
        let to_date = to_date.unwrap_or(default_to);

        // Build the base query
        let mut query_parts = vec![
            "SELECT DISTINCT t.id, t.project_id, t.author_id, t.assignee_id, t.series_id, t.title, t.description,".to_string(),
            "       t.status, t.scheduled_date, t.scheduled_time_minutes, t.deadline_date, t.deadline_time_minutes,".to_string(),
            "       t.completed_at, t.completed_by, t.abandoned_at, t.abandoned_by, t.created_at, t.updated_at".to_string(),
            "FROM tasks t".to_string(),
        ];

        let mut joins = Vec::new();
        let mut conditions = Vec::new();
        let mut bind_values: Vec<String> = Vec::new();

        // Status filter - expect done or abandoned
        let status_conditions: Vec<String> = statuses
            .iter()
            .map(|status| match status {
                TaskStatus::Done => "t.status = 'done'".to_string(),
                TaskStatus::Abandoned => "t.status = 'abandoned'".to_string(),
                TaskStatus::Todo => "t.status = 'todo'".to_string(), // Include but shouldn't be common for history
            })
            .collect();

        if !status_conditions.is_empty() {
            conditions.push(format!("({})", status_conditions.join(" OR ")));
        }

        // Project access check and filter
        if let Some(proj_id) = &project_id {
            // Check user has access to this project
            let has_access = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM projects p \
                 LEFT JOIN project_members pm ON p.id = pm.project_id \
                 WHERE p.id = ?1 AND (p.owner_id = ?2 OR pm.user_id = ?2)",
            )
            .bind(proj_id)
            .bind(&user_id)
            .fetch_one(pool)
            .await?
            .0;

            if has_access == 0 {
                return Err(async_graphql::Error::new(
                    "Project not found or access denied",
                ));
            }

            conditions.push(format!("t.project_id = ?{}", bind_values.len() + 1));
            bind_values.push(proj_id.clone());
        } else {
            // No specific project - show tasks from all projects user has access to
            joins.push("LEFT JOIN project_members pm ON t.project_id = pm.project_id".to_string());
            joins.push("LEFT JOIN projects p ON t.project_id = p.id".to_string());
            conditions.push(format!(
                "(p.owner_id = ?{} OR pm.user_id = ?{})",
                bind_values.len() + 1,
                bind_values.len() + 2
            ));
            bind_values.push(user_id.clone());
            bind_values.push(user_id.clone());
        }

        // Tag filter
        if let Some(tag_list) = &tag_ids {
            if !tag_list.is_empty() {
                joins.push("INNER JOIN task_tags tt ON t.id = tt.task_id".to_string());
                let tag_placeholders: Vec<String> = tag_list
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("?{}", bind_values.len() + i + 1))
                    .collect();
                conditions.push(format!("tt.tag_id IN ({})", tag_placeholders.join(", ")));
                bind_values.extend(tag_list.clone());
            }
        }

        // Completer filter
        if let Some(completer) = &completer_id {
            conditions.push(format!(
                "(t.completed_by = ?{} OR t.abandoned_by = ?{})",
                bind_values.len() + 1,
                bind_values.len() + 2
            ));
            bind_values.push(completer.clone());
            bind_values.push(completer.clone());
        }

        // Date range filter - use completed_at and abandoned_at dates
        conditions.push(format!(
            "((t.status = 'done' AND DATE(t.completed_at) >= ?{} AND DATE(t.completed_at) <= ?{}) OR \
             (t.status = 'abandoned' AND DATE(t.abandoned_at) >= ?{} AND DATE(t.abandoned_at) <= ?{}))",
            bind_values.len() + 1, bind_values.len() + 2, bind_values.len() + 3, bind_values.len() + 4
        ));
        bind_values.push(from_date.clone());
        bind_values.push(to_date.clone());
        bind_values.push(from_date.clone());
        bind_values.push(to_date.clone());

        // Build the full query
        if !joins.is_empty() {
            query_parts.extend(joins.clone());
        }

        if !conditions.is_empty() {
            query_parts.push(format!("WHERE {}", conditions.join(" AND ")));
        }

        // Order by completion/abandonment date DESC for day grouping (most recent first)
        query_parts.push(
            "ORDER BY COALESCE(t.completed_at, t.abandoned_at) DESC, t.created_at DESC".to_string(),
        );

        query_parts.push(format!("LIMIT {} OFFSET {}", limit, offset));

        let main_query = query_parts.join(" ");

        // Build count query
        let mut count_query_parts = vec!["SELECT COUNT(DISTINCT t.id) FROM tasks t".to_string()];
        if !joins.is_empty() {
            // Re-add joins for count query (excluding the INNER JOIN for tags if present)
            for join in &joins {
                if join.contains("LEFT JOIN") {
                    count_query_parts.push(join.clone());
                } else if join.contains("task_tags") {
                    // For tag filtering in count, we need to handle distinctness properly
                    count_query_parts.push(join.clone());
                }
            }
        }
        if !conditions.is_empty() {
            count_query_parts.push(format!("WHERE {}", conditions.join(" AND ")));
        }
        let count_query = count_query_parts.join(" ");

        // Execute count query
        let mut count_stmt = sqlx::query(&count_query);
        for value in &bind_values {
            count_stmt = count_stmt.bind(value);
        }
        let total_count = count_stmt.fetch_one(pool).await?.get::<i64, _>(0);

        // Execute main query
        let mut main_stmt = sqlx::query(&main_query);
        for value in &bind_values {
            main_stmt = main_stmt.bind(value);
        }
        let rows = main_stmt.fetch_all(pool).await?;

        // Convert rows to Task objects
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
