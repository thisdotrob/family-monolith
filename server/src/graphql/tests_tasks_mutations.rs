#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::Claims;
    use crate::tasks::{Task, TaskStatus};
    use async_graphql::{Request, Schema, Variables};
    use serde_json::json;
    use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
    use std::str::FromStr;
    use std::sync::Arc;

    async fn create_test_pool() -> SqlitePool {
        let options = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true)
            .foreign_keys(true);

        let pool = SqlitePool::connect_with(options).await.unwrap();

        // Apply all migrations by running them manually
        sqlx::query(include_str!(
            "../../migrations/20250101010101_create_users.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(include_str!(
            "../../migrations/20250101010202_create_refresh_tokens.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(include_str!(
            "../../migrations/20250101010303_create_projects.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(include_str!(
            "../../migrations/20250101010304_create_tags.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(include_str!(
            "../../migrations/20250101010404_create_project_members.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(include_str!(
            "../../migrations/20250101010505_create_tasks.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(include_str!(
            "../../migrations/20250101010506_create_task_tags.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    async fn setup_test_data(pool: &SqlitePool) -> (String, String, String, String) {
        let test_suffix = uuid::Uuid::new_v4().to_string()[..8].to_string();
        let user_id = format!("user_id_{}", test_suffix);
        let project_id = format!("project_id_{}", test_suffix);
        let tag_id = format!("tag_id_{}", test_suffix);

        // Insert user
        sqlx::query("INSERT INTO users (id, username, password, first_name) VALUES (?, ?, ?, ?)")
            .bind(&user_id)
            .bind("testuser")
            .bind("password")
            .bind("Test User")
            .execute(pool)
            .await
            .unwrap();

        // Insert project owned by user
        sqlx::query("INSERT INTO projects (id, name, owner_id) VALUES (?, ?, ?)")
            .bind(&project_id)
            .bind("Test Project")
            .bind(&user_id)
            .execute(pool)
            .await
            .unwrap();

        // Insert tag
        sqlx::query("INSERT INTO tags (id, name) VALUES (?, ?)")
            .bind(&tag_id)
            .bind("test-tag")
            .execute(pool)
            .await
            .unwrap();

        let task_id = format!("task_id_{}", test_suffix);
        (user_id, project_id, tag_id, task_id)
    }

    #[tokio::test]
    async fn test_create_task_success() {
        let pool = create_test_pool().await;
        let (user_id, project_id, tag_id, _task_id) = setup_test_data(&pool).await;

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: chrono::Utc::now().timestamp() + 3600,
        });

        let schema = crate::graphql::build(pool);

        let query = r#"
            mutation CreateTask($input: CreateTaskInput!) {
                createTask(input: $input) {
                    id
                    title
                    description
                    status
                    projectId
                    authorId
                    assigneeId
                }
            }
        "#;

        let variables = json!({
            "input": {
                "projectId": project_id,
                "title": "Test Task",
                "description": "Test Description",
                "tagIds": [tag_id]
            }
        });

        let req = Request::new(query)
            .variables(Variables::from_json(variables))
            .data(claims);

        let response = schema.execute(req).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.into_json().unwrap();
        let task = data.get("createTask").unwrap();

        assert_eq!(task.get("title").unwrap(), "Test Task");
        assert_eq!(task.get("description").unwrap(), "Test Description");
        assert_eq!(task.get("status").unwrap(), "TODO");
        assert_eq!(task.get("projectId").unwrap(), &project_id);
        assert_eq!(task.get("authorId").unwrap(), &user_id);
        assert_eq!(task.get("assigneeId").unwrap(), &user_id); // Default to creator
    }

    #[tokio::test]
    async fn test_create_task_validation_errors() {
        let pool = create_test_pool().await;
        let (user_id, project_id, _tag_id, _task_id) = setup_test_data(&pool).await;

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: chrono::Utc::now().timestamp() + 3600,
        });

        let schema = crate::graphql::build(pool);

        // Test empty title
        let query = r#"
            mutation CreateTask($input: CreateTaskInput!) {
                createTask(input: $input) {
                    id
                }
            }
        "#;

        let variables = json!({
            "input": {
                "projectId": project_id,
                "title": "",
            }
        });

        let req = Request::new(query)
            .variables(Variables::from_json(variables))
            .data(claims.clone());

        let response = schema.execute(req).await;
        assert!(!response.errors.is_empty());
        assert!(response.errors[0].message.contains("Title cannot be empty"));

        // Test title too long
        let long_title = "a".repeat(121);
        let variables = json!({
            "input": {
                "projectId": project_id,
                "title": long_title,
            }
        });

        let req = Request::new(query)
            .variables(Variables::from_json(variables))
            .data(claims);

        let response = schema.execute(req).await;
        assert!(!response.errors.is_empty());
        assert!(
            response.errors[0]
                .message
                .contains("Title cannot exceed 120 characters")
        );
    }

    #[tokio::test]
    async fn test_complete_task_success() {
        let pool = create_test_pool().await;
        let (user_id, project_id, _tag_id, task_id) = setup_test_data(&pool).await;

        // Insert a todo task
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, assignee_id, title, status) 
             VALUES (?, ?, ?, ?, ?, 'todo')",
        )
        .bind(&task_id)
        .bind(&project_id)
        .bind(&user_id)
        .bind(&user_id)
        .bind("Test Task")
        .execute(&pool)
        .await
        .unwrap();

        // Get the updated_at timestamp
        let updated_at: String =
            sqlx::query_as::<_, (String,)>("SELECT updated_at FROM tasks WHERE id = ?1")
                .bind(&task_id)
                .fetch_one(&pool)
                .await
                .unwrap()
                .0;

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: chrono::Utc::now().timestamp() + 3600,
        });

        let schema = crate::graphql::build(pool);

        let query = r#"
            mutation CompleteTask($id: String!, $lastKnownUpdatedAt: String!) {
                completeTask(id: $id, lastKnownUpdatedAt: $lastKnownUpdatedAt) {
                    id
                    status
                    completedAt
                    completedBy
                }
            }
        "#;

        let variables = json!({
            "id": task_id,
            "lastKnownUpdatedAt": updated_at
        });

        let req = Request::new(query)
            .variables(Variables::from_json(variables))
            .data(claims);

        let response = schema.execute(req).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.into_json().unwrap();
        let task = data.get("completeTask").unwrap();

        assert_eq!(task.get("id").unwrap(), &task_id);
        assert_eq!(task.get("status").unwrap(), "DONE");
        assert!(task.get("completedAt").unwrap().is_string());
        assert_eq!(task.get("completedBy").unwrap(), &user_id);
    }

    #[tokio::test]
    async fn test_complete_task_concurrency_error() {
        let pool = create_test_pool().await;
        let (user_id, project_id, _tag_id, task_id) = setup_test_data(&pool).await;

        // Insert a todo task
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, assignee_id, title, status) 
             VALUES (?, ?, ?, ?, ?, 'todo')",
        )
        .bind(&task_id)
        .bind(&project_id)
        .bind(&user_id)
        .bind(&user_id)
        .bind("Test Task")
        .execute(&pool)
        .await
        .unwrap();

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: chrono::Utc::now().timestamp() + 3600,
        });

        let schema = crate::graphql::build(pool);

        let query = r#"
            mutation CompleteTask($id: String!, $lastKnownUpdatedAt: String!) {
                completeTask(id: $id, lastKnownUpdatedAt: $lastKnownUpdatedAt) {
                    id
                }
            }
        "#;

        let variables = json!({
            "id": task_id,
            "lastKnownUpdatedAt": "2020-01-01 00:00:00.000"  // Stale timestamp
        });

        let req = Request::new(query)
            .variables(Variables::from_json(variables))
            .data(claims);

        let response = schema.execute(req).await;
        assert!(!response.errors.is_empty());
        assert!(
            response.errors[0]
                .message
                .contains("modified by another user")
        );

        // Check for the correct error code extension
        let extensions = response.errors[0].extensions.as_ref().unwrap();
        assert_eq!(extensions.get("code").unwrap(), "CONFLICT_STALE_WRITE");
    }

    #[tokio::test]
    async fn test_status_transition_rules() {
        let pool = create_test_pool().await;
        let (user_id, project_id, _tag_id, task_id) = setup_test_data(&pool).await;

        // Insert a done task
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, assignee_id, title, status, completed_at, completed_by) 
             VALUES (?, ?, ?, ?, ?, 'done', CURRENT_TIMESTAMP, ?)"
        )
        .bind(&task_id)
        .bind(&project_id)
        .bind(&user_id)
        .bind(&user_id)
        .bind("Test Task")
        .bind(&user_id)
        .execute(&pool)
        .await
        .unwrap();

        let updated_at: String =
            sqlx::query_as::<_, (String,)>("SELECT updated_at FROM tasks WHERE id = ?1")
                .bind(&task_id)
                .fetch_one(&pool)
                .await
                .unwrap()
                .0;

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: chrono::Utc::now().timestamp() + 3600,
        });

        let schema = crate::graphql::build(pool);

        // Try to complete an already done task
        let query = r#"
            mutation CompleteTask($id: String!, $lastKnownUpdatedAt: String!) {
                completeTask(id: $id, lastKnownUpdatedAt: $lastKnownUpdatedAt) {
                    id
                }
            }
        "#;

        let variables = json!({
            "id": task_id,
            "lastKnownUpdatedAt": updated_at
        });

        let req = Request::new(query)
            .variables(Variables::from_json(variables))
            .data(claims);

        let response = schema.execute(req).await;
        assert!(!response.errors.is_empty());
        assert!(
            response.errors[0]
                .message
                .contains("Can only complete todo tasks")
        );
    }

    #[tokio::test]
    async fn test_restore_task_success() {
        let pool = create_test_pool().await;
        let (user_id, project_id, _tag_id, task_id) = setup_test_data(&pool).await;

        // Insert an abandoned task
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, assignee_id, title, status, abandoned_at, abandoned_by) 
             VALUES (?, ?, ?, ?, ?, 'abandoned', CURRENT_TIMESTAMP, ?)"
        )
        .bind(&task_id)
        .bind(&project_id)
        .bind(&user_id)
        .bind(&user_id)
        .bind("Test Task")
        .bind(&user_id)
        .execute(&pool)
        .await
        .unwrap();

        let updated_at: String =
            sqlx::query_as::<_, (String,)>("SELECT updated_at FROM tasks WHERE id = ?1")
                .bind(&task_id)
                .fetch_one(&pool)
                .await
                .unwrap()
                .0;

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: chrono::Utc::now().timestamp() + 3600,
        });

        let schema = crate::graphql::build(pool);

        let query = r#"
            mutation RestoreTask($id: String!, $lastKnownUpdatedAt: String!) {
                restoreTask(id: $id, lastKnownUpdatedAt: $lastKnownUpdatedAt) {
                    id
                    status
                    abandonedAt
                    abandonedBy
                }
            }
        "#;

        let variables = json!({
            "id": task_id,
            "lastKnownUpdatedAt": updated_at
        });

        let req = Request::new(query)
            .variables(Variables::from_json(variables))
            .data(claims);

        let response = schema.execute(req).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.into_json().unwrap();
        let task = data.get("restoreTask").unwrap();

        assert_eq!(task.get("id").unwrap(), &task_id);
        assert_eq!(task.get("status").unwrap(), "TODO");
        assert!(task.get("abandonedAt").unwrap().is_null());
        assert!(task.get("abandonedBy").unwrap().is_null());
    }

    #[tokio::test]
    async fn test_archived_project_write_rejection() {
        let pool = create_test_pool().await;
        let (user_id, project_id, _tag_id, _task_id) = setup_test_data(&pool).await;

        // Archive the project
        sqlx::query("UPDATE projects SET archived_at = CURRENT_TIMESTAMP WHERE id = ?1")
            .bind(&project_id)
            .execute(&pool)
            .await
            .unwrap();

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: chrono::Utc::now().timestamp() + 3600,
        });

        let schema = crate::graphql::build(pool);

        let query = r#"
            mutation CreateTask($input: CreateTaskInput!) {
                createTask(input: $input) {
                    id
                }
            }
        "#;

        let variables = json!({
            "input": {
                "projectId": project_id,
                "title": "Test Task"
            }
        });

        let req = Request::new(query)
            .variables(Variables::from_json(variables))
            .data(claims);

        let response = schema.execute(req).await;
        assert!(!response.errors.is_empty());
        assert!(
            response.errors[0]
                .message
                .contains("Cannot create tasks in archived project")
        );

        let extensions = response.errors[0].extensions.as_ref().unwrap();
        assert_eq!(extensions.get("code").unwrap(), "PERMISSION_DENIED");
    }
}
