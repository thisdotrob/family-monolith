use async_graphql::InputObject;

#[derive(InputObject)]
pub struct LogoutInput {
    pub refresh_token: String,
}
