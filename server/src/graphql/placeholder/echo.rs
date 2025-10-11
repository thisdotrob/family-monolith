use async_graphql::{Context, InputObject, Object, SimpleObject};

#[derive(InputObject)]
pub struct EchoInput {
    pub message: String,
}

#[derive(SimpleObject)]
pub struct EchoPayload {
    pub echo: String,
    pub success: bool,
}

#[derive(Default)]
pub struct EchoMutation;

#[Object]
impl EchoMutation {
    async fn echo(&self, _ctx: &Context<'_>, input: EchoInput) -> EchoPayload {
        EchoPayload {
            echo: format!("Echo: {}", input.message),
            success: true,
        }
    }
}
