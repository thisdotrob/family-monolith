use async_graphql::InputObject;

#[derive(InputObject, Clone, Debug, Default)]
pub struct UpdateTaskInput {
    pub title: Option<String>,
    pub description: Option<String>,
    #[graphql(name = "assigneeId")]
    pub assignee_id: Option<String>,
    #[graphql(name = "scheduledDate")]
    pub scheduled_date: Option<String>,
    #[graphql(name = "scheduledTimeMinutes")]
    pub scheduled_time_minutes: Option<i32>,
    #[graphql(name = "deadlineDate")]
    pub deadline_date: Option<String>,
    #[graphql(name = "deadlineTimeMinutes")]
    pub deadline_time_minutes: Option<i32>,
    #[graphql(name = "tagIds")]
    pub tag_ids: Option<Vec<String>>,
}
