use async_graphql::InputObject;

use crate::tasks::TaskStatus;

#[derive(InputObject)]
pub struct SavedViewFiltersInput {
    pub statuses: Vec<TaskStatus>,
    pub assignee: Option<String>,
    #[graphql(name = "includeUnassigned")]
    pub include_unassigned: bool,
    #[graphql(name = "assignedToMe")]
    pub assigned_to_me: bool,
    #[graphql(name = "tagIds")]
    pub tag_ids: Vec<String>,
}
