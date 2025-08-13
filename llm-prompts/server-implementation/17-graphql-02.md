Add login mutation.

Tasks
1. Define `graphql/auth.rs` with struct `AuthMutation`.

2. `AuthMutation` methods:
   ```rust
   #[derive(InputObject)]
   struct LoginInput { username: String, password: String }

   #[derive(SimpleObject)]
   struct LoginPayload { success: bool, token: Option<String>, refresh_token: Option<String>, errors: Vec<String> }

   #[Object]
   impl AuthMutation {
       async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> LoginPayload {
           let pool = ctx.data::<SqlitePool>().unwrap();
           if let Ok(user): Result<(String, String, String), _> = sqlx::query_as("SELECT id, username, password FROM users WHERE username = ?1")
                     .bind(&input.username.to_lowercase()).fetch_one(pool).await {
               if crate::auth::verify(&user.2, &input.password) {
                   let token = crate::auth::jwt::encode(&user.1, 24*60*60).unwrap();
                   let refresh = crate::auth::refresh::create(&user.0).await.unwrap();
                   return LoginPayload{ success:true, token:Some(token), refresh_token:Some(refresh), errors:vec![] }
               }
           }
           LoginPayload{ success:false, token:None, refresh_token:None, errors:vec!["INVALID_CREDENTIALS".into()] }
       }
   }
   ```

3. Replace previously empty Mutation in schema builder with AuthMutation.

4. Unit test happy path with in-memory DB + dummy user.

Commit message: "feat(gql): auth.login mutation"
