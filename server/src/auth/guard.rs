use async_graphql::ErrorExtensions;
use sqlx::SqlitePool;

use crate::error_codes::ErrorCode;

/// Check if a user is the owner of a project
pub async fn is_owner(
    pool: &SqlitePool,
    user_id: &str,
    project_id: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM projects WHERE id = ?1 AND owner_id = ?2",
    )
    .bind(project_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(result.0 > 0)
}

/// Check if a user is a member of a project (including owner)
pub async fn is_member(
    pool: &SqlitePool,
    user_id: &str,
    project_id: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM projects p 
         LEFT JOIN project_members pm ON p.id = pm.project_id 
         WHERE p.id = ?1 AND (p.owner_id = ?2 OR pm.user_id = ?2)",
    )
    .bind(project_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(result.0 > 0)
}

/// Check if a user is the owner of a project and return appropriate GraphQL error if not
pub async fn require_owner(
    pool: &SqlitePool,
    user_id: &str,
    project_id: &str,
) -> async_graphql::Result<()> {
    match is_owner(pool, user_id, project_id).await {
        Ok(true) => Ok(()),
        Ok(false) => {
            let error = async_graphql::Error::new("Only project owner can perform this action")
                .extend_with(|_, e| e.set("code", ErrorCode::PermissionDenied.as_str()));
            Err(error)
        }
        Err(_) => {
            let error = async_graphql::Error::new("Project not found")
                .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
            Err(error)
        }
    }
}

/// Check if a user is a member of a project and return appropriate GraphQL error if not
pub async fn require_member(
    pool: &SqlitePool,
    user_id: &str,
    project_id: &str,
) -> async_graphql::Result<()> {
    match is_member(pool, user_id, project_id).await {
        Ok(true) => Ok(()),
        Ok(false) => {
            let error = async_graphql::Error::new("Project not found or access denied")
                .extend_with(|_, e| e.set("code", ErrorCode::PermissionDenied.as_str()));
            Err(error)
        }
        Err(_) => {
            let error = async_graphql::Error::new("Project not found")
                .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
    use std::str::FromStr;

    async fn create_test_pool() -> SqlitePool {
        let options = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true)
            .foreign_keys(true);

        let pool = SqlitePool::connect_with(options).await.unwrap();

        // Create tables manually to avoid migration conflicts
        sqlx::query(
            r#"
            CREATE TABLE users (
                id TEXT PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                password TEXT NOT NULL,
                first_name TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            )
        "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                owner_id TEXT NOT NULL,
                archived_at TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (owner_id) REFERENCES users(id)
            )
        "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE project_members (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(project_id, user_id),
                FOREIGN KEY (project_id) REFERENCES projects(id),
                FOREIGN KEY (user_id) REFERENCES users(id)
            )
        "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    async fn setup_test_data(pool: &SqlitePool) -> (String, String, String, String) {
        // Insert users with unique IDs to avoid conflicts
        let test_suffix = uuid::Uuid::new_v4().to_string()[..8].to_string();
        let owner_id = format!("owner_user_id_{}", test_suffix);
        let member_id = format!("member_user_id_{}", test_suffix);
        let non_member_id = format!("non_member_user_id_{}", test_suffix);
        let project_id = format!("test_project_id_{}", test_suffix);

        sqlx::query("INSERT INTO users (id, username, password, first_name) VALUES (?, ?, ?, ?)")
            .bind(&owner_id)
            .bind("owner")
            .bind("password")
            .bind("Owner")
            .execute(pool)
            .await
            .expect("insert owner user");

        sqlx::query("INSERT INTO users (id, username, password, first_name) VALUES (?, ?, ?, ?)")
            .bind(&member_id)
            .bind("member")
            .bind("password")
            .bind("Member")
            .execute(pool)
            .await
            .expect("insert member user");

        sqlx::query("INSERT INTO users (id, username, password, first_name) VALUES (?, ?, ?, ?)")
            .bind(&non_member_id)
            .bind("nonmember")
            .bind("password")
            .bind("NonMember")
            .execute(pool)
            .await
            .expect("insert non-member user");

        // Insert project owned by owner
        sqlx::query("INSERT INTO projects (id, name, owner_id) VALUES (?, ?, ?)")
            .bind(&project_id)
            .bind("Test Project")
            .bind(&owner_id)
            .execute(pool)
            .await
            .expect("insert project");

        // Add member to project
        sqlx::query("INSERT INTO project_members (project_id, user_id) VALUES (?, ?)")
            .bind(&project_id)
            .bind(&member_id)
            .execute(pool)
            .await
            .expect("insert project membership");

        (owner_id, member_id, non_member_id, project_id)
    }

    #[tokio::test]
    async fn test_is_owner_returns_true_for_owner() {
        let pool = create_test_pool().await;
        let (owner_id, _member_id, _non_member_id, project_id) = setup_test_data(&pool).await;

        let result = is_owner(&pool, &owner_id, &project_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_is_owner_returns_false_for_member() {
        let pool = create_test_pool().await;
        let (_owner_id, member_id, _non_member_id, project_id) = setup_test_data(&pool).await;

        let result = is_owner(&pool, &member_id, &project_id).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_is_member_returns_true_for_owner() {
        let pool = create_test_pool().await;
        let (owner_id, _member_id, _non_member_id, project_id) = setup_test_data(&pool).await;

        let result = is_member(&pool, &owner_id, &project_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_is_member_returns_true_for_member() {
        let pool = create_test_pool().await;
        let (_owner_id, member_id, _non_member_id, project_id) = setup_test_data(&pool).await;

        let result = is_member(&pool, &member_id, &project_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_is_member_returns_false_for_non_member() {
        let pool = create_test_pool().await;
        let (_owner_id, _member_id, non_member_id, project_id) = setup_test_data(&pool).await;

        let result = is_member(&pool, &non_member_id, &project_id).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_require_owner_succeeds_for_owner() {
        let pool = create_test_pool().await;
        let (owner_id, _member_id, _non_member_id, project_id) = setup_test_data(&pool).await;

        let result = require_owner(&pool, &owner_id, &project_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_require_owner_fails_for_member() {
        let pool = create_test_pool().await;
        let (_owner_id, member_id, _non_member_id, project_id) = setup_test_data(&pool).await;

        let result = require_owner(&pool, &member_id, &project_id).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.message, "Only project owner can perform this action");
        assert_eq!(
            error.extensions.as_ref().unwrap().get("code").unwrap(),
            &async_graphql::Value::from("PERMISSION_DENIED")
        );
    }

    #[tokio::test]
    async fn test_require_member_succeeds_for_member() {
        let pool = create_test_pool().await;
        let (_owner_id, member_id, _non_member_id, project_id) = setup_test_data(&pool).await;

        let result = require_member(&pool, &member_id, &project_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_require_member_fails_for_non_member() {
        let pool = create_test_pool().await;
        let (_owner_id, _member_id, non_member_id, project_id) = setup_test_data(&pool).await;

        let result = require_member(&pool, &non_member_id, &project_id).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.message, "Project not found or access denied");
        assert_eq!(
            error.extensions.as_ref().unwrap().get("code").unwrap(),
            &async_graphql::Value::from("PERMISSION_DENIED")
        );
    }
}
