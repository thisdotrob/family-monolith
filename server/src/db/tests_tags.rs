#[cfg(test)]
mod db_tags_tests {
    use sqlx::{sqlite::SqliteRow, Row};

    use crate::db;
    use crate::db::helpers::normalize_tag_name;

    fn tmp_db_path() -> String {
        format!("./tmp_rovodev_tags_test_{}.sqlite", uuid::Uuid::new_v4())
    }

    #[tokio::test]
    async fn tags_table_enforces_uniqueness_on_normalized_values() {
        let path = tmp_db_path();
        let pool = db::init(&path).await.expect("db init");

        // Helper to insert a tag
        async fn insert_tag(pool: &sqlx::SqlitePool, name: &str) -> Result<(), sqlx::Error> {
            let normalized = normalize_tag_name(name);
            sqlx::query("INSERT INTO tags (id, name) VALUES (?, ?)")
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(normalized)
                .execute(pool)
                .await?
                .rows_affected();
            Ok(())
        }

        insert_tag(&pool, "  #Work  ").await.unwrap();
        // Different casings and extra spacing should normalize to same stored value
        let dup_cases = ["work", "WoRk", "work  ", "##Work", "\tWork\n"]; // variations that normalize to "work"
        for v in dup_cases {
            let res = insert_tag(&pool, v).await;
            assert!(res.is_err(), "expected uniqueness violation for value: {}", v);
        }

        // Verify the stored value is the normalized one
        let row: SqliteRow = sqlx::query("SELECT name FROM tags LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();
        let stored: String = row.get(0);
        assert_eq!(stored, "work");

        // Cleanup
        drop(pool);
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn tags_updated_at_changes_on_update() {
        let path = tmp_db_path();
        let pool = db::init(&path).await.expect("db init");
        let id = uuid::Uuid::new_v4().to_string();
        let now_row = sqlx::query("INSERT INTO tags (id, name) VALUES (?, ?)")
            .bind(&id)
            .bind(normalize_tag_name("#Home  Stuff"))
            .execute(&pool)
            .await
            .unwrap();
        assert_eq!(now_row.rows_affected(), 1);

        let row1: (String, String) = sqlx::query_as("SELECT created_at, updated_at FROM tags WHERE id = ?")
            .bind(&id)
            .fetch_one(&pool)
            .await
            .unwrap();

        // Sleep a bit to ensure a timestamp change
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Perform an update to trigger updated_at change (rename)
        let _ = sqlx::query("UPDATE tags SET name = ? WHERE id = ?")
            .bind(normalize_tag_name("home stuff 2"))
            .bind(&id)
            .execute(&pool)
            .await
            .unwrap();

        let row2: (String, String) = sqlx::query_as("SELECT created_at, updated_at FROM tags WHERE id = ?")
            .bind(&id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(row1.0, row2.0, "created_at should remain unchanged");
        assert_ne!(row1.1, row2.1, "updated_at should update on row change");

        // Cleanup
        drop(pool);
        let _ = std::fs::remove_file(&path);
    }
}
