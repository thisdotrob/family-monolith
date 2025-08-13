//! Database migration CLI tool
//!
//! Usage: cargo run --bin migrate

use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Print current directory for debugging
    println!("Current directory: {:?}", std::env::current_dir()?);

    // Ensure the database directory exists
    if let Some(parent) = Path::new(monolith_backend::config::DB_PATH).parent() {
        if !parent.exists() {
            println!("Creating directory: {:?}", parent);
            fs::create_dir_all(parent)?;
        }
    }

    println!("Using database at: {}", monolith_backend::config::DB_PATH);

    // Touch the database file to ensure it exists
    if !Path::new(monolith_backend::config::DB_PATH).exists() {
        println!("Creating empty database file");
        fs::File::create(monolith_backend::config::DB_PATH)?;
    }

    let pool = monolith_backend::db::init(monolith_backend::config::DB_PATH).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("✅ Migrations complete");
    Ok(())
}
