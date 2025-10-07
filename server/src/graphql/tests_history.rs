#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::Claims;
    use async_graphql::{Context, Schema};
    use chrono::{Duration, Utc};
    use sqlx::SqlitePool;
    use std::sync::Arc;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        pool
    }

    async fn create_test_user(pool: &SqlitePool) -> String {
        let user_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO users (id, username, password, first_name) VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(&user_id)
        .bind("testuser")
        .bind("password_hash")
        .bind("Test User")
        .execute(pool)
        .await
        .unwrap();

        user_id
    }

    async fn create_test_project(pool: &SqlitePool, owner_id: &str) -> String {
        let project_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO projects (id, name, owner_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        )
        .bind(&project_id)
        .bind("Test Project")
        .bind(owner_id)
        .bind(Utc::now().format("%Y-%m-%d %H:%M:%S%.f").to_string())
        .bind(Utc::now().format("%Y-%m-%d %H:%M:%S%.f").to_string())
        .execute(pool)
        .await
        .unwrap();

        project_id
    }

    async fn create_test_tag(pool: &SqlitePool) -> String {
        let tag_id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO tags (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)")
            .bind(&tag_id)
            .bind("test-tag")
            .bind(Utc::now().format("%Y-%m-%d %H:%M:%S%.f").to_string())
            .bind(Utc::now().format("%Y-%m-%d %H:%M:%S%.f").to_string())
            .execute(pool)
            .await
            .unwrap();

        tag_id
    }

    async fn create_completed_task(
        pool: &SqlitePool,
        project_id: &str,
        author_id: &str,
        completed_by: &str,
        days_ago: i64,
        tag_id: Option<&str>,
    ) -> String {
        let task_id = uuid::Uuid::new_v4().to_string();
        let completed_at = (Utc::now() - Duration::days(days_ago))
            .format("%Y-%m-%d %H:%M:%S%.f")
            .to_string();

        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, title, status, completed_at, completed_by, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        )
        .bind(&task_id)
        .bind(project_id)
        .bind(author_id)
        .bind(format!("Task completed {} days ago", days_ago))
        .bind("done")
        .bind(&completed_at)
        .bind(completed_by)
        .bind(Utc::now().format("%Y-%m-%d %H:%M:%S%.f").to_string())
        .bind(Utc::now().format("%Y-%m-%d %H:%M:%S%.f").to_string())
        .execute(pool)
        .await
        .unwrap();

        // Add tag if provided
        if let Some(tag) = tag_id {
            sqlx::query("INSERT INTO task_tags (task_id, tag_id) VALUES (?1, ?2)")
                .bind(&task_id)
                .bind(tag)
                .execute(pool)
                .await
                .unwrap();
        }

        task_id
    }

    async fn create_abandoned_task(
        pool: &SqlitePool,
        project_id: &str,
        author_id: &str,
        abandoned_by: &str,
        days_ago: i64,
    ) -> String {
        let task_id = uuid::Uuid::new_v4().to_string();
        let abandoned_at = (Utc::now() - Duration::days(days_ago))
            .format("%Y-%m-%d %H:%M:%S%.f")
            .to_string();

        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, title, status, abandoned_at, abandoned_by, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        )
        .bind(&task_id)
        .bind(project_id)
        .bind(author_id)
        .bind(format!("Task abandoned {} days ago", days_ago))
        .bind("abandoned")
        .bind(&abandoned_at)
        .bind(abandoned_by)
        .bind(Utc::now().format("%Y-%m-%d %H:%M:%S%.f").to_string())
        .bind(Utc::now().format("%Y-%m-%d %H:%M:%S%.f").to_string())
        .execute(pool)
        .await
        .unwrap();

        task_id
    }

    #[tokio::test]
    async fn test_history_query_basic() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let project_id = create_test_project(&pool, &user_id).await;

        // Create some completed and abandoned tasks
        create_completed_task(&pool, &project_id, &user_id, &user_id, 1, None).await;
        create_completed_task(&pool, &project_id, &user_id, &user_id, 3, None).await;
        create_abandoned_task(&pool, &project_id, &user_id, &user_id, 2).await;

        let schema = crate::graphql::build(pool);

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        });

        let query = r#"
            query {
                history(statuses: [DONE, ABANDONED], timezone: "UTC") {
                    totalCount
                    items {
                        id
                        title
                        status
                        completedAt
                        abandonedAt
                    }
                }
            }
        "#;

        let request = async_graphql::Request::new(query).data(claims);
        let response = schema.execute(request).await;

        assert!(
            response.errors.is_empty(),
            "Query should not have errors: {:?}",
            response.errors
        );

        let data = response.data.into_json().unwrap();
        let history = &data["history"];

        assert_eq!(history["totalCount"], 3);
        assert_eq!(history["items"].as_array().unwrap().len(), 3);

        // Verify ordering - most recent first
        let items = history["items"].as_array().unwrap();
        assert!(
            items[0]["completedAt"].as_str().is_some()
                || items[0]["abandonedAt"].as_str().is_some()
        );
    }

    #[tokio::test]
    async fn test_history_query_status_filter() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let project_id = create_test_project(&pool, &user_id).await;

        // Create some completed and abandoned tasks
        create_completed_task(&pool, &project_id, &user_id, &user_id, 1, None).await;
        create_completed_task(&pool, &project_id, &user_id, &user_id, 3, None).await;
        create_abandoned_task(&pool, &project_id, &user_id, &user_id, 2).await;

        let schema = crate::graphql::build(pool);

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        });

        // Test only done tasks
        let query = r#"
            query {
                history(statuses: [DONE], timezone: "UTC") {
                    totalCount
                    items {
                        id
                        status
                    }
                }
            }
        "#;

        let request = async_graphql::Request::new(query).data(claims.clone());
        let response = schema.execute(request).await;

        assert!(response.errors.is_empty());
        let data = response.data.into_json().unwrap();
        let history = &data["history"];

        assert_eq!(history["totalCount"], 2);
        let items = history["items"].as_array().unwrap();
        for item in items {
            assert_eq!(item["status"], "DONE");
        }

        // Test only abandoned tasks
        let query = r#"
            query {
                history(statuses: [ABANDONED], timezone: "UTC") {
                    totalCount
                    items {
                        id
                        status
                    }
                }
            }
        "#;

        let request = async_graphql::Request::new(query).data(claims);
        let response = schema.execute(request).await;

        assert!(response.errors.is_empty());
        let data = response.data.into_json().unwrap();
        let history = &data["history"];

        assert_eq!(history["totalCount"], 1);
        let items = history["items"].as_array().unwrap();
        assert_eq!(items[0]["status"], "ABANDONED");
    }

    #[tokio::test]
    async fn test_history_query_project_filter() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let project1_id = create_test_project(&pool, &user_id).await;
        let project2_id = create_test_project(&pool, &user_id).await;

        // Create tasks in different projects
        create_completed_task(&pool, &project1_id, &user_id, &user_id, 1, None).await;
        create_completed_task(&pool, &project2_id, &user_id, &user_id, 2, None).await;

        let schema = crate::graphql::build(pool);

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        });

        let query = format!(
            r#"
            query {{
                history(statuses: [DONE], projectId: "{}", timezone: "UTC") {{
                    totalCount
                    items {{
                        id
                        projectId
                    }}
                }}
            }}
            "#,
            project1_id
        );

        let request = async_graphql::Request::new(query).data(claims);
        let response = schema.execute(request).await;

        assert!(response.errors.is_empty());
        let data = response.data.into_json().unwrap();
        let history = &data["history"];

        assert_eq!(history["totalCount"], 1);
        let items = history["items"].as_array().unwrap();
        assert_eq!(items[0]["projectId"], project1_id);
    }

    #[tokio::test]
    async fn test_history_query_tag_filter() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let project_id = create_test_project(&pool, &user_id).await;
        let tag_id = create_test_tag(&pool).await;

        // Create tasks with and without tags
        create_completed_task(&pool, &project_id, &user_id, &user_id, 1, Some(&tag_id)).await;
        create_completed_task(&pool, &project_id, &user_id, &user_id, 2, None).await;

        let schema = crate::graphql::build(pool);

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        });

        let query = format!(
            r#"
            query {{
                history(statuses: [DONE], tagIds: ["{}"], timezone: "UTC") {{
                    totalCount
                    items {{
                        id
                    }}
                }}
            }}
            "#,
            tag_id
        );

        let request = async_graphql::Request::new(query).data(claims);
        let response = schema.execute(request).await;

        assert!(response.errors.is_empty());
        let data = response.data.into_json().unwrap();
        let history = &data["history"];

        assert_eq!(history["totalCount"], 1);
    }

    #[tokio::test]
    async fn test_history_query_pagination() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let project_id = create_test_project(&pool, &user_id).await;

        // Create 5 completed tasks
        for i in 1..=5 {
            create_completed_task(&pool, &project_id, &user_id, &user_id, i, None).await;
        }

        let schema = crate::graphql::build(pool);

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        });

        let query = r#"
            query {
                history(statuses: [DONE], timezone: "UTC", limit: 2, offset: 0) {
                    totalCount
                    items {
                        id
                    }
                }
            }
        "#;

        let request = async_graphql::Request::new(query).data(claims);
        let response = schema.execute(request).await;

        assert!(response.errors.is_empty());
        let data = response.data.into_json().unwrap();
        let history = &data["history"];

        assert_eq!(history["totalCount"], 5);
        assert_eq!(history["items"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_history_query_date_range() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let project_id = create_test_project(&pool, &user_id).await;

        // Create tasks at different dates
        create_completed_task(&pool, &project_id, &user_id, &user_id, 1, None).await; // Recent
        create_completed_task(&pool, &project_id, &user_id, &user_id, 10, None).await; // Older

        let schema = crate::graphql::build(pool);

        let claims = Arc::new(Claims {
            sub: "testuser".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        });

        // Get date range for last 5 days
        let from_date = (Utc::now() - Duration::days(5))
            .format("%Y-%m-%d")
            .to_string();
        let to_date = Utc::now().format("%Y-%m-%d").to_string();

        let query = format!(
            r#"
            query {{
                history(statuses: [DONE], timezone: "UTC", fromDate: "{}", toDate: "{}") {{
                    totalCount
                    items {{
                        id
                    }}
                }}
            }}
            "#,
            from_date, to_date
        );

        let request = async_graphql::Request::new(query).data(claims);
        let response = schema.execute(request).await;

        assert!(response.errors.is_empty());
        let data = response.data.into_json().unwrap();
        let history = &data["history"];

        // Should only include the recent task (1 day ago), not the older one (10 days ago)
        assert_eq!(history["totalCount"], 1);
    }
}
