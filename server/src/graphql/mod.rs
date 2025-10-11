use async_graphql::{EmptySubscription, MergedObject, Schema};

mod placeholder;
pub mod shared;
mod takenlijst;
mod tests_history;
mod tests_integration;
mod tests_recurring_series;
mod tests_saved_views;
pub mod types;

use crate::graphql::placeholder::{PlaceholderMutation, PlaceholderQuery};
use crate::graphql::shared::{SharedMutation, SharedQuery};
use crate::graphql::takenlijst::{TakenlijstMutation, TakenlijstQuery};
#[derive(MergedObject, Default)]
pub struct CombinedMutation(SharedMutation, TakenlijstMutation, PlaceholderMutation);

#[derive(MergedObject, Default)]
pub struct QueryRoot(SharedQuery, TakenlijstQuery, PlaceholderQuery);

pub type AppSchema = Schema<QueryRoot, CombinedMutation, EmptySubscription>;

pub fn build(pool: sqlx::SqlitePool) -> AppSchema {
    Schema::build(
        QueryRoot::default(),
        CombinedMutation::default(),
        EmptySubscription,
    )
    .data(pool)
    .limit_depth(5)
    .limit_complexity(50)
    .disable_introspection()
    .finish()
}
