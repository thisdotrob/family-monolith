use crate::config::JWT_SECRET;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn encode(username: &str, exp_seconds: usize) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (chrono::Utc::now().timestamp() as usize) + exp_seconds;
    jsonwebtoken::encode(
        &Header::default(),
        &Claims {
            sub: username.to_owned(),
            exp,
        },
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

#[allow(dead_code)]
pub fn decode(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 0;

    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
}
