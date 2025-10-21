use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::error_codes::ErrorCode;
use crate::graphql::takenlijst::types::{SavedView, SavedViewFilters, SavedViewFiltersInput};
use async_graphql::{Context, ErrorExtensions, Object};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct UpdateSavedViewMutation;

#[Object]
impl UpdateSavedViewMutation {
    async fn update_saved_view(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: Option<String>,
        filters: Option<SavedViewFiltersInput>,
        last_known_updated_at: String,
    ) -> async_graphql::Result<SavedView> {
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
}
