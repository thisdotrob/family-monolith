use std::sync::Arc;

use async_graphql::{Context, ErrorExtensions, Object};
use sqlx::SqlitePool;

use crate::auth::Claims;
use crate::db::helpers::normalize_tag_name;
use crate::graphql::types::Tag;

#[derive(Default)]
pub struct CreateTagMutation;

#[Object]
impl CreateTagMutation {
    async fn create_tag(&self, ctx: &Context<'_>, name: String) -> async_graphql::Result<Tag> {
        // Require authentication
        let _claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let normalized_name = normalize_tag_name(&name);

        if normalized_name.is_empty() {
            let error = async_graphql::Error::new("Tag name cannot be empty after normalization")
                .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
            return Err(error);
        }

        // Check if tag already exists (normalized comparison)
        if let Ok(existing_tag) = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, name, created_at, updated_at FROM tags WHERE name = ?1",
        )
        .bind(&normalized_name)
        .fetch_one(pool)
        .await
        {
            // Return existing tag
            return Ok(Tag {
                id: existing_tag.0,
                name: existing_tag.1,
                created_at: existing_tag.2,
                updated_at: existing_tag.3,
            });
        }

        // Create new tag
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO tags (id, name) VALUES (?1, ?2)")
            .bind(&id)
            .bind(&normalized_name)
            .execute(pool)
            .await?;

        // Fetch the created tag
        let tag = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, name, created_at, updated_at FROM tags WHERE id = ?1",
        )
        .bind(&id)
        .fetch_one(pool)
        .await?;

        Ok(Tag {
            id: tag.0,
            name: tag.1,
            created_at: tag.2,
            updated_at: tag.3,
        })
    }
}
