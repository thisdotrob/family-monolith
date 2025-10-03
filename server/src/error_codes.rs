#[derive(Clone, Copy, Debug)]
pub enum ErrorCode {
    InvalidCredentials,
    TokenExpired,
    ValidationFailed,
    NotFound,
    Internal,
    PermissionDenied,
    ConflictStaleWrite,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::TokenExpired => "TOKEN_EXPIRED",
            Self::ValidationFailed => "VALIDATION_FAILED",
            Self::NotFound => "NOT_FOUND",
            Self::Internal => "INTERNAL_ERROR",
            Self::PermissionDenied => "PERMISSION_DENIED",
            Self::ConflictStaleWrite => "CONFLICT_STALE_WRITE",
        }
    }
}
