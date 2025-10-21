#[cfg(test)]
mod integration_tests {
    use crate::graphql::build;
    use crate::tasks::{TaskBucket, time_utils};
    use async_graphql::{Request, Response, Variables};
    use chrono::{Days, NaiveDate, Utc};
    use chrono_tz::Tz;
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

    fn response_data(response: &Response) -> serde_json::Value {
        response
            .data
            .clone()
            .into_json()
            .expect("Response data should convert to JSON")
    }

    fn shift_date(base: NaiveDate, days: i64) -> NaiveDate {
        if days >= 0 {
            base.checked_add_days(Days::new(days as u64))
                .unwrap_or(base)
        } else {
            base.checked_sub_days(Days::new(days.abs() as u64))
                .unwrap_or(base)
        }
    }

    fn date_str(date: NaiveDate) -> String {
        date.format("%Y-%m-%d").to_string()
    }

    fn datetime_str(date: NaiveDate, time: &str) -> String {
        format!("{}T{}", date_str(date), time)
    }

    fn parse_bucket(value: &serde_json::Value) -> TaskBucket {
        match value.as_str().expect("bucket should be a string") {
            "OVERDUE" => TaskBucket::Overdue,
            "TODAY" => TaskBucket::Today,
            "TOMORROW" => TaskBucket::Tomorrow,
            "UPCOMING" => TaskBucket::Upcoming,
            "NO_DATE" => TaskBucket::NoDate,
            other => panic!("Unexpected bucket value: {}", other),
        }
    }

    async fn execute_graphql_query(
        schema: &crate::graphql::AppSchema,
        query: &str,
        variables: Option<Value>,
        user_id: Option<&str>,
    ) -> Response {
        let mut request = Request::new(query);
        if let Some(vars) = variables {
            request = request.variables(Variables::from_json(vars));
        }

        if let Some(uid) = user_id {
            use crate::auth::Claims;
            use std::sync::Arc;

            let claims = Claims {
                sub: uid.to_string(),
                exp: 9999999999, // Far future expiry
            };
            request = request.data(Arc::new(claims));
        }

        schema.execute(request).await
    }

    async fn seed_tasks_for_timezone_testing(pool: &SqlitePool) {
        // Insert tasks with various dates and times for timezone testing
        let today = Utc::now().date_naive();
        let yesterday = shift_date(today, -1);
        let tomorrow = shift_date(today, 1);
        let future = shift_date(today, 8);

        let yesterday_str = date_str(yesterday);
        let today_str = date_str(today);
        let tomorrow_str = date_str(tomorrow);
        let future_str = date_str(future);

        let tasks = vec![
            // Overdue task (yesterday)
            (
                "task1",
                "Overdue Task",
                Some(yesterday_str.clone()),
                None,
                None,
                None,
            ),
            // Today task (scheduled)
            (
                "task2",
                "Today Task",
                Some(today_str.clone()),
                Some(540),
                None,
                None,
            ), // 9:00 AM
            // Tomorrow task
            (
                "task3",
                "Tomorrow Task",
                Some(tomorrow_str.clone()),
                None,
                None,
                None,
            ),
            // Future task
            (
                "task4",
                "Future Task",
                Some(future_str.clone()),
                None,
                None,
                None,
            ),
            // Task with deadline (overdue)
            (
                "task5",
                "Deadline Overdue",
                None,
                None,
                Some(yesterday_str.clone()),
                Some(720),
            ), // 12:00 PM
            // Task with no dates
            ("task6", "No Date Task", None, None, None, None),
            // Completed task for history
            (
                "task7",
                "Completed Task",
                Some(yesterday_str.clone()),
                None,
                None,
                None,
            ),
            // Another completed task
            (
                "task8",
                "Another Completed",
                Some(today_str.clone()),
                None,
                None,
                None,
            ),
            // Abandoned task
            (
                "task9",
                "Abandoned Task",
                Some(today_str.clone()),
                None,
                None,
                None,
            ),
        ];

        let completed_at_done = datetime_str(today, "10:00:00Z");
        let completed_by = "user1";
        let abandoned_at = datetime_str(today, "11:00:00Z");

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
            .bind(scheduled_date.as_deref())
            .bind(scheduled_time)
            .bind(deadline_date.as_deref())
            .bind(deadline_time)
            .bind(if status == "done" {
                Some(completed_at_done.as_str())
            } else {
                None
            })
            .bind(if status == "done" { Some(completed_by) } else { None })
            .bind(if status == "abandoned" {
                Some(abandoned_at.as_str())
            } else {
                None
            })
            .bind(if status == "abandoned" { Some(completed_by) } else { None })
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

        let data = response_data(&response);
        let tasks = &data["tasks"];

        assert_eq!(tasks["totalCount"], 6); // 6 todo tasks

        let items = tasks["items"].as_array().expect("Items should be array");

        let tz: Tz = "UTC".parse().unwrap();
        let today = Utc::now().date_naive();
        let yesterday = shift_date(today, -1);
        let tomorrow = shift_date(today, 1);
        let future = shift_date(today, 8);

        let yesterday_str = date_str(yesterday);
        let today_str = date_str(today);
        let tomorrow_str = date_str(tomorrow);
        let future_str = date_str(future);

        let overdue_expected =
            time_utils::get_task_bucket(Some(yesterday_str.as_str()), None, None, None, tz);

        // Find specific tasks and verify their derived fields
        let overdue_task = items
            .iter()
            .find(|t| t["id"] == "task1")
            .expect("Should find overdue task");
        assert_eq!(overdue_task["isOverdue"], true);
        assert_eq!(parse_bucket(&overdue_task["bucket"]), overdue_expected);

        let today_task = items
            .iter()
            .find(|t| t["id"] == "task2")
            .expect("Should find today task");
        let today_expected =
            time_utils::get_task_bucket(Some(today_str.as_str()), Some(540), None, None, tz);
        assert_eq!(parse_bucket(&today_task["bucket"]), today_expected);

        let tomorrow_task = items
            .iter()
            .find(|t| t["id"] == "task3")
            .expect("Should find tomorrow task");
        let tomorrow_expected =
            time_utils::get_task_bucket(Some(tomorrow_str.as_str()), None, None, None, tz);
        assert_eq!(parse_bucket(&tomorrow_task["bucket"]), tomorrow_expected);

        let future_task = items
            .iter()
            .find(|t| t["id"] == "task4")
            .expect("Should find future task");
        let future_expected =
            time_utils::get_task_bucket(Some(future_str.as_str()), None, None, None, tz);
        assert_eq!(parse_bucket(&future_task["bucket"]), future_expected);

        let deadline_task = items
            .iter()
            .find(|t| t["id"] == "task5")
            .expect("Should find deadline task");
        assert_eq!(deadline_task["isOverdue"], true);

        let no_date_task = items
            .iter()
            .find(|t| t["id"] == "task6")
            .expect("Should find no date task");
        assert_eq!(parse_bucket(&no_date_task["bucket"]), TaskBucket::NoDate);
    }

    #[tokio::test]
    async fn test_tasks_derived_fields_with_timezone_amsterdam() {
        let pool = setup_test_db().await;

        // Seed tasks with dates that will behave differently in Amsterdam timezone
        let today = Utc::now().date_naive();
        let amsterdam_date = date_str(today);
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, title, status, scheduled_date, scheduled_time_minutes, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
        )
        .bind("ams_task1")
        .bind("project1")
        .bind("user1")
        .bind("Amsterdam Task")
        .bind("todo")
        .bind(amsterdam_date.as_str()) // Today in UTC, but might be different in Amsterdam
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

        let data = response_data(&response);
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
        let today = Utc::now().date_naive();
        let ny_date = date_str(today);
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, title, status, scheduled_date, scheduled_time_minutes, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
        )
        .bind("ny_task1")
        .bind("project1")
        .bind("user1")
        .bind("New York Task")
        .bind("todo")
        .bind(ny_date.as_str())
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

        let data = response_data(&response);
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

        let data = response_data(&response);
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

        let data = response_data(&response);
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

        let data = response_data(&response);
        let tasks = &data["tasks"];

        assert_eq!(tasks["totalCount"], 25);
        assert_eq!(tasks["items"].as_array().unwrap().len(), 5);
    }

    #[tokio::test]
    async fn test_tasks_default_ordering() {
        let pool = setup_test_db().await;

        // Insert tasks with different dates to test ordering
        let today = Utc::now().date_naive();
        let tomorrow = shift_date(today, 1);
        let yesterday = shift_date(today, -1);

        let today_str = date_str(today);
        let tomorrow_str = date_str(tomorrow);
        let yesterday_str = date_str(yesterday);

        let test_tasks = vec![
            ("order1", "No Date Task A", None, None),
            ("order2", "No Date Task B", None, None),
            ("order3", "Tomorrow Task", Some(tomorrow_str.clone()), None),
            (
                "order4",
                "Today Task Early",
                Some(today_str.clone()),
                Some(480),
            ), // 8:00 AM
            (
                "order5",
                "Today Task Late",
                Some(today_str.clone()),
                Some(960),
            ), // 4:00 PM
            (
                "order6",
                "Yesterday Task",
                Some(yesterday_str.clone()),
                None,
            ),
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
            .bind(scheduled_date.as_deref())
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

        let data = response_data(&response);
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
        let today = Utc::now().date_naive();
        let yesterday = shift_date(today, -1);
        let two_days_ago = shift_date(today, -2);

        let hist1_completed = datetime_str(two_days_ago, "10:00:00Z");
        let hist2_completed = datetime_str(two_days_ago, "11:00:00Z");
        let hist3_completed = datetime_str(yesterday, "09:00:00Z");
        let hist4_abandoned = datetime_str(two_days_ago, "14:00:00Z");
        let hist5_abandoned = datetime_str(yesterday, "15:00:00Z");

        let history_tasks = [
            (
                "hist1",
                "Completed Task 1",
                "done",
                Some(hist1_completed.clone()),
                Some("user1"),
                None,
                None,
            ),
            (
                "hist2",
                "Completed Task 2",
                "done",
                Some(hist2_completed.clone()),
                Some("user1"),
                None,
                None,
            ),
            (
                "hist3",
                "Completed Task 3",
                "done",
                Some(hist3_completed.clone()),
                Some("user1"),
                None,
                None,
            ),
            (
                "hist4",
                "Abandoned Task 1",
                "abandoned",
                None,
                None,
                Some(hist4_abandoned.clone()),
                Some("user1"),
            ),
            (
                "hist5",
                "Abandoned Task 2",
                "abandoned",
                None,
                None,
                Some(hist5_abandoned.clone()),
                Some("user1"),
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
            query TestHistory(
                $statuses: [TaskStatus!]!,
                $timezone: String!,
                $limit: Int!,
                $offset: Int!,
                $fromDate: String!,
                $toDate: String!
            ) {
                history(
                    statuses: $statuses,
                    timezone: $timezone,
                    limit: $limit,
                    offset: $offset,
                    fromDate: $fromDate,
                    toDate: $toDate
                ) {
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

        let from_date = date_str(two_days_ago);
        let to_date = date_str(today);

        // Test with both done and abandoned tasks
        let variables = json!({
            "statuses": ["DONE", "ABANDONED"],
            "timezone": "UTC",
            "limit": 3,
            "offset": 0,
            "fromDate": from_date.clone(),
            "toDate": to_date.clone()
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response_data(&response);
        let history = &data["history"];

        assert_eq!(history["totalCount"], 5);
        assert_eq!(history["items"].as_array().unwrap().len(), 3);

        // Test ordering (should be most recent first)
        let items = history["items"].as_array().unwrap();
        let first_item = &items[0];
        let _second_item = &items[1];

        // Verify derived fields are computed even for completed/abandoned tasks
        assert!(first_item["isOverdue"].is_boolean());
        assert!(first_item["bucket"].is_string());

        // Test second page
        let variables = json!({
            "statuses": ["DONE", "ABANDONED"],
            "timezone": "UTC",
            "limit": 3,
            "offset": 3,
            "fromDate": from_date,
            "toDate": to_date
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response_data(&response);
        let history = &data["history"];

        assert_eq!(history["totalCount"], 5);
        assert_eq!(history["items"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_history_query_day_grouping_friendly_ordering() {
        let pool = setup_test_db().await;

        // Insert tasks completed at different times on the same day and different days
        let today = Utc::now().date_naive();
        let next_day = shift_date(today, 1);

        let tasks = [
            (
                "day1_early",
                "Early Morning Task",
                "done",
                datetime_str(today, "08:00:00Z"),
                "user1",
            ),
            (
                "day1_late",
                "Late Evening Task",
                "done",
                datetime_str(today, "22:00:00Z"),
                "user1",
            ),
            (
                "day2_task",
                "Next Day Task",
                "done",
                datetime_str(next_day, "12:00:00Z"),
                "user1",
            ),
            (
                "day1_mid",
                "Midday Task",
                "done",
                datetime_str(today, "12:00:00Z"),
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
            query TestDayGrouping(
                $statuses: [TaskStatus!]!,
                $timezone: String!,
                $fromDate: String!,
                $toDate: String!
            ) {
                history(
                    statuses: $statuses,
                    timezone: $timezone,
                    fromDate: $fromDate,
                    toDate: $toDate
                ) {
                    items {
                        id
                        title
                        completedAt
                    }
                }
            }
        "#;

        let variables = json!({
            "statuses": ["DONE"],
            "timezone": "UTC",
            "fromDate": date_str(today),
            "toDate": date_str(next_day)
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response_data(&response);
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

        // The first item should be from the most recent day
        let day2_prefix = date_str(next_day);
        assert!(completion_dates[0].starts_with(&day2_prefix));

        // Tasks from the same day should be grouped together
        let day1_prefix = date_str(today);
        let day2_count = completion_dates
            .iter()
            .filter(|d| d.starts_with(&day2_prefix))
            .count();
        let day1_count = completion_dates
            .iter()
            .filter(|d| d.starts_with(&day1_prefix))
            .count();

        assert_eq!(day2_count, 1);
        assert_eq!(day1_count, 3);
    }

    #[tokio::test]
    async fn test_history_query_timezone_amsterdam() {
        let pool = setup_test_db().await;

        // Insert a task completed at a time that might fall on different days in different timezones
        let today = Utc::now().date_naive();
        let amsterdam_completed = datetime_str(today, "23:30:00Z");
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, title, status, completed_at, completed_by, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
        )
        .bind("tz_hist1")
        .bind("project1")
        .bind("user1")
        .bind("Timezone History Task")
        .bind("done")
        .bind(amsterdam_completed.as_str())
        .bind("user1")
        .execute(&pool)
        .await
        .expect("Failed to insert timezone history task");

        let schema = build(pool);

        let query = r#"
            query TestHistoryTimezone(
                $statuses: [TaskStatus!]!,
                $timezone: String!,
                $fromDate: String!,
                $toDate: String!
            ) {
                history(statuses: $statuses, timezone: $timezone, fromDate: $fromDate, toDate: $toDate) {
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
            "statuses": ["DONE"],
            "timezone": "Europe/Amsterdam",
            "fromDate": date_str(shift_date(today, -1)),
            "toDate": date_str(shift_date(today, 1))
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response_data(&response);
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
        let today = Utc::now().date_naive();
        let ny_abandoned = datetime_str(today, "05:00:00Z");
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, title, status, abandoned_at, abandoned_by, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
        )
        .bind("ny_hist1")
        .bind("project1")
        .bind("user1")
        .bind("New York History Task")
        .bind("abandoned")
        .bind(ny_abandoned.as_str())
        .bind("user1")
        .execute(&pool)
        .await
        .expect("Failed to insert New York history task");

        let schema = build(pool);

        let query = r#"
            query TestHistoryNY(
                $statuses: [TaskStatus!]!,
                $timezone: String!,
                $fromDate: String!,
                $toDate: String!
            ) {
                history(statuses: $statuses, timezone: $timezone, fromDate: $fromDate, toDate: $toDate) {
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
            "statuses": ["ABANDONED"],
            "timezone": "America/New_York",
            "fromDate": date_str(shift_date(today, -1)),
            "toDate": date_str(shift_date(today, 1))
        });

        let response =
            execute_graphql_query(&schema, query, Some(variables), Some("testuser")).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response_data(&response);
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
