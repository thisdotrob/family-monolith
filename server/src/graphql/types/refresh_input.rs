use async_graphql::InputObject;

#[derive(InputObject)]
pub struct RefreshInput {
    pub refresh_token: String,
}
