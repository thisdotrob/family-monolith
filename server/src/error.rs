use crate::error_codes::ErrorCode;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub struct AppError {
    pub code: ErrorCode,
    pub msg: String,
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    success: bool,
    errors: [(&'a str, &'a str); 1],
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self.code {
            ErrorCode::InvalidCredentials => StatusCode::UNAUTHORIZED,
            ErrorCode::TokenExpired => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Json(ErrorBody {
            success: false,
            errors: [(self.code.as_str(), &self.msg)],
        });
        (status, body).into_response()
    }
}
