use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::error_codes::ErrorCode;
use async_graphql::{Context, ErrorExtensions, Object};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct DeleteSavedViewMutation;

#[Object]
impl DeleteSavedViewMutation {
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
}
