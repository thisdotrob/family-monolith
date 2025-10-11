use crate::auth::Claims;
use crate::graphql::types::{LogoutInput, LogoutPayload};
use async_graphql::{Context, Object};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct LogoutMutation;

#[Object]
impl LogoutMutation {
    async fn logout(&self, ctx: &Context<'_>, input: LogoutInput) -> LogoutPayload {
        // Require valid claims for logout
        let _claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(c) => c,
            None => {
                return LogoutPayload { success: false };
            }
        };

        let pool = ctx.data::<SqlitePool>().unwrap();
        let rows = crate::auth::refresh::delete(pool, &input.refresh_token)
            .await
            .unwrap_or(0);
        LogoutPayload { success: rows > 0 }
    }
}
