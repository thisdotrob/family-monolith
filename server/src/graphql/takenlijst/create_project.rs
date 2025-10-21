use std::sync::Arc;

use async_graphql::{Context, ErrorExtensions, Object};
use sqlx::SqlitePool;

use crate::auth::Claims;
use crate::db::helpers::normalize_project_name;
use crate::error_codes::ErrorCode;
use crate::graphql::takenlijst::types::Project;

#[derive(Default)]
pub struct CreateProjectMutation;

#[Object]
impl CreateProjectMutation {
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
}
