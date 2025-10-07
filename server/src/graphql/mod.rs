use sqlx::{Row, SqlitePool};
mod auth;
mod takenlijst;
mod tests_history;
mod tests_integration;
mod tests_recurring_series;
mod tests_saved_views;
mod unauthenticated;

pub use crate::graphql::auth::AuthenticatedMutation;
pub use crate::graphql::unauthenticated::UnauthenticatedMutation;
use async_graphql::{Context, EmptySubscription, Schema};
use async_graphql::{InputObject, MergedObject, Object, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::tasks::{Task, TaskStatus, time_utils};
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

#[derive(SimpleObject)]
pub struct SavedView {
    id: String,
    #[graphql(name = "projectId")]
    project_id: String,
    name: String,
    filters: SavedViewFilters,
    #[graphql(name = "createdBy")]
    created_by: String,
    #[graphql(name = "createdAt")]
    created_at: String,
    #[graphql(name = "updatedAt")]
    updated_at: String,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct SavedViewFilters {
    statuses: Vec<TaskStatus>,
    assignee: Option<String>,
    #[graphql(name = "includeUnassigned")]
    #[serde(rename = "includeUnassigned")]
    include_unassigned: bool,
    #[graphql(name = "assignedToMe")]
    #[serde(rename = "assignedToMe")]
    assigned_to_me: bool,
    #[graphql(name = "tagIds")]
    #[serde(rename = "tagIds")]
    tag_ids: Vec<String>,
}

#[derive(InputObject)]
pub struct SavedViewFiltersInput {
    statuses: Vec<TaskStatus>,
    assignee: Option<String>,
    #[graphql(name = "includeUnassigned")]
    include_unassigned: bool,
    #[graphql(name = "assignedToMe")]
    assigned_to_me: bool,
    #[graphql(name = "tagIds")]
    tag_ids: Vec<String>,
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
            "SELECT DISTINCT t.id, t.project_id, t.author_id, t.assignee_id, t.series_id, t.title, t.description, 
                    t.status, t.scheduled_date, t.scheduled_time_minutes, t.deadline_date, t.deadline_time_minutes,
                    t.completed_at, t.completed_by, t.abandoned_at, t.abandoned_by, t.created_at, t.updated_at
             FROM tasks t{}{}
             ORDER BY 
                CASE WHEN t.scheduled_date IS NULL AND t.deadline_date IS NULL THEN t.title ELSE '' END ASC,
                CASE WHEN t.scheduled_date IS NULL THEN 1 ELSE 0 END,
                t.scheduled_date ASC,
                CASE WHEN t.scheduled_time_minutes IS NULL THEN 1 ELSE 0 END,
                t.scheduled_time_minutes ASC,
                CASE WHEN t.deadline_date IS NULL THEN 1 ELSE 0 END,
                t.deadline_date ASC,
                CASE WHEN t.deadline_time_minutes IS NULL THEN 1 ELSE 0 END,
                t.deadline_time_minutes ASC,
                t.created_at ASC
             LIMIT {} OFFSET {}",
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
            .map(|status| {
                match status {
                    TaskStatus::Done => "t.status = 'done'".to_string(),
                    TaskStatus::Abandoned => "t.status = 'abandoned'".to_string(),
                    TaskStatus::Todo => "t.status = 'todo'".to_string(), // Include but shouldn't be common for history
                }
            })
            .collect();

        if !status_conditions.is_empty() {
            conditions.push(format!("({})", status_conditions.join(" OR ")));
        }

        // Project access check and filter
        if let Some(proj_id) = &project_id {
            // Check user has access to this project
            let has_access = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM projects p 
                 LEFT JOIN project_members pm ON p.id = pm.project_id 
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

    async fn saved_views(
        &self,
        ctx: &Context<'_>,
        project_id: String,
    ) -> async_graphql::Result<Vec<SavedView>> {
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let username = &claims.sub;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(username)
            .fetch_one(pool)
            .await?
            .0;

        // Check if user has access to this project
        require_member(pool, &user_id, &project_id).await?;

        // Fetch saved views for the project
        let rows = sqlx::query(
            "SELECT id, project_id, name, filters, created_by, created_at, updated_at 
             FROM saved_views 
             WHERE project_id = ?1 
             ORDER BY name",
        )
        .bind(&project_id)
        .fetch_all(pool)
        .await?;

        let mut saved_views = Vec::new();
        for row in rows {
            let id: String = row.get("id");
            let project_id: String = row.get("project_id");
            let name: String = row.get("name");
            let filters_json: String = row.get("filters");
            let created_by: String = row.get("created_by");
            let created_at: String = row.get("created_at");
            let updated_at: String = row.get("updated_at");

            // Parse filters from JSON
            let filters: SavedViewFilters = match serde_json::from_str(&filters_json) {
                Ok(filters) => filters,
                Err(_) => {
                    return Err(async_graphql::Error::new("Invalid saved view filters"));
                }
            };

            saved_views.push(SavedView {
                id,
                project_id,
                name,
                filters,
                created_by,
                created_at,
                updated_at,
            });
        }

        Ok(saved_views)
    }

    async fn project_default_saved_view(
        &self,
        ctx: &Context<'_>,
        project_id: String,
    ) -> async_graphql::Result<Option<SavedView>> {
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let username = &claims.sub;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(username)
            .fetch_one(pool)
            .await?
            .0;

        // Check if user has access to this project
        require_member(pool, &user_id, &project_id).await?;

        // Fetch default saved view for the project
        let row_result = sqlx::query(
            "SELECT sv.id, sv.project_id, sv.name, sv.filters, sv.created_by, sv.created_at, sv.updated_at 
             FROM saved_views sv
             INNER JOIN project_default_view pdv ON sv.id = pdv.saved_view_id
             WHERE pdv.project_id = ?1"
        )
        .bind(&project_id)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row_result {
            let id: String = row.get("id");
            let project_id: String = row.get("project_id");
            let name: String = row.get("name");
            let filters_json: String = row.get("filters");
            let created_by: String = row.get("created_by");
            let created_at: String = row.get("created_at");
            let updated_at: String = row.get("updated_at");

            // Parse filters from JSON
            let filters: SavedViewFilters = match serde_json::from_str(&filters_json) {
                Ok(filters) => filters,
                Err(_) => {
                    return Err(async_graphql::Error::new("Invalid saved view filters"));
                }
            };

            Ok(Some(SavedView {
                id,
                project_id,
                name,
                filters,
                created_by,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
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
