use sqlx::SqlitePool;
mod auth;

use crate::auth::guard::AuthGuard;
pub use crate::graphql::auth::{AuthenticatedMutation, UnauthenticatedMutation};
use async_graphql::Object;
use async_graphql::{EmptySubscription, Schema};

// Define an empty query root
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn type_name(&self) -> &'static str {
        "Query"
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
        .extension(AuthGuard)
        .limit_depth(5)
        .limit_complexity(50)
        .disable_introspection()
        .finish()
}
