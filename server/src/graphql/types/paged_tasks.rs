use async_graphql::SimpleObject;

use crate::graphql::types::Task;

#[derive(SimpleObject)]
pub struct PagedTasks {
    pub items: Vec<Task>,
    #[graphql(name = "totalCount")]
    pub total_count: i32,
}
