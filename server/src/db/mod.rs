use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::str::FromStr;

pub mod helpers;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_tags;

pub async fn init(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    tracing::info!("Initializing database at {}", db_path);
    let mut connect_options = SqliteConnectOptions::from_str(db_path)?
        .create_if_missing(true)
        .foreign_keys(true);

    let mut pool_options = SqlitePoolOptions::new();
    if db_path.contains(":memory:") {
        pool_options = pool_options.max_connections(1);
    } else {
        pool_options = pool_options.max_connections(5);
    }

    let pool = pool_options.connect_with(connect_options).await?;

    // Enforce foreign key constraints in SQLite
    if let Err(e) = sqlx::query("PRAGMA foreign_keys = ON").execute(&pool).await {
        tracing::error!("Enabling SQLite foreign_keys failed: {}", e);
        pool.close().await;
        return Err(e.into());
    }

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
