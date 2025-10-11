use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Project {
    pub id: String,
    pub name: String,
    #[graphql(name = "ownerId")]
    pub owner_id: String,
    #[graphql(name = "archivedAt")]
    pub archived_at: Option<String>,
    #[graphql(name = "createdAt")]
    pub created_at: String,
    #[graphql(name = "updatedAt")]
    pub updated_at: String,
}
