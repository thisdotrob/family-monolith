use std::sync::Arc;

use async_graphql::{Context, ErrorExtensions, Object};
use sqlx::SqlitePool;

use crate::auth::Claims;

#[derive(Default)]
pub struct DeleteTagMutation;

#[Object]
impl DeleteTagMutation {
    async fn delete_tag(&self, ctx: &Context<'_>, tag_id: String) -> async_graphql::Result<bool> {
        // Require authentication
        let _claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Check if tag exists
        let tag_exists = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tags WHERE id = ?1")
            .bind(&tag_id)
            .fetch_one(pool)
            .await?;

        if tag_exists.0 == 0 {
            let error = async_graphql::Error::new("Tag not found")
                .extend_with(|_, e| e.set("code", "NOT_FOUND"));
            return Err(error);
        }

        // Check if tag is in use by any tasks (when task_tags table exists)
        // For now, we'll allow deletion since task_tags doesn't exist yet
        // TODO: When task_tags table exists, implement proper check.

        // Delete the tag
        let result = sqlx::query("DELETE FROM tags WHERE id = ?1")
            .bind(&tag_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
