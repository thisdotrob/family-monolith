use sqlx::SqlitePool;

/// Fetch a single row and convert it to the specified type
pub async fn fetch_one<T>(pool: &SqlitePool, sql: &str, args: &[&str]) -> Result<T, sqlx::Error>
where
    T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    let mut query = sqlx::query_as::<_, T>(sql);
    for arg in args {
        query = query.bind(arg);
    }
    query.fetch_one(pool).await
}

/// Execute a query and return the number of affected rows
pub async fn execute(pool: &SqlitePool, sql: &str, args: &[&str]) -> Result<u64, sqlx::Error> {
    let mut query = sqlx::query(sql);
    for arg in args {
        query = query.bind(arg);
    }
    Ok(query.execute(pool).await?.rows_affected())
}
