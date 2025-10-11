use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct User {
    pub username: String,
    #[graphql(name = "firstName")]
    pub first_name: Option<String>,
}
