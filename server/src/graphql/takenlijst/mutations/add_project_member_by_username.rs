use std::sync::Arc;

use async_graphql::{Context, ErrorExtensions, Object};
use sqlx::SqlitePool;

use crate::auth::Claims;
use crate::auth::guard::require_owner;
use crate::error_codes::ErrorCode;

#[derive(Default)]
pub struct AddProjectMemberByUsernameMutation;

#[Object]
impl AddProjectMemberByUsernameMutation {
    async fn add_project_member_by_username(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        username: String,
    ) -> async_graphql::Result<bool> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get current user ID
        let current_user_id =
            sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
                .bind(&claims.sub)
                .fetch_one(pool)
                .await?
                .0;

        // Check permission (only owner can add members)
        require_owner(pool, &current_user_id, &project_id).await?;

        // Get project info for owner check later
        let project = sqlx::query_as::<_, (String, String)>(
            "SELECT id, owner_id FROM projects WHERE id = ?1",
        )
        .bind(&project_id)
        .fetch_one(pool)
        .await?;

        // Find the user to add
        let target_user =
            sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
                .bind(&username.to_lowercase())
                .fetch_one(pool)
                .await;

        let target_user_id = match target_user {
            Ok(u) => u.0,
            Err(_) => {
                let error = async_graphql::Error::new("User not found")
                    .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                return Err(error);
            }
        };

        // Check if user is already a member or owner
        let existing_membership = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM project_members WHERE project_id = ?1 AND user_id = ?2",
        )
        .bind(&project_id)
        .bind(&target_user_id)
        .fetch_one(pool)
        .await?;

        if existing_membership.0 > 0 || project.1 == target_user_id {
            let error = async_graphql::Error::new("User is already a member of this project")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Add the member
        sqlx::query("INSERT INTO project_members (project_id, user_id) VALUES (?1, ?2)")
            .bind(&project_id)
            .bind(&target_user_id)
            .execute(pool)
            .await?;

        Ok(true)
    }
}
