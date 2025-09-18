use sqlx::SqlitePool;
mod auth;
mod champ_tracker;

pub use crate::graphql::auth::{AuthenticatedMutation, UnauthenticatedMutation};
pub use crate::graphql::champ_tracker::{ChampTrackerQueries, ChampTrackerMutations};
use async_graphql::{Context, EmptySubscription, Schema};
use async_graphql::{MergedObject, Object, SimpleObject};

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

    async fn champ_tracker(&self) -> ChampTrackerQueries {
        ChampTrackerQueries::default()
    }
}

#[derive(MergedObject, Default)]
pub struct CombinedMutation(UnauthenticatedMutation, AuthenticatedMutation, ChampTrackerMutations);

pub type AppSchema = Schema<QueryRoot, CombinedMutation, EmptySubscription>;

pub fn build(pool: SqlitePool) -> AppSchema {
    Schema::build(QueryRoot, CombinedMutation::default(), EmptySubscription)
        .data(pool)
        .limit_depth(5)
        .limit_complexity(50)
        .disable_introspection()
        .finish()
}
