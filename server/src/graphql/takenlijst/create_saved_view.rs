use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::error_codes::ErrorCode;
use crate::graphql::types::{SavedView, SavedViewFilters, SavedViewFiltersInput};
use async_graphql::{Context, ErrorExtensions, Object};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct CreateSavedViewMutation;

#[Object]
impl CreateSavedViewMutation {
    async fn create_saved_view(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        name: String,
        filters: SavedViewFiltersInput,
    ) -> async_graphql::Result<SavedView> {
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
}
