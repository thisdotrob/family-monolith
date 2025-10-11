use async_graphql::{EmptySubscription, MergedObject, Schema};

mod auth;
pub mod shared;
mod takenlijst;
mod tests_history;
mod tests_integration;
mod tests_recurring_series;
mod tests_saved_views;
pub mod types;

pub use crate::graphql::auth::AuthenticatedMutation;
use crate::graphql::shared::{SharedMutation, SharedQuery};
use crate::graphql::takenlijst::{TakenlijstMutation, TakenlijstQuery};
use crate::graphql::types::Task;

#[derive(async_graphql::SimpleObject)]
pub struct PagedTasks {
    items: Vec<Task>,
    #[graphql(name = "totalCount")]
    total_count: i32,
}

#[derive(MergedObject, Default)]
pub struct CombinedMutation(SharedMutation, AuthenticatedMutation, TakenlijstMutation);

#[derive(MergedObject, Default)]
pub struct CombinedQuery(TakenlijstQuery, SharedQuery);

pub type AppSchema = Schema<CombinedQuery, CombinedMutation, EmptySubscription>;

pub fn build(pool: sqlx::SqlitePool) -> AppSchema {
    Schema::build(
        CombinedQuery::default(),
        CombinedMutation::default(),
        EmptySubscription,
    )
    .data(pool)
    .limit_depth(5)
    .limit_complexity(50)
    .disable_introspection()
    .finish()
}
