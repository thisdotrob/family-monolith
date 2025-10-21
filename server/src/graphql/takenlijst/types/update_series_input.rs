use async_graphql::InputObject;

#[derive(InputObject)]
pub struct UpdateSeriesInput {
    pub title: Option<String>,
    pub description: Option<String>,
    #[graphql(name = "assigneeId")]
    pub assignee_id: Option<String>,
    #[graphql(name = "defaultTagIds")]
    pub default_tag_ids: Option<Vec<String>>,
    pub rrule: Option<String>,
    #[graphql(name = "dtstartDate")]
    pub dtstart_date: Option<String>,
    #[graphql(name = "dtstartTimeMinutes")]
    pub dtstart_time_minutes: Option<i32>,
    #[graphql(name = "deadlineOffsetMinutes")]
    pub deadline_offset_minutes: Option<i32>,
    pub timezone: String,
}
