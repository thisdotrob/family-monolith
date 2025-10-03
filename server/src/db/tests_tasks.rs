use super::init;

#[tokio::test]
async fn test_tasks_table_constraints() {
    let pool = init("sqlite::memory:").await.expect("db init");

    // Insert test users
    sqlx::query("INSERT INTO users (id, username, password, first_name) VALUES (?, ?, ?, ?)")
        .bind("u1")
        .bind("alice")
        .bind("password")
        .bind("Alice")
        .execute(&pool)
        .await
        .expect("insert user 1");

    sqlx::query("INSERT INTO users (id, username, password, first_name) VALUES (?, ?, ?, ?)")
        .bind("u2")
        .bind("bob")
        .bind("password")
        .bind("Bob")
        .execute(&pool)
        .await
        .expect("insert user 2");

    // Insert test project
    sqlx::query("INSERT INTO projects (id, name, owner_id) VALUES (?, ?, ?)")
        .bind("p1")
        .bind("Test Project")
        .bind("u1")
        .execute(&pool)
        .await
        .expect("insert project");

    // Insert test tag
    sqlx::query("INSERT INTO tags (id, name) VALUES (?, ?)")
        .bind("t1")
        .bind("urgent")
        .execute(&pool)
        .await
        .expect("insert tag");

    // Test valid task insertion
    sqlx::query(
        "INSERT INTO tasks (id, project_id, author_id, assignee_id, title, status) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind("task1")
    .bind("p1")
    .bind("u1")
    .bind("u2")
    .bind("Test Task")
    .bind("todo")
    .execute(&pool)
    .await
    .expect("insert valid task");

    // Test task with scheduling fields
    sqlx::query(
        "INSERT INTO tasks (id, project_id, author_id, title, status, scheduled_date, scheduled_time_minutes, deadline_date, deadline_time_minutes) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind("task2")
    .bind("p1")
    .bind("u1")
    .bind("Scheduled Task")
    .bind("todo")
    .bind("2024-12-25")
    .bind(720) // 12:00 PM
    .bind("2024-12-26")
    .bind(1080) // 6:00 PM
    .execute(&pool)
    .await
    .expect("insert scheduled task");

    // Test FK constraint: invalid project_id should fail
    let err = sqlx::query(
        "INSERT INTO tasks (id, project_id, author_id, title, status) VALUES (?, ?, ?, ?, ?)",
    )
    .bind("task_bad")
    .bind("invalid_project")
    .bind("u1")
    .bind("Bad Task")
    .bind("todo")
    .execute(&pool)
    .await
    .expect_err("expected FK violation for project_id");

    println!("FK violation error for project_id: {err}");

    // Test FK constraint: invalid author_id should fail
    let err = sqlx::query(
        "INSERT INTO tasks (id, project_id, author_id, title, status) VALUES (?, ?, ?, ?, ?)",
    )
    .bind("task_bad2")
    .bind("p1")
    .bind("invalid_user")
    .bind("Bad Task 2")
    .bind("todo")
    .execute(&pool)
    .await
    .expect_err("expected FK violation for author_id");

    println!("FK violation error for author_id: {err}");

    // Test status CHECK constraint: invalid status should fail
    let err = sqlx::query(
        "INSERT INTO tasks (id, project_id, author_id, title, status) VALUES (?, ?, ?, ?, ?)",
    )
    .bind("task_bad3")
    .bind("p1")
    .bind("u1")
    .bind("Bad Status Task")
    .bind("invalid_status")
    .execute(&pool)
    .await
    .expect_err("expected CHECK violation for status");

    println!("CHECK violation error for status: {err}");

    // Test time constraints: invalid scheduled_time_minutes should fail
    let err = sqlx::query(
        "INSERT INTO tasks (id, project_id, author_id, title, status, scheduled_time_minutes) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind("task_bad4")
    .bind("p1")
    .bind("u1")
    .bind("Bad Time Task")
    .bind("todo")
    .bind(1440) // Invalid: > 1439
    .execute(&pool)
    .await
    .expect_err("expected CHECK violation for scheduled_time_minutes");

    println!("CHECK violation error for scheduled_time_minutes: {err}");
}

#[tokio::test]
async fn test_task_tags_constraints() {
    let pool = init("sqlite::memory:").await.expect("db init");

    // Insert test data
    sqlx::query("INSERT INTO users (id, username, password, first_name) VALUES (?, ?, ?, ?)")
        .bind("u1")
        .bind("alice")
        .bind("password")
        .bind("Alice")
        .execute(&pool)
        .await
        .expect("insert user");

    sqlx::query("INSERT INTO projects (id, name, owner_id) VALUES (?, ?, ?)")
        .bind("p1")
        .bind("Test Project")
        .bind("u1")
        .execute(&pool)
        .await
        .expect("insert project");

    sqlx::query(
        "INSERT INTO tasks (id, project_id, author_id, title, status) VALUES (?, ?, ?, ?, ?)",
    )
    .bind("task1")
    .bind("p1")
    .bind("u1")
    .bind("Test Task")
    .bind("todo")
    .execute(&pool)
    .await
    .expect("insert task");

    sqlx::query("INSERT INTO tags (id, name) VALUES (?, ?)")
        .bind("t1")
        .bind("urgent")
        .execute(&pool)
        .await
        .expect("insert tag 1");

    sqlx::query("INSERT INTO tags (id, name) VALUES (?, ?)")
        .bind("t2")
        .bind("work")
        .execute(&pool)
        .await
        .expect("insert tag 2");

    // Test valid task_tag insertion
    sqlx::query("INSERT INTO task_tags (task_id, tag_id) VALUES (?, ?)")
        .bind("task1")
        .bind("t1")
        .execute(&pool)
        .await
        .expect("insert valid task_tag");

    // Test another valid task_tag insertion
    sqlx::query("INSERT INTO task_tags (task_id, tag_id) VALUES (?, ?)")
        .bind("task1")
        .bind("t2")
        .execute(&pool)
        .await
        .expect("insert second valid task_tag");

    // Test unique constraint: duplicate task_id + tag_id should fail
    let err = sqlx::query("INSERT INTO task_tags (task_id, tag_id) VALUES (?, ?)")
        .bind("task1")
        .bind("t1")
        .execute(&pool)
        .await
        .expect_err("expected uniqueness violation");

    println!("Uniqueness violation error for task_tags: {err}");

    // Test FK constraint: invalid task_id should fail
    let err = sqlx::query("INSERT INTO task_tags (task_id, tag_id) VALUES (?, ?)")
        .bind("invalid_task")
        .bind("t1")
        .execute(&pool)
        .await
        .expect_err("expected FK violation for task_id");

    println!("FK violation error for task_id: {err}");

    // Test FK constraint: invalid tag_id should fail
    let err = sqlx::query("INSERT INTO task_tags (task_id, tag_id) VALUES (?, ?)")
        .bind("task1")
        .bind("invalid_tag")
        .execute(&pool)
        .await
        .expect_err("expected FK violation for tag_id");

    println!("FK violation error for tag_id: {err}");

    // Test cascade delete: deleting task should remove task_tags
    let count_before: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM task_tags WHERE task_id = ?")
        .bind("task1")
        .fetch_one(&pool)
        .await
        .expect("count task_tags before delete");

    assert_eq!(count_before.0, 2, "Should have 2 task_tags before delete");

    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind("task1")
        .execute(&pool)
        .await
        .expect("delete task");

    let count_after: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM task_tags WHERE task_id = ?")
        .bind("task1")
        .fetch_one(&pool)
        .await
        .expect("count task_tags after delete");

    assert_eq!(
        count_after.0, 0,
        "Should have 0 task_tags after cascade delete"
    );
}
