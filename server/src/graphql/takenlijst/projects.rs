use std::sync::Arc;

use async_graphql::{Context, ErrorExtensions, Object};
use sqlx::SqlitePool;

use crate::auth::Claims;
use crate::auth::guard::require_owner;
use crate::db::helpers::normalize_project_name;
use crate::error_codes::ErrorCode;
use crate::graphql::types::Project;

#[derive(Default)]
pub struct ProjectsMutation;

#[Object]
impl ProjectsMutation {
    async fn create_project(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> async_graphql::Result<Project> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let normalized_name = normalize_project_name(&name);

        // Validate name
        if normalized_name.is_empty() {
            let error = async_graphql::Error::new("Project name cannot be empty")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        if normalized_name.len() > 60 {
            let error = async_graphql::Error::new("Project name cannot exceed 60 characters")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Create new project
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO projects (id, name, owner_id) VALUES (?1, ?2, ?3)")
            .bind(&id)
            .bind(&normalized_name)
            .bind(&user_id)
            .execute(pool)
            .await?;

        // Fetch the created project
        let project = sqlx::query_as::<_, (String, String, String, Option<String>, String, String)>(
            "SELECT id, name, owner_id, archived_at, created_at, updated_at FROM projects WHERE id = ?1",
        )
        .bind(&id)
        .fetch_one(pool)
        .await?;

        Ok(Project {
            id: project.0,
            name: project.1,
            owner_id: project.2,
            archived_at: project.3,
            created_at: project.4,
            updated_at: project.5,
        })
    }

    async fn rename_project(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        name: String,
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
        let normalized_name = normalize_project_name(&name);

        // Validate name
        if normalized_name.is_empty() {
            let error = async_graphql::Error::new("Project name cannot be empty")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        if normalized_name.len() > 60 {
            let error = async_graphql::Error::new("Project name cannot exceed 60 characters")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Check permission (only owner can rename)
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

        // Update the project
        sqlx::query("UPDATE projects SET name = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2")
            .bind(&normalized_name)
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

    async fn unarchive_project(
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

        // Check permission (only owner can unarchive)
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

        // Unarchive the project
        sqlx::query(
            "UPDATE projects SET archived_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
        )
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
