#[derive(Clone, Copy, Debug)]
pub enum ErrorCode {
    InvalidCredentials,
    TokenExpired,
    Internal,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::TokenExpired => "TOKEN_EXPIRED",
            Self::Internal => "INTERNAL_ERROR",
        }
    }
}
