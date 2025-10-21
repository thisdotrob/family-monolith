use crate::graphql::build;
use async_graphql::{Request, Variables, value};
use sqlx::SqlitePool;

const TEST_TIMEZONE: &str = "UTC";

async fn setup_test_db() -> (crate::graphql::AppSchema, SqlitePool) {
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

    let schema = build(pool.clone());

    // Add auth context
    use crate::auth::Claims;
    use std::sync::Arc;
    let claims = Claims {
        sub: "testuser".to_string(),
        exp: 9999999999,
    };

    (schema, pool)
}

#[tokio::test]
async fn test_update_recurring_series_success() {
    let (schema, _pool) = setup_test_db().await;

    use crate::auth::Claims;
    use std::sync::Arc;
    let claims = Claims {
        sub: "testuser".to_string(),
        exp: 9999999999,
    };

    // First create a project
    let create_project_query = r#"
        mutation CreateProject($name: String!) {
            createProject(name: $name) {
                id
                name
            }
        }
    "#;

    let create_project_variables = Variables::from_value(value!({
        "name": "Test Project"
    }));

    let create_project_response = schema
        .execute(Request::new(create_project_query).variables(create_project_variables))
        .await;

    assert!(create_project_response.errors.is_empty());
    let project_id = create_project_response.data.as_object().unwrap()["createProject"]
        .as_object()
        .unwrap()["id"]
        .as_str()
        .unwrap();

    // Create a recurring series
    let create_series_query = r#"
        mutation CreateRecurringSeries($input: CreateSeriesInput!) {
            createRecurringSeries(input: $input) {
                id
                title
                description
                rrule
                dtstartDate
                dtstartTimeMinutes
                deadlineOffsetMinutes
                updatedAt
            }
        }
    "#;

    let create_series_variables = Variables::from_value(value!({
        "input": {
            "projectId": project_id,
            "title": "Original Title",
            "description": "Original Description",
            "rrule": "FREQ=DAILY;INTERVAL=1",
            "dtstartDate": "2025-12-01",
            "dtstartTimeMinutes": 540,
            "deadlineOffsetMinutes": 60,
            "timezone": TEST_TIMEZONE
        }
    }));

    let create_series_response = schema
        .execute(Request::new(create_series_query).variables(create_series_variables))
        .await;

    assert!(create_series_response.errors.is_empty());
    let series_data = create_series_response.data.as_object().unwrap()["createRecurringSeries"]
        .as_object()
        .unwrap();
    let series_id = series_data["id"].as_str().unwrap();
    let original_updated_at = series_data["updatedAt"].as_str().unwrap();

    // Now update the series
    let update_series_query = r#"
        mutation UpdateRecurringSeries($id: ID!, $input: UpdateSeriesInput!, $lastKnownUpdatedAt: String!) {
            updateRecurringSeries(id: $id, input: $input, lastKnownUpdatedAt: $lastKnownUpdatedAt) {
                id
                title
                description
                rrule
                dtstartDate
                dtstartTimeMinutes
                deadlineOffsetMinutes
                updatedAt
            }
        }
    "#;

    let update_series_variables = Variables::from_value(value!({
        "id": series_id,
        "input": {
            "title": "Updated Title",
            "description": "Updated Description",
            "rrule": "FREQ=WEEKLY;INTERVAL=1",
            "deadlineOffsetMinutes": 120,
            "timezone": TEST_TIMEZONE
        },
        "lastKnownUpdatedAt": original_updated_at
    }));

    let update_series_response = schema
        .execute(Request::new(update_series_query).variables(update_series_variables))
        .await;

    assert!(update_series_response.errors.is_empty());
    let updated_series = update_series_response.data.as_object().unwrap()["updateRecurringSeries"]
        .as_object()
        .unwrap();

    assert_eq!(updated_series["id"].as_str().unwrap(), series_id);
    assert_eq!(updated_series["title"].as_str().unwrap(), "Updated Title");
    assert_eq!(
        updated_series["description"].as_str().unwrap(),
        "Updated Description"
    );
    assert_eq!(
        updated_series["rrule"].as_str().unwrap(),
        "FREQ=WEEKLY;INTERVAL=1"
    );
    assert_eq!(
        updated_series["deadlineOffsetMinutes"].as_i64().unwrap(),
        120
    );
    assert_ne!(
        updated_series["updatedAt"].as_str().unwrap(),
        original_updated_at
    );
}

#[tokio::test]
async fn test_update_recurring_series_stale_write() {
    let (schema, _pool) = setup_test_db().await;

    // First create a project
    let create_project_query = r#"
        mutation CreateProject($name: String!) {
            createProject(name: $name) {
                id
                name
            }
        }
    "#;

    let create_project_variables = Variables::from_value(value!({
        "name": "Test Project"
    }));

    let create_project_response = schema
        .execute(Request::new(create_project_query).variables(create_project_variables))
        .await;

    assert!(create_project_response.errors.is_empty());
    let project_id = create_project_response.data.as_object().unwrap()["createProject"]
        .as_object()
        .unwrap()["id"]
        .as_str()
        .unwrap();

    // Create a recurring series
    let create_series_query = r#"
        mutation CreateRecurringSeries($input: CreateSeriesInput!) {
            createRecurringSeries(input: $input) {
                id
                updatedAt
            }
        }
    "#;

    let create_series_variables = Variables::from_value(value!({
        "input": {
            "projectId": project_id,
            "title": "Original Title",
            "rrule": "FREQ=DAILY;INTERVAL=1",
            "dtstartDate": "2025-12-01",
            "deadlineOffsetMinutes": 60,
            "timezone": TEST_TIMEZONE
        }
    }));

    let create_series_response = schema
        .execute(Request::new(create_series_query).variables(create_series_variables))
        .await;

    assert!(create_series_response.errors.is_empty());
    let series_data = create_series_response.data.as_object().unwrap()["createRecurringSeries"]
        .as_object()
        .unwrap();
    let series_id = series_data["id"].as_str().unwrap();

    // Try to update with a stale timestamp
    let update_series_query = r#"
        mutation UpdateRecurringSeries($id: ID!, $input: UpdateSeriesInput!, $lastKnownUpdatedAt: String!) {
            updateRecurringSeries(id: $id, input: $input, lastKnownUpdatedAt: $lastKnownUpdatedAt) {
                id
                title
            }
        }
    "#;

    let update_series_variables = Variables::from_value(value!({
        "id": series_id,
        "input": {
            "title": "Updated Title",
            "timezone": TEST_TIMEZONE
        },
        "lastKnownUpdatedAt": "2023-01-01 00:00:00.000"
    }));

    let update_series_response = schema
        .execute(Request::new(update_series_query).variables(update_series_variables))
        .await;

    assert!(!update_series_response.errors.is_empty());
    let error = &update_series_response.errors[0];
    assert_eq!(
        error.extensions.as_ref().unwrap()["code"],
        "CONFLICT_STALE_WRITE"
    );
}

#[tokio::test]
async fn test_update_recurring_series_not_found() {
    let (schema, _pool) = setup_test_db().await;

    let update_series_query = r#"
        mutation UpdateRecurringSeries($id: ID!, $input: UpdateSeriesInput!, $lastKnownUpdatedAt: String!) {
            updateRecurringSeries(id: $id, input: $input, lastKnownUpdatedAt: $lastKnownUpdatedAt) {
                id
                title
            }
        }
    "#;

    let update_series_variables = Variables::from_value(value!({
        "id": "non-existent-id",
        "input": {
            "title": "Updated Title",
            "timezone": TEST_TIMEZONE
        },
        "lastKnownUpdatedAt": "2023-01-01 00:00:00.000"
    }));

    let update_series_response = schema
        .execute(Request::new(update_series_query).variables(update_series_variables))
        .await;

    assert!(!update_series_response.errors.is_empty());
    let error = &update_series_response.errors[0];
    assert_eq!(error.extensions.as_ref().unwrap()["code"], "NOT_FOUND");
}

#[tokio::test]
async fn test_update_recurring_series_validation_failed() {
    let (schema, _pool) = setup_test_db().await;

    // First create a project
    let create_project_query = r#"
        mutation CreateProject($name: String!) {
            createProject(name: $name) {
                id
                name
            }
        }
    "#;

    let create_project_variables = Variables::from_value(value!({
        "name": "Test Project"
    }));

    let create_project_response = schema
        .execute(Request::new(create_project_query).variables(create_project_variables))
        .await;

    assert!(create_project_response.errors.is_empty());
    let project_id = create_project_response.data.as_object().unwrap()["createProject"]
        .as_object()
        .unwrap()["id"]
        .as_str()
        .unwrap();

    // Create a recurring series
    let create_series_query = r#"
        mutation CreateRecurringSeries($input: CreateSeriesInput!) {
            createRecurringSeries(input: $input) {
                id
                updatedAt
            }
        }
    "#;

    let create_series_variables = Variables::from_value(value!({
        "input": {
            "projectId": project_id,
            "title": "Original Title",
            "rrule": "FREQ=DAILY;INTERVAL=1",
            "dtstartDate": "2025-12-01",
            "deadlineOffsetMinutes": 60,
            "timezone": TEST_TIMEZONE
        }
    }));

    let create_series_response = schema
        .execute(Request::new(create_series_query).variables(create_series_variables))
        .await;

    assert!(create_series_response.errors.is_empty());
    let series_data = create_series_response.data.as_object().unwrap()["createRecurringSeries"]
        .as_object()
        .unwrap();
    let series_id = series_data["id"].as_str().unwrap();
    let original_updated_at = series_data["updatedAt"].as_str().unwrap();

    // Try to update with invalid deadline offset
    let update_series_query = r#"
        mutation UpdateRecurringSeries($id: ID!, $input: UpdateSeriesInput!, $lastKnownUpdatedAt: String!) {
            updateRecurringSeries(id: $id, input: $input, lastKnownUpdatedAt: $lastKnownUpdatedAt) {
                id
                title
            }
        }
    "#;

    let update_series_variables = Variables::from_value(value!({
        "id": series_id,
        "input": {
            "deadlineOffsetMinutes": 999999, // Invalid: exceeds maximum
            "timezone": TEST_TIMEZONE
        },
        "lastKnownUpdatedAt": original_updated_at
    }));

    let update_series_response = schema
        .execute(Request::new(update_series_query).variables(update_series_variables))
        .await;

    assert!(!update_series_response.errors.is_empty());
    let error = &update_series_response.errors[0];
    assert_eq!(
        error.extensions.as_ref().unwrap()["code"],
        "VALIDATION_FAILED"
    );
}
