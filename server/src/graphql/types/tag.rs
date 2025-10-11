use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Tag {
    pub id: String,
    pub name: String,
    #[graphql(name = "createdAt")]
    pub created_at: String,
    #[graphql(name = "updatedAt")]
    pub updated_at: String,
}
