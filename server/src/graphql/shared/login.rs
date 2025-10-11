use crate::graphql::types::{LoginInput, LoginPayload};
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct LoginMutation;

#[Object]
impl LoginMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> LoginPayload {
        let pool = ctx.data::<sqlx::SqlitePool>().unwrap();
        let user_result = sqlx::query_as::<_, (String, String, String)>(
            "SELECT id, username, password FROM users WHERE username = ?1",
        )
        .bind(&input.username.to_lowercase())
        .fetch_one(pool)
        .await;

        if let Ok(user) = user_result {
            if crate::auth::verify(&user.2, &input.password).await {
                let token = crate::auth::encode(&user.1, 5).unwrap();
                let refresh = crate::auth::refresh::create(pool, &user.0).await.unwrap();
                return LoginPayload {
                    success: true,
                    token: Some(token),
                    refresh_token: Some(refresh),
                    errors: vec![],
                };
            }
        }
        LoginPayload {
            success: false,
            token: None,
            refresh_token: None,
            errors: vec!["INVALID_CREDENTIALS".into()],
        }
    }
}
