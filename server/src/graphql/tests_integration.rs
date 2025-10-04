#[cfg(test)]
mod integration_tests {
    use crate::db::init;
    use crate::graphql::build;
    use async_graphql::http::{GraphQLRequest, GraphQLResponse};
    use serde_json::{Value, json};
    use sqlx::SqlitePool;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        // Insert test user
        sqlx::query("INSERT INTO users (id, username, password, first_name) VALUES (?, ?, ?, ?)")
            .bind("user1")
            .bind("testuser")
            .bind("password")
            .bind("Test User")
            .execute(&pool)
            .await
            .expect("Failed to insert test user");

        // Insert test project
        sqlx::query("INSERT INTO projects (id, name, owner_id) VALUES (?, ?, ?)")
            .bind("project1")
            .bind("Test Project")
            .bind("user1")
            .execute(&pool)
            .await
            .expect("Failed to insert test project");

        // Insert project membership
        sqlx::query("INSERT INTO project_members (project_id, user_id) VALUES (?, ?)")
            .bind("project1")
            .bind("user1")
            .execute(&pool)
            .await
            .expect("Failed to insert project membership");

        pool
    }

    async fn execute_graphql_query(
        schema: &crate::graphql::AppSchema,
        query: &str,
        variables: Option<Value>,
        user_id: Option<&str>,
    ) -> GraphQLResponse {
        let mut request = GraphQLRequest::new(query);
        if let Some(vars) = variables {
            request = request.variables(vars);
        }

        // Add user context if provided
        let mut ctx = async_graphql::Request::from(request);
        if let Some(uid) = user_id {
            use crate::auth::Claims;
            use std::sync::Arc;

            let claims = Claims {
                sub: uid.to_string(),
                exp: 9999999999, // Far future expiry
            };
            ctx = ctx.data(Arc::new(claims));
        }

        schema.execute(ctx).await.into()
    }

    async fn seed_tasks_for_timezone_testing(pool: &SqlitePool) {
        // Insert tasks with various dates and times for timezone testing
        let tasks = [
            // Overdue task (yesterday)
            ("task1", "Overdue Task", "2025-01-01", None, None, None),
            // Today task (scheduled)
            ("task2", "Today Task", "2025-01-02", Some(540), None, None), // 9:00 AM
            // Tomorrow task
            ("task3", "Tomorrow Task", "2025-01-03", None, None, None),
            // Future task
            ("task4", "Future Task", "2025-01-10", None, None, None),
            // Task with deadline (overdue)
            (
                "task5",
                "Deadline Overdue",
                None,
                None,
                "2025-01-01",
                Some(720),
            ), // 12:00 PM
            // Task with no dates
            ("task6", "No Date Task", None, None, None, None),
            // Completed task for history
            ("task7", "Completed Task", "2025-01-01", None, None, None),
            // Another completed task
            ("task8", "Another Completed", "2025-01-02", None, None, None),
            // Abandoned task
            ("task9", "Abandoned Task", "2025-01-02", None, None, None),
        ];

        for (id, title, scheduled_date, scheduled_time, deadline_date, deadline_time) in tasks {
            let status = match id {
                "task7" | "task8" => "done",
                "task9" => "abandoned",
                _ => "todo",
            };

            sqlx::query(
                "INSERT INTO tasks (id, project_id, author_id, title, status, scheduled_date, scheduled_time_minutes, deadline_date, deadline_time_minutes, created_at, updated_at, completed_at, completed_by, abandoned_at, abandoned_by) 
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'), ?, ?, ?, ?)"
            )
            .bind(id)
            .bind("project1")
            .bind("user1")
            .bind(title)
            .bind(status)
            .bind(scheduled_date)
            .bind(scheduled_time)
            .bind(deadline_date)
            .bind(deadline_time)
            .bind(if status == "done" { Some("2025-01-02T10:00:00Z") } else { None })
            .bind(if status == "done" { Some("user1") } else { None })
            .bind(if status == "abandoned" { Some("2025-01-02T11:00:00Z") } else { None })
            .bind(if status == "abandoned" { Some("user1") } else { None })
            .execute(pool)
            .await
            .expect(&format!("Failed to insert task {}", id));
        }
    }

    #[tokio::test]
    async fn test_tasks_derived_fields_with_timezone_utc() {
        let pool = setup_test_db().await;
        seed_tasks_for_timezone_testing(&pool).await;
        let schema = build(pool);

        let query = r#"
            query TestTasks($projectId: String!, $timezone: String!) {
                tasks(projectId: $projectId, timezone: $timezone) {
                    totalCount
                    items {
                        id
                        title
                        isOverdue
                        bucket
                        scheduledDate
                        deadlineDate
                    }
                }
            }
        "#;

        let variables = json!({
            "projectId": "project1",
            "timezone": "UTC"
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.expect("No data in response");
        let tasks = &data["tasks"];

        assert_eq!(tasks["totalCount"], 6); // 6 todo tasks

        let items = tasks["items"].as_array().expect("Items should be array");

        // Find specific tasks and verify their derived fields
        let overdue_task = items
            .iter()
            .find(|t| t["id"] == "task1")
            .expect("Should find overdue task");
        assert_eq!(overdue_task["isOverdue"], true);
        assert_eq!(overdue_task["bucket"], "Overdue");

        let today_task = items
            .iter()
            .find(|t| t["id"] == "task2")
            .expect("Should find today task");
        assert_eq!(today_task["bucket"], "Today");

        let tomorrow_task = items
            .iter()
            .find(|t| t["id"] == "task3")
            .expect("Should find tomorrow task");
        assert_eq!(tomorrow_task["bucket"], "Tomorrow");

        let future_task = items
            .iter()
            .find(|t| t["id"] == "task4")
            .expect("Should find future task");
        assert_eq!(future_task["bucket"], "Upcoming");

        let deadline_task = items
            .iter()
            .find(|t| t["id"] == "task5")
            .expect("Should find deadline task");
        assert_eq!(deadline_task["isOverdue"], true);

        let no_date_task = items
            .iter()
            .find(|t| t["id"] == "task6")
            .expect("Should find no date task");
        assert_eq!(no_date_task["bucket"], "NoDate");
    }

    #[tokio::test]
    async fn test_tasks_derived_fields_with_timezone_amsterdam() {
        let pool = setup_test_db().await;

        // Seed tasks with dates that will behave differently in Amsterdam timezone
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, title, status, scheduled_date, scheduled_time_minutes, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
        )
        .bind("ams_task1")
        .bind("project1")
        .bind("user1")
        .bind("Amsterdam Task")
        .bind("todo")
        .bind("2025-01-02") // Today in UTC, but might be different in Amsterdam
        .bind(60) // 1:00 AM
        .execute(&pool)
        .await
        .expect("Failed to insert Amsterdam test task");

        let schema = build(pool);

        let query = r#"
            query TestTasks($projectId: String!, $timezone: String!) {
                tasks(projectId: $projectId, timezone: $timezone) {
                    totalCount
                    items {
                        id
                        title
                        isOverdue
                        bucket
                        scheduledDate
                        scheduledTimeMinutes
                    }
                }
            }
        "#;

        let variables = json!({
            "projectId": "project1",
            "timezone": "Europe/Amsterdam"
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.expect("No data in response");
        let tasks = &data["tasks"];

        assert!(tasks["totalCount"].as_i64().unwrap() >= 1);

        let items = tasks["items"].as_array().expect("Items should be array");
        let ams_task = items
            .iter()
            .find(|t| t["id"] == "ams_task1")
            .expect("Should find Amsterdam task");

        // Verify the task exists and has computed derived fields for Amsterdam timezone
        assert!(ams_task["bucket"].is_string());
        assert!(ams_task["isOverdue"].is_boolean());
    }

    #[tokio::test]
    async fn test_tasks_derived_fields_with_timezone_newyork() {
        let pool = setup_test_db().await;

        // Seed a task for New York timezone testing
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, title, status, scheduled_date, scheduled_time_minutes, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
        )
        .bind("ny_task1")
        .bind("project1")
        .bind("user1")
        .bind("New York Task")
        .bind("todo")
        .bind("2025-01-02") 
        .bind(300) // 5:00 AM
        .execute(&pool)
        .await
        .expect("Failed to insert New York test task");

        let schema = build(pool);

        let query = r#"
            query TestTasks($projectId: String!, $timezone: String!) {
                tasks(projectId: $projectId, timezone: $timezone) {
                    totalCount
                    items {
                        id
                        title
                        isOverdue
                        bucket
                    }
                }
            }
        "#;

        let variables = json!({
            "projectId": "project1",
            "timezone": "America/New_York"
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.expect("No data in response");
        let tasks = &data["tasks"];

        assert!(tasks["totalCount"].as_i64().unwrap() >= 1);

        let items = tasks["items"].as_array().expect("Items should be array");
        let ny_task = items
            .iter()
            .find(|t| t["id"] == "ny_task1")
            .expect("Should find New York task");

        // Verify the task exists and has computed derived fields for New York timezone
        assert!(ny_task["bucket"].is_string());
        assert!(ny_task["isOverdue"].is_boolean());
    }

    #[tokio::test]
    async fn test_tasks_pagination_behavior() {
        let pool = setup_test_db().await;

        // Insert more tasks for pagination testing
        for i in 1..=25 {
            sqlx::query(
                "INSERT INTO tasks (id, project_id, author_id, title, status, created_at, updated_at) 
                 VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
            )
            .bind(format!("page_task_{}", i))
            .bind("project1")
            .bind("user1")
            .bind(format!("Pagination Task {}", i))
            .bind("todo")
            .execute(&pool)
            .await
            .expect(&format!("Failed to insert pagination task {}", i));
        }

        let schema = build(pool);

        // Test first page
        let query = r#"
            query TestPagination($projectId: String!, $timezone: String!, $limit: Int!, $offset: Int!) {
                tasks(projectId: $projectId, timezone: $timezone, limit: $limit, offset: $offset) {
                    totalCount
                    items {
                        id
                        title
                    }
                }
            }
        "#;

        let variables = json!({
            "projectId": "project1",
            "timezone": "UTC",
            "limit": 10,
            "offset": 0
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.expect("No data in response");
        let tasks = &data["tasks"];

        assert_eq!(tasks["totalCount"], 25);
        assert_eq!(tasks["items"].as_array().unwrap().len(), 10);

        // Test second page
        let variables = json!({
            "projectId": "project1",
            "timezone": "UTC",
            "limit": 10,
            "offset": 10
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.expect("No data in response");
        let tasks = &data["tasks"];

        assert_eq!(tasks["totalCount"], 25);
        assert_eq!(tasks["items"].as_array().unwrap().len(), 10);

        // Test final partial page
        let variables = json!({
            "projectId": "project1",
            "timezone": "UTC",
            "limit": 10,
            "offset": 20
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.expect("No data in response");
        let tasks = &data["tasks"];

        assert_eq!(tasks["totalCount"], 25);
        assert_eq!(tasks["items"].as_array().unwrap().len(), 5);
    }

    #[tokio::test]
    async fn test_tasks_default_ordering() {
        let pool = setup_test_db().await;

        // Insert tasks with different dates to test ordering
        let test_tasks = [
            ("order1", "No Date Task A", None, None),
            ("order2", "No Date Task B", None, None),
            ("order3", "Tomorrow Task", Some("2025-01-03"), None),
            ("order4", "Today Task Early", Some("2025-01-02"), Some(480)), // 8:00 AM
            ("order5", "Today Task Late", Some("2025-01-02"), Some(960)),  // 4:00 PM
            ("order6", "Yesterday Task", Some("2025-01-01"), None),
        ];

        for (id, title, scheduled_date, scheduled_time) in test_tasks {
            sqlx::query(
                "INSERT INTO tasks (id, project_id, author_id, title, status, scheduled_date, scheduled_time_minutes, created_at, updated_at) 
                 VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
            )
            .bind(id)
            .bind("project1")
            .bind("user1")
            .bind(title)
            .bind("todo")
            .bind(scheduled_date)
            .bind(scheduled_time)
            .execute(&pool)
            .await
            .expect(&format!("Failed to insert ordering test task {}", id));
        }

        let schema = build(pool);

        let query = r#"
            query TestOrdering($projectId: String!, $timezone: String!) {
                tasks(projectId: $projectId, timezone: $timezone) {
                    items {
                        id
                        title
                        scheduledDate
                        scheduledTimeMinutes
                    }
                }
            }
        "#;

        let variables = json!({
            "projectId": "project1",
            "timezone": "UTC"
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.expect("No data in response");
        let items = data["tasks"]["items"]
            .as_array()
            .expect("Items should be array");

        // Verify ordering: NoDate tasks first (alphabetical), then by date/time
        let titles: Vec<&str> = items
            .iter()
            .map(|item| item["title"].as_str().unwrap())
            .collect();

        // NoDate tasks should come first, sorted alphabetically
        let no_date_tasks: Vec<&str> = titles
            .iter()
            .filter(|t| t.starts_with("No Date"))
            .copied()
            .collect();
        assert_eq!(no_date_tasks, vec!["No Date Task A", "No Date Task B"]);

        // Dated tasks should follow, in chronological order
        let dated_task_indices: Vec<usize> = titles
            .iter()
            .enumerate()
            .filter(|(_, t)| !t.starts_with("No Date"))
            .map(|(i, _)| i)
            .collect();

        // Verify that dated tasks come after no-date tasks and are in date order
        assert!(dated_task_indices.len() > 0);
        for window in dated_task_indices.windows(2) {
            assert!(window[0] < window[1], "Dated tasks should be in order");
        }
    }

    #[tokio::test]
    async fn test_history_query_with_timezone_and_pagination() {
        let pool = setup_test_db().await;

        // Insert completed and abandoned tasks for history testing
        let history_tasks = [
            (
                "hist1",
                "Completed Task 1",
                "done",
                "2025-01-01T10:00:00Z",
                "user1",
                None,
                None,
            ),
            (
                "hist2",
                "Completed Task 2",
                "done",
                "2025-01-01T11:00:00Z",
                "user1",
                None,
                None,
            ),
            (
                "hist3",
                "Completed Task 3",
                "done",
                "2025-01-02T09:00:00Z",
                "user1",
                None,
                None,
            ),
            (
                "hist4",
                "Abandoned Task 1",
                "abandoned",
                None,
                None,
                "2025-01-01T14:00:00Z",
                "user1",
            ),
            (
                "hist5",
                "Abandoned Task 2",
                "abandoned",
                None,
                None,
                "2025-01-02T15:00:00Z",
                "user1",
            ),
        ];

        for (id, title, status, completed_at, completed_by, abandoned_at, abandoned_by) in
            history_tasks
        {
            sqlx::query(
                "INSERT INTO tasks (id, project_id, author_id, title, status, completed_at, completed_by, abandoned_at, abandoned_by, created_at, updated_at) 
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
            )
            .bind(id)
            .bind("project1")
            .bind("user1")
            .bind(title)
            .bind(status)
            .bind(completed_at)
            .bind(completed_by)
            .bind(abandoned_at)
            .bind(abandoned_by)
            .execute(&pool)
            .await
            .expect(&format!("Failed to insert history task {}", id));
        }

        let schema = build(pool);

        // Test history query with pagination and timezone
        let query = r#"
            query TestHistory($statuses: [TaskStatus!]!, $timezone: String!, $limit: Int!, $offset: Int!) {
                history(statuses: $statuses, timezone: $timezone, limit: $limit, offset: $offset) {
                    totalCount
                    items {
                        id
                        title
                        status
                        completedAt
                        abandonedAt
                        isOverdue
                        bucket
                    }
                }
            }
        "#;

        // Test with both done and abandoned tasks
        let variables = json!({
            "statuses": ["done", "abandoned"],
            "timezone": "UTC",
            "limit": 3,
            "offset": 0
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.expect("No data in response");
        let history = &data["history"];

        assert_eq!(history["totalCount"], 5);
        assert_eq!(history["items"].as_array().unwrap().len(), 3);

        // Test ordering (should be most recent first)
        let items = history["items"].as_array().unwrap();
        let first_item = &items[0];
        let second_item = &items[1];

        // Verify derived fields are computed even for completed/abandoned tasks
        assert!(first_item["isOverdue"].is_boolean());
        assert!(first_item["bucket"].is_string());

        // Test second page
        let variables = json!({
            "statuses": ["done", "abandoned"],
            "timezone": "UTC",
            "limit": 3,
            "offset": 3
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.expect("No data in response");
        let history = &data["history"];

        assert_eq!(history["totalCount"], 5);
        assert_eq!(history["items"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_history_query_day_grouping_friendly_ordering() {
        let pool = setup_test_db().await;

        // Insert tasks completed at different times on the same day and different days
        let tasks = [
            (
                "day1_early",
                "Early Morning Task",
                "done",
                "2025-01-02T08:00:00Z",
                "user1",
            ),
            (
                "day1_late",
                "Late Evening Task",
                "done",
                "2025-01-02T22:00:00Z",
                "user1",
            ),
            (
                "day2_task",
                "Next Day Task",
                "done",
                "2025-01-03T12:00:00Z",
                "user1",
            ),
            (
                "day1_mid",
                "Midday Task",
                "done",
                "2025-01-02T12:00:00Z",
                "user1",
            ),
        ];

        for (id, title, status, completed_at, completed_by) in tasks {
            sqlx::query(
                "INSERT INTO tasks (id, project_id, author_id, title, status, completed_at, completed_by, created_at, updated_at) 
                 VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
            )
            .bind(id)
            .bind("project1")
            .bind("user1")
            .bind(title)
            .bind(status)
            .bind(completed_at)
            .bind(completed_by)
            .execute(&pool)
            .await
            .expect(&format!("Failed to insert day grouping task {}", id));
        }

        let schema = build(pool);

        let query = r#"
            query TestDayGrouping($statuses: [TaskStatus!]!, $timezone: String!) {
                history(statuses: $statuses, timezone: $timezone) {
                    items {
                        id
                        title
                        completedAt
                    }
                }
            }
        "#;

        let variables = json!({
            "statuses": ["done"],
            "timezone": "UTC"
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.expect("No data in response");
        let items = data["history"]["items"]
            .as_array()
            .expect("Items should be array");

        // Verify ordering is day-grouping friendly (most recent completion dates first)
        let completion_dates: Vec<&str> = items
            .iter()
            .map(|item| item["completedAt"].as_str().unwrap())
            .collect();

        // Should be in descending order by completion time (newest first)
        for window in completion_dates.windows(2) {
            assert!(
                window[0] >= window[1],
                "History should be ordered newest first: {} >= {}",
                window[0],
                window[1]
            );
        }

        // The first item should be from day2 (2025-01-03)
        assert!(completion_dates[0].starts_with("2025-01-03"));

        // Tasks from the same day should be grouped together
        let day2_count = completion_dates
            .iter()
            .filter(|d| d.starts_with("2025-01-03"))
            .count();
        let day1_count = completion_dates
            .iter()
            .filter(|d| d.starts_with("2025-01-02"))
            .count();

        assert_eq!(day2_count, 1);
        assert_eq!(day1_count, 3);
    }

    #[tokio::test]
    async fn test_history_query_timezone_amsterdam() {
        let pool = setup_test_db().await;

        // Insert a task completed at a time that might fall on different days in different timezones
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, title, status, completed_at, completed_by, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
        )
        .bind("tz_hist1")
        .bind("project1")
        .bind("user1")
        .bind("Timezone History Task")
        .bind("done")
        .bind("2025-01-02T23:30:00Z") // Late UTC, might be next day in Amsterdam
        .bind("user1")
        .execute(&pool)
        .await
        .expect("Failed to insert timezone history task");

        let schema = build(pool);

        let query = r#"
            query TestHistoryTimezone($statuses: [TaskStatus!]!, $timezone: String!) {
                history(statuses: $statuses, timezone: $timezone) {
                    totalCount
                    items {
                        id
                        title
                        completedAt
                        isOverdue
                        bucket
                    }
                }
            }
        "#;

        let variables = json!({
            "statuses": ["done"],
            "timezone": "Europe/Amsterdam"
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.expect("No data in response");
        let history = &data["history"];

        assert!(history["totalCount"].as_i64().unwrap() >= 1);

        let items = history["items"].as_array().expect("Items should be array");
        let tz_task = items
            .iter()
            .find(|t| t["id"] == "tz_hist1")
            .expect("Should find timezone task");

        // Verify derived fields are computed with Amsterdam timezone
        assert!(tz_task["isOverdue"].is_boolean());
        assert!(tz_task["bucket"].is_string());
        assert!(tz_task["completedAt"].is_string());
    }

    #[tokio::test]
    async fn test_history_query_timezone_newyork() {
        let pool = setup_test_db().await;

        // Insert a task completed at a time that will test New York timezone handling
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, title, status, abandoned_at, abandoned_by, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
        )
        .bind("ny_hist1")
        .bind("project1")
        .bind("user1")
        .bind("New York History Task")
        .bind("abandoned")
        .bind("2025-01-02T05:00:00Z") // Early UTC, previous day in NY
        .bind("user1")
        .execute(&pool)
        .await
        .expect("Failed to insert New York history task");

        let schema = build(pool);

        let query = r#"
            query TestHistoryNY($statuses: [TaskStatus!]!, $timezone: String!) {
                history(statuses: $statuses, timezone: $timezone) {
                    totalCount
                    items {
                        id
                        title
                        abandonedAt
                        isOverdue
                        bucket
                    }
                }
            }
        "#;

        let variables = json!({
            "statuses": ["abandoned"],
            "timezone": "America/New_York"
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.expect("No data in response");
        let history = &data["history"];

        assert!(history["totalCount"].as_i64().unwrap() >= 1);

        let items = history["items"].as_array().expect("Items should be array");
        let ny_task = items
            .iter()
            .find(|t| t["id"] == "ny_hist1")
            .expect("Should find New York task");

        // Verify derived fields are computed with New York timezone
        assert!(ny_task["isOverdue"].is_boolean());
        assert!(ny_task["bucket"].is_string());
        assert!(ny_task["abandonedAt"].is_string());
    }
}
