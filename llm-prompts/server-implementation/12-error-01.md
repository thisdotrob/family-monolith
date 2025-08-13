Introduce unified error handling.

Tasks
1. Add `error_codes.rs` enum:
   ```rust
   #[derive(Clone, Copy)]
   pub enum ErrorCode { InvalidCredentials, TokenExpired, Internal }

   impl ErrorCode {
       pub fn as_str(&self) -> &'static str { match self {
           Self::InvalidCredentials => "INVALID_CREDENTIALS",
           Self::TokenExpired => "TOKEN_EXPIRED",
           Self::Internal => "INTERNAL_ERROR",
       }}}
   }
   ```

2. Create error.rs with:
   ```rust
   use axum::{response::{IntoResponse, Response}, Json};
   use http::StatusCode;
   use serde::Serialize;
   use crate::error_codes::ErrorCode;

   #[derive(Debug)]
   pub struct AppError { pub code: ErrorCode, pub msg: String }

   #[derive(Serialize)]
   struct ErrorBody<'a> { success: bool, errors: [(&'a str, &'a str); 1] }

   impl IntoResponse for AppError {
       fn into_response(self) -> Response {
           let status = match self.code {
               ErrorCode::InvalidCredentials => StatusCode::UNAUTHORIZED,
               ErrorCode::TokenExpired => StatusCode::UNAUTHORIZED,
               _ => StatusCode::INTERNAL_SERVER_ERROR,
           };
           let body = Json(ErrorBody { success:false, errors:[(self.code.as_str(), &self.msg)]});
           (status, body).into_response()
       }
    }
   ```

3. Glob-import error::AppError at crate root.

Commit message: "feat(error): AppError with consistent codes"
