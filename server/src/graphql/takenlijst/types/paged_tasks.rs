use async_graphql::SimpleObject;

use super::Task;

#[derive(SimpleObject)]
pub struct PagedTasks {
    pub items: Vec<Task>,
    #[graphql(name = "totalCount")]
    pub total_count: i32,
}
