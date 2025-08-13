use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub mod helpers;

pub async fn init(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path)
        .await
}
