use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::error_codes::ErrorCode;
use crate::graphql::types::{
    CreateSeriesInput, LogoutInput, LogoutPayload, RecurringSeries, SavedView, SavedViewFilters,
    SavedViewFiltersInput,
};
use async_graphql::{Context, ErrorExtensions, Object};
use chrono::{NaiveDate, NaiveTime, TimeZone, Utc};
use chrono_tz::Tz;
use rrule::RRule;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct AuthenticatedMutation;

#[Object]
impl AuthenticatedMutation {
    async fn logout(&self, ctx: &Context<'_>, input: LogoutInput) -> LogoutPayload {
        // Require valid claims for logout
        let _claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(c) => c,
            None => {
                return LogoutPayload { success: false };
            }
        };

        let pool = ctx.data::<SqlitePool>().unwrap();
        let rows = crate::auth::refresh::delete(pool, &input.refresh_token)
            .await
            .unwrap_or(0);
        LogoutPayload { success: rows > 0 }
    }

    async fn create_recurring_series(
        &self,
        ctx: &Context<'_>,
        input: CreateSeriesInput,
    ) -> async_graphql::Result<RecurringSeries> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID from claims
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Validate deadlineOffsetMinutes bounds (0 to 525600 minutes = 365 days)
        if input.deadline_offset_minutes < 0 || input.deadline_offset_minutes > 525600 {
            let error =
                async_graphql::Error::new("deadlineOffsetMinutes must be between 0 and 525600")
                    .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
            return Err(error);
        }

        // Validate RRULE
        let _rrule = match input.rrule.parse::<RRule<rrule::Unvalidated>>() {
            Ok(rrule) => rrule,
            Err(_) => {
                let error = async_graphql::Error::new("Invalid RRULE format")
                    .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                return Err(error);
            }
        };

        // Parse and validate dtstart date
        let dtstart_date = match NaiveDate::parse_from_str(&input.dtstart_date, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                let error =
                    async_graphql::Error::new("Invalid dtstartDate format, expected YYYY-MM-DD")
                        .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                return Err(error);
            }
        };

        // Validate timezone
        let tz = match input.timezone.parse::<Tz>() {
            Ok(tz) => tz,
            Err(_) => {
                let error = async_graphql::Error::new("Invalid timezone")
                    .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                return Err(error);
            }
        };

        // Get current time in client timezone
        let now_in_tz = Utc::now().with_timezone(&tz);
        let today_in_tz = now_in_tz.date_naive();

        // Validate that first occurrence is today or future
        if dtstart_date < today_in_tz {
            let error =
                async_graphql::Error::new("First occurrence must be today or in the future")
                    .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
            return Err(error);
        }

        // If time is present, validate that first datetime is >= now (in client timezone)
        if let Some(time_minutes) = input.dtstart_time_minutes {
            if time_minutes < 0 || time_minutes >= 1440 {
                let error =
                    async_graphql::Error::new("dtstartTimeMinutes must be between 0 and 1439")
                        .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                return Err(error);
            }

            if dtstart_date == today_in_tz {
                let hours = time_minutes / 60;
                let minutes = time_minutes % 60;
                let dtstart_time = match NaiveTime::from_hms_opt(hours as u32, minutes as u32, 0) {
                    Some(time) => time,
                    None => {
                        let error = async_graphql::Error::new("Invalid dtstartTimeMinutes")
                            .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                        return Err(error);
                    }
                };

                let dtstart_datetime = dtstart_date.and_time(dtstart_time);
                let dtstart_in_tz = tz.from_local_datetime(&dtstart_datetime).single();

                if let Some(dtstart_in_tz) = dtstart_in_tz {
                    if dtstart_in_tz < now_in_tz {
                        let error = async_graphql::Error::new(
                            "First occurrence datetime must be >= now in client timezone",
                        )
                        .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                        return Err(error);
                    }
                } else {
                    let error = async_graphql::Error::new("Invalid datetime in client timezone")
                        .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                    return Err(error);
                }
            }
        }

        // Validate project exists and user has access
        require_member(pool, &user_id, &input.project_id).await?;

        // Validate assignee exists if provided
        if let Some(ref assignee_id) = input.assignee_id {
            let assignee_exists =
                sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM users WHERE id = ?1")
                    .bind(assignee_id)
                    .fetch_one(pool)
                    .await?;

            if assignee_exists.0 == 0 {
                let error = async_graphql::Error::new("Assignee not found")
                    .extend_with(|_, e| e.set("code", "NOT_FOUND"));
                return Err(error);
            }
        }

        // Validate and normalize defaultTagIds
        let default_tag_ids = input.default_tag_ids.unwrap_or_default();
        for tag_id in &default_tag_ids {
            let tag_exists = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tags WHERE id = ?1")
                .bind(tag_id)
                .fetch_one(pool)
                .await?;

            if tag_exists.0 == 0 {
                let error = async_graphql::Error::new("One or more tags not found")
                    .extend_with(|_, e| e.set("code", "NOT_FOUND"));
                return Err(error);
            }
        }

        // Create the recurring series
        let series_id = uuid::Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO recurring_series 
             (id, project_id, created_by, title, description, assignee_id, rrule, dtstart_date, dtstart_time_minutes, deadline_offset_minutes) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
        )
        .bind(&series_id)
        .bind(&input.project_id)
        .bind(&user_id)
        .bind(&input.title)
        .bind(&input.description)
        .bind(&input.assignee_id)
        .bind(&input.rrule)
        .bind(&input.dtstart_date)
        .bind(input.dtstart_time_minutes)
        .bind(input.deadline_offset_minutes)
        .execute(pool)
        .await?;

        // Insert default tags
        for tag_id in &default_tag_ids {
            sqlx::query("INSERT INTO recurring_series_tags (series_id, tag_id) VALUES (?1, ?2)")
                .bind(&series_id)
                .bind(tag_id)
                .execute(pool)
                .await?;
        }

        // Fetch the created series
        let series = sqlx::query_as::<_, (String, String, String, String, Option<String>, Option<String>, String, String, Option<i32>, i32, String, String)>(
            "SELECT id, project_id, created_by, title, description, assignee_id, rrule, dtstart_date, dtstart_time_minutes, deadline_offset_minutes, created_at, updated_at 
             FROM recurring_series WHERE id = ?1"
        )
        .bind(&series_id)
        .fetch_one(pool)
        .await?;

        Ok(RecurringSeries {
            id: series.0,
            project_id: series.1,
            created_by: series.2,
            title: series.3,
            description: series.4,
            assignee_id: series.5,
            rrule: series.6,
            dtstart_date: series.7,
            dtstart_time_minutes: series.8,
            deadline_offset_minutes: series.9,
            created_at: series.10,
            updated_at: series.11,
            default_tag_ids,
        })
    }

    async fn create_saved_view(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        name: String,
        filters: SavedViewFiltersInput,
    ) -> async_graphql::Result<crate::graphql::SavedView> {
        use crate::db::helpers::normalize_project_name; // Reuse for general name normalization

        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Check if user has access to this project
        require_member(pool, &user_id, &project_id).await?;

        // Validate and normalize name
        let normalized_name = normalize_project_name(&name); // Reuse existing normalization
        if normalized_name.is_empty() {
            let error = async_graphql::Error::new("Saved view name cannot be empty")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        if normalized_name.len() > 60 {
            let error = async_graphql::Error::new("Saved view name cannot exceed 60 characters")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Check for unique name per project (case-insensitive)
        let existing_count = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM saved_views WHERE project_id = ?1 AND LOWER(TRIM(name)) = LOWER(TRIM(?2))"
        )
        .bind(&project_id)
        .bind(&normalized_name)
        .fetch_one(pool)
        .await?;

        if existing_count.0 > 0 {
            let error = async_graphql::Error::new(
                "A saved view with this name already exists in this project",
            )
            .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Validate assignee exists if provided
        if let Some(ref assignee_id) = filters.assignee {
            let assignee_exists =
                sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM users WHERE id = ?1")
                    .bind(assignee_id)
                    .fetch_one(pool)
                    .await?;

            if assignee_exists.0 == 0 {
                let error = async_graphql::Error::new("Assignee not found")
                    .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                return Err(error);
            }
        }

        // Validate tag IDs exist
        for tag_id in &filters.tag_ids {
            let tag_exists = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tags WHERE id = ?1")
                .bind(tag_id)
                .fetch_one(pool)
                .await?;

            if tag_exists.0 == 0 {
                let error = async_graphql::Error::new("One or more tags not found")
                    .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                return Err(error);
            }
        }

        // Convert input filters to JSON
        let filters_obj = SavedViewFilters {
            statuses: filters.statuses,
            assignee: filters.assignee,
            include_unassigned: filters.include_unassigned,
            assigned_to_me: filters.assigned_to_me,
            tag_ids: filters.tag_ids,
        };

        let filters_json = serde_json::to_string(&filters_obj).map_err(|_| {
            async_graphql::Error::new("Failed to serialize filters")
                .extend_with(|_, e| e.set("code", ErrorCode::Internal.as_str()))
        })?;

        // Create the saved view
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO saved_views (id, project_id, name, filters, created_by) VALUES (?1, ?2, ?3, ?4, ?5)"
        )
        .bind(&id)
        .bind(&project_id)
        .bind(&normalized_name)
        .bind(&filters_json)
        .bind(&user_id)
        .execute(pool)
        .await?;

        // Fetch the created saved view
        let saved_view = sqlx::query_as::<_, (String, String, String, String, String, String, String)>(
            "SELECT id, project_id, name, filters, created_by, created_at, updated_at FROM saved_views WHERE id = ?1"
        )
        .bind(&id)
        .fetch_one(pool)
        .await?;

        Ok(SavedView {
            id: saved_view.0,
            project_id: saved_view.1,
            name: saved_view.2,
            filters: filters_obj,
            created_by: saved_view.4,
            created_at: saved_view.5,
            updated_at: saved_view.6,
        })
    }

    async fn update_saved_view(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: Option<String>,
        filters: Option<SavedViewFiltersInput>,
        last_known_updated_at: String,
    ) -> async_graphql::Result<crate::graphql::SavedView> {
        use crate::db::helpers::normalize_project_name;

        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Get current saved view
        let current = sqlx::query_as::<_, (String, String, String, String, String, String, String)>(
            "SELECT id, project_id, name, filters, created_by, created_at, updated_at FROM saved_views WHERE id = ?1"
        )
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            async_graphql::Error::new("Saved view not found")
                .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()))
        })?;

        // Check if user has access to this project
        require_member(pool, &user_id, &current.1).await?;

        // Check for stale write
        if current.6 != last_known_updated_at {
            let error = async_graphql::Error::new("Saved view has been modified by another user")
                .extend_with(|_, e| e.set("code", ErrorCode::ConflictStaleWrite.as_str()));
            return Err(error);
        }

        // Prepare updated values
        let updated_name = if let Some(name) = name {
            let normalized_name = normalize_project_name(&name);
            if normalized_name.is_empty() {
                let error = async_graphql::Error::new("Saved view name cannot be empty")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }

            if normalized_name.len() > 60 {
                let error =
                    async_graphql::Error::new("Saved view name cannot exceed 60 characters")
                        .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }

            // Check for unique name per project (case-insensitive), excluding current view
            let existing_count = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM saved_views WHERE project_id = ?1 AND LOWER(TRIM(name)) = LOWER(TRIM(?2)) AND id != ?3"
            )
            .bind(&current.1)
            .bind(&normalized_name)
            .bind(&id)
            .fetch_one(pool)
            .await?;

            if existing_count.0 > 0 {
                let error = async_graphql::Error::new(
                    "A saved view with this name already exists in this project",
                )
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }

            normalized_name
        } else {
            current.2.clone()
        };

        let updated_filters_json = if let Some(filters) = filters {
            // Validate assignee exists if provided
            if let Some(ref assignee_id) = filters.assignee {
                let assignee_exists =
                    sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM users WHERE id = ?1")
                        .bind(assignee_id)
                        .fetch_one(pool)
                        .await?;

                if assignee_exists.0 == 0 {
                    let error = async_graphql::Error::new("Assignee not found")
                        .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                    return Err(error);
                }
            }

            // Validate tag IDs exist
            for tag_id in &filters.tag_ids {
                let tag_exists =
                    sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tags WHERE id = ?1")
                        .bind(tag_id)
                        .fetch_one(pool)
                        .await?;

                if tag_exists.0 == 0 {
                    let error = async_graphql::Error::new("One or more tags not found")
                        .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                    return Err(error);
                }
            }

            let filters_obj = SavedViewFilters {
                statuses: filters.statuses,
                assignee: filters.assignee,
                include_unassigned: filters.include_unassigned,
                assigned_to_me: filters.assigned_to_me,
                tag_ids: filters.tag_ids,
            };

            serde_json::to_string(&filters_obj).map_err(|_| {
                async_graphql::Error::new("Failed to serialize filters")
                    .extend_with(|_, e| e.set("code", ErrorCode::Internal.as_str()))
            })?
        } else {
            current.3.clone()
        };

        // Update the saved view
        sqlx::query(
            "UPDATE saved_views SET name = ?1, filters = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?3"
        )
        .bind(&updated_name)
        .bind(&updated_filters_json)
        .bind(&id)
        .execute(pool)
        .await?;

        // Fetch updated saved view
        let updated = sqlx::query_as::<_, (String, String, String, String, String, String, String)>(
            "SELECT id, project_id, name, filters, created_by, created_at, updated_at FROM saved_views WHERE id = ?1"
        )
        .bind(&id)
        .fetch_one(pool)
        .await?;

        let filters: SavedViewFilters = serde_json::from_str(&updated.3).map_err(|_| {
            async_graphql::Error::new("Invalid saved view filters")
                .extend_with(|_, e| e.set("code", ErrorCode::Internal.as_str()))
        })?;

        Ok(SavedView {
            id: updated.0,
            project_id: updated.1,
            name: updated.2,
            filters,
            created_by: updated.4,
            created_at: updated.5,
            updated_at: updated.6,
        })
    }

    async fn delete_saved_view(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<bool> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Get saved view to check project access
        let saved_view = sqlx::query_as::<_, (String, String)>(
            "SELECT id, project_id FROM saved_views WHERE id = ?1",
        )
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            async_graphql::Error::new("Saved view not found")
                .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()))
        })?;

        // Check if user has access to this project
        require_member(pool, &user_id, &saved_view.1).await?;

        // Remove from default view if it's set as default
        sqlx::query("DELETE FROM project_default_view WHERE saved_view_id = ?1")
            .bind(&id)
            .execute(pool)
            .await?;

        // Delete the saved view
        let result = sqlx::query("DELETE FROM saved_views WHERE id = ?1")
            .bind(&id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn set_project_default_saved_view(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        saved_view_id: Option<String>,
    ) -> async_graphql::Result<bool> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Check if user has access to this project
        require_member(pool, &user_id, &project_id).await?;

        if let Some(view_id) = saved_view_id {
            // Validate that the saved view exists and belongs to this project
            let view_exists = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM saved_views WHERE id = ?1 AND project_id = ?2",
            )
            .bind(&view_id)
            .bind(&project_id)
            .fetch_one(pool)
            .await?;

            if view_exists.0 == 0 {
                let error = async_graphql::Error::new("Saved view not found in this project")
                    .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                return Err(error);
            }

            // Set or update the default view
            sqlx::query(
                "INSERT OR REPLACE INTO project_default_view (project_id, saved_view_id) VALUES (?1, ?2)"
            )
            .bind(&project_id)
            .bind(&view_id)
            .execute(pool)
            .await?;
        } else {
            // Clear the default view
            sqlx::query("DELETE FROM project_default_view WHERE project_id = ?1")
                .bind(&project_id)
                .execute(pool)
                .await?;
        }

        Ok(true)
    }
}
