use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct RecurringSeries {
    pub id: String,
    #[graphql(name = "projectId")]
    pub project_id: String,
    #[graphql(name = "createdBy")]
    pub created_by: String,
    pub title: String,
    pub description: Option<String>,
    #[graphql(name = "assigneeId")]
    pub assignee_id: Option<String>,
    pub rrule: String,
    #[graphql(name = "dtstartDate")]
    pub dtstart_date: String,
    #[graphql(name = "dtstartTimeMinutes")]
    pub dtstart_time_minutes: Option<i32>,
    #[graphql(name = "deadlineOffsetMinutes")]
    pub deadline_offset_minutes: i32,
    #[graphql(name = "createdAt")]
    pub created_at: String,
    #[graphql(name = "updatedAt")]
    pub updated_at: String,
    #[graphql(name = "defaultTagIds")]
    pub default_tag_ids: Vec<String>,
}
