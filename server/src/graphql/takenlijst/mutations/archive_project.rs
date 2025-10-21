use std::sync::Arc;

use async_graphql::{Context, ErrorExtensions, Object};
use sqlx::SqlitePool;

use crate::auth::Claims;
use crate::auth::guard::require_owner;
use crate::error_codes::ErrorCode;
use crate::graphql::takenlijst::types::Project;

#[derive(Default)]
pub struct ArchiveProjectMutation;

#[Object]
impl ArchiveProjectMutation {
    async fn archive_project(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        last_known_updated_at: String,
    ) -> async_graphql::Result<Project> {
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

        // Check permission (only owner can archive)
        require_owner(pool, &user_id, &project_id).await?;

        // Get current project state
        let project = sqlx::query_as::<_, (String, String, String, Option<String>, String, String)>(
            "SELECT id, name, owner_id, archived_at, created_at, updated_at FROM projects WHERE id = ?1",
        )
        .bind(&project_id)
        .fetch_one(pool)
        .await?;

        // Check for stale write
        if project.5 != last_known_updated_at {
            let error = async_graphql::Error::new("Project has been modified by another user")
                .extend_with(|_, e| e.set("code", ErrorCode::ConflictStaleWrite.as_str()));
            return Err(error);
        }

        // Archive the project
        sqlx::query("UPDATE projects SET archived_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = ?1")
            .bind(&project_id)
            .execute(pool)
            .await?;

        // Fetch updated project
        let updated_project = sqlx::query_as::<_, (String, String, String, Option<String>, String, String)>(
            "SELECT id, name, owner_id, archived_at, created_at, updated_at FROM projects WHERE id = ?1",
        )
        .bind(&project_id)
        .fetch_one(pool)
        .await?;

        Ok(Project {
            id: updated_project.0,
            name: updated_project.1,
            owner_id: updated_project.2,
            archived_at: updated_project.3,
            created_at: updated_project.4,
            updated_at: updated_project.5,
        })
    }
}
