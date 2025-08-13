use crate::db;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rand::{RngCore, rngs::OsRng};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create(pool: &SqlitePool, user_id: &str) -> sqlx::Result<String> {
    let token = random_token();
    db::helpers::execute(
        pool,
        "INSERT INTO refresh_tokens (id, user_id, token) VALUES (?1, ?2, ?3)",
        &[&Uuid::new_v4().to_string(), user_id, &token],
    )
    .await?;
    Ok(token)
}

#[allow(dead_code)]
pub async fn delete(pool: &SqlitePool, token: &str) -> sqlx::Result<u64> {
    db::helpers::execute(
        pool,
        "DELETE FROM refresh_tokens WHERE token = ?1",
        &[&token],
    )
    .await
}

#[allow(dead_code)]
pub async fn rotate(pool: &SqlitePool, old: &str) -> sqlx::Result<Option<String>> {
    let result = db::helpers::fetch_one::<(String,)>(
        pool,
        "SELECT user_id FROM refresh_tokens WHERE token = ?1",
        &[old],
    )
    .await;
    if let Ok(record) = result {
        delete(pool, old).await?;
        Ok(Some(create(pool, &record.0).await?))
    } else {
        Ok(None)
    }
}

fn random_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}
