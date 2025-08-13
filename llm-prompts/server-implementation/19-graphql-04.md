Add logout mutation.

Tasks
1. Extend `AuthMutation` in `src/graphql/auth.rs` with `logout` function:

   ```rust
   #[derive(InputObject)]
   struct LogoutInput { refresh_token: String }

   #[derive(SimpleObject)]
   struct LogoutPayload { success: bool }

   #[Object]
   impl AuthMutation {
       async fn logout(&self, input: LogoutInput) -> LogoutPayload {
           let rows = crate::auth::refresh::delete(&input.refresh_token).await.unwrap_or(0);
           LogoutPayload{ success: rows > 0 }
       }
   }
   ```

2. Tests: token removed, subsequent rotate fails.

Commit message: "feat(gql): auth.logout"
