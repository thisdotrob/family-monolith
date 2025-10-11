use crate::auth::Claims;
use crate::graphql::types::Tag;
use async_graphql::{Context, Object};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct TagsQuery;

#[Object]
impl TagsQuery {
    async fn tags(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] offset: i32,
        #[graphql(default = 200)] limit: i32,
    ) -> async_graphql::Result<Vec<Tag>> {
        // Require authentication
        let _claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        let tags = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, name, created_at, updated_at FROM tags ORDER BY name LIMIT ?1 OFFSET ?2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(tags
            .into_iter()
            .map(|(id, name, created_at, updated_at)| Tag {
                id,
                name,
                created_at,
                updated_at,
            })
            .collect())
    }
}
