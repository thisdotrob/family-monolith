use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::str::FromStr;

pub mod helpers;

pub async fn init(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    tracing::info!("Initializing database at {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(db_path)?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    tracing::info!("Applying database migrations from ./migrations");
    // Always run migrations on startup; they are idempotent and ensure the schema is up to date
    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
        tracing::error!("Database migration failed: {}", e);
        // Close pool before bubbling up the error
        pool.close().await;
        return Err(e.into());
    }
    tracing::info!("Database migrations complete");

    Ok(pool)
}
