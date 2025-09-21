use super::init;

#[tokio::test]
async fn duplicate_project_membership_fails() {
    let pool = init("sqlite::memory:").await.expect("db init");

    // Debug: list available tables
    let tables: Vec<(String,)> = sqlx::query_as("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
        .fetch_all(&pool)
        .await
        .expect("list tables");
    println!("tables: {:?}", tables);

    // Insert a user
    sqlx::query("INSERT INTO users (id, username, password, first_name) VALUES (?, ?, ?, ?)")
        .bind("u1")
        .bind("alice")
        .bind("x")
        .bind("Alice")
        .execute(&pool)
        .await
        .expect("insert user");

    // Insert a project owned by that user
    sqlx::query("INSERT INTO projects (id, name, owner_id) VALUES (?, ?, ?)")
        .bind("p1")
        .bind("Project One")
        .bind("u1")
        .execute(&pool)
        .await
        .expect("insert project");

    // First membership insert succeeds
    sqlx::query("INSERT INTO project_members (project_id, user_id) VALUES (?, ?)")
        .bind("p1")
        .bind("u1")
        .execute(&pool)
        .await
        .expect("insert membership");

    // Second identical membership should fail due to UNIQUE(project_id, user_id)
    let err = sqlx::query("INSERT INTO project_members (project_id, user_id) VALUES (?, ?)")
        .bind("p1")
        .bind("u1")
        .execute(&pool)
        .await
        .expect_err("expected uniqueness violation");

    // Optional: print error for debugging context
    println!("duplicate insert error: {err}");
}
