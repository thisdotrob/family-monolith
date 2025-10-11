use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::error_codes::ErrorCode;
use async_graphql::{Context, ErrorExtensions, Object};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct SetProjectDefaultSavedViewMutation;

#[Object]
impl SetProjectDefaultSavedViewMutation {
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
