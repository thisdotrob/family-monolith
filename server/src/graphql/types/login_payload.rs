use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct LoginPayload {
    pub success: bool,
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub errors: Vec<String>,
}
