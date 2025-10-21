use async_graphql::InputObject;

#[derive(InputObject)]
pub struct CreateSeriesInput {
    #[graphql(name = "projectId")]
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
    #[graphql(name = "assigneeId")]
    pub assignee_id: Option<String>,
    #[graphql(name = "defaultTagIds")]
    pub default_tag_ids: Option<Vec<String>>,
    pub rrule: String,
    #[graphql(name = "dtstartDate")]
    pub dtstart_date: String,
    #[graphql(name = "dtstartTimeMinutes")]
    pub dtstart_time_minutes: Option<i32>,
    #[graphql(name = "deadlineOffsetMinutes")]
    pub deadline_offset_minutes: i32,
    pub timezone: String,
}
