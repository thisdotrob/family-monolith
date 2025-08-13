use sqlx::SqlitePool;
mod auth;

pub use crate::graphql::auth::{AuthenticatedMutation, UnauthenticatedMutation};
use async_graphql::{Object, SimpleObject};
use async_graphql::{Context, EmptySubscription, Schema};

use crate::auth::Claims;
use std::sync::Arc;

#[derive(SimpleObject)]
struct User {
    username: String,
    #[graphql(name = "firstName")]
    first_name: Option<String>,
}

// Define a query root
pub struct QueryRoot;

#[Object]
impl QueryRoot {
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

pub type UnauthenticatedSchema = Schema<QueryRoot, UnauthenticatedMutation, EmptySubscription>;
pub type AuthenticatedSchema = Schema<QueryRoot, AuthenticatedMutation, EmptySubscription>;

pub fn build_unauthenticated(pool: SqlitePool) -> UnauthenticatedSchema {
    Schema::build(QueryRoot, UnauthenticatedMutation, EmptySubscription)
        .data(pool)
        .limit_depth(5)
        .limit_complexity(50)
        .disable_introspection()
        .finish()
}

pub fn build_authenticated(pool: SqlitePool) -> AuthenticatedSchema {
    Schema::build(QueryRoot, AuthenticatedMutation, EmptySubscription)
        .data(pool)
        .limit_depth(5)
        .limit_complexity(50)
        .disable_introspection()
        .finish()
}
