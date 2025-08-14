use async_graphql::{Context, InputObject, Object, SimpleObject};
use sqlx::SqlitePool;

#[derive(InputObject)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct LoginPayload {
    pub success: bool,
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub errors: Vec<String>,
}

pub struct UnauthenticatedMutation;

#[Object]
impl UnauthenticatedMutation {
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
                let token = crate::auth::encode(&user.1, 30).unwrap();
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

    async fn refresh_token(&self, ctx: &Context<'_>, input: RefreshInput) -> RefreshPayload {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let refresh_token = input.refresh_token;

        // Since we can't access the private refresh module directly,
        // we need to implement the rotation logic here

        // First, try to find the user_id associated with this token
        let user_id_result = crate::db::helpers::fetch_one::<(String,)>(
            pool,
            "SELECT user_id FROM refresh_tokens WHERE token = ?1",
            &[&refresh_token],
        )
        .await;

        if let Ok((user_id,)) = user_id_result {
            // Delete the old token
            let _ = crate::db::helpers::execute(
                pool,
                "DELETE FROM refresh_tokens WHERE token = ?1",
                &[&refresh_token],
            )
            .await;

            // Create a new token for the same user
            if let Ok(new_rt) = crate::auth::refresh::create(pool, &user_id).await {
                // Get the username to embed in JWT
                if let Ok((username,)) = crate::db::helpers::fetch_one::<(String,)>(
                    pool,
                    "SELECT username FROM users WHERE id = ?1",
                    &[&user_id],
                )
                .await
                {
                    let token = crate::auth::encode(&username, 30).unwrap();
                    return RefreshPayload {
                        success: true,
                        token: Some(token),
                        refresh_token: Some(new_rt),
                        errors: vec![],
                    };
                }
            }
        }

        RefreshPayload {
            success: false,
            token: None,
            refresh_token: None,
            errors: vec!["TOKEN_INVALID".into()],
        }
    }
}

pub struct AuthenticatedMutation;

#[Object]
impl AuthenticatedMutation {
    async fn logout(&self, ctx: &Context<'_>, input: LogoutInput) -> LogoutPayload {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let rows = crate::auth::refresh::delete(pool, &input.refresh_token)
            .await
            .unwrap_or(0);
        LogoutPayload { success: rows > 0 }
    }
}

#[derive(InputObject)]
struct RefreshInput {
    refresh_token: String,
}

#[derive(SimpleObject)]
struct RefreshPayload {
    success: bool,
    token: Option<String>,
    refresh_token: Option<String>,
    errors: Vec<String>,
}

#[derive(InputObject)]
struct LogoutInput {
    refresh_token: String,
}

#[derive(SimpleObject)]
struct LogoutPayload {
    success: bool,
}
