use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct LogoutPayload {
    pub success: bool,
}
