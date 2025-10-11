use crate::auth::Claims;
use crate::graphql::types::User;
use async_graphql::{Context, Object};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct MeQuery;

#[Object]
impl MeQuery {
    async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        let username = &claims.sub;

        let user_data = sqlx::query_as::<_, (String, Option<String>)>(
            "SELECT username, first_name FROM users WHERE username = ?1",
        )
        .bind(username)
        .fetch_one(pool)
        .await?;

        Ok(User {
            username: user_data.0,
            first_name: user_data.1,
        })
    }
}
