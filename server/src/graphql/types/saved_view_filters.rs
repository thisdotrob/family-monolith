use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::tasks::TaskStatus;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct SavedViewFilters {
    pub statuses: Vec<TaskStatus>,
    pub assignee: Option<String>,
    #[graphql(name = "includeUnassigned")]
    #[serde(rename = "includeUnassigned")]
    pub include_unassigned: bool,
    #[graphql(name = "assignedToMe")]
    #[serde(rename = "assignedToMe")]
    pub assigned_to_me: bool,
    #[graphql(name = "tagIds")]
    #[serde(rename = "tagIds")]
    pub tag_ids: Vec<String>,
}
