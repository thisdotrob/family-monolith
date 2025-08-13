Add refreshToken mutation.

Tasks
1. In `graphql/auth.rs` append:

   ```rust
   #[derive(InputObject)]
   struct RefreshInput { refresh_token: String }

   #[derive(SimpleObject)]
   struct RefreshPayload { success: bool, token: Option<String>, refresh_token: Option<String>, errors: Vec<String> }

   #[Object]
   impl AuthMutation {
       async fn refresh_token(&self, ctx:&Context<'_>, input:RefreshInput) -> RefreshPayload {
           match crate::auth::refresh::rotate(&input.refresh_token).await {
               Ok(Some(new_rt)) => {
                   // We still need username to embed in JWT
                   if let Ok((username,)) = crate::db::fetch_one::<(String,)>(
                         "SELECT username FROM users WHERE id = (SELECT user_id FROM refresh_tokens WHERE token = ?1)", (&new_rt,)).await {
                       let token = crate::auth::jwt::encode(&username, 24*60*60).unwrap();
                       return RefreshPayload{ success:true, token:Some(token), refresh_token:Some(new_rt), errors:vec![] }
                   }
               }
               _ => {}
           }
           RefreshPayload{ success:false, token:None, refresh_token:None, errors:vec!["TOKEN_INVALID".into()] }
       }
   }
   ```

2. Add resolver to schema in `graphql/mod.rs`.

3. Tests: rotate succeeds, invalid token fails.

Commit message: "feat(gql): auth.refreshToken"
