use sqlx::SqlitePool;

/// Normalize a tag name according to database rules:
/// - Trim leading/trailing whitespace
/// - Strip all leading '#'
/// - Collapse internal whitespace runs to a single space
/// - Lowercase for deterministic, case-insensitive uniqueness
pub fn normalize_tag_name<S: AsRef<str>>(name: S) -> String {
    let mut s = name.as_ref().trim().to_string();
    // Strip all leading '#'
    while s.starts_with('#') {
        s.remove(0);
    }
    // Collapse internal whitespace (any consecutive Unicode whitespace -> single ASCII space)
    let mut collapsed = String::with_capacity(s.len());
    let mut prev_space = false;
    for ch in s.chars() {
        if ch.is_whitespace() {
            if !prev_space {
                collapsed.push(' ');
                prev_space = true;
            }
        } else {
            collapsed.push(ch);
            prev_space = false;
        }
    }
    collapsed.trim().to_lowercase()
}

/// Normalize a project name according to database rules:
/// - Trim leading/trailing whitespace
/// - Collapse internal whitespace runs to a single space
/// - Keep original case (unlike tags)
pub fn normalize_project_name<S: AsRef<str>>(name: S) -> String {
    let s = name.as_ref().trim();
    // Collapse internal whitespace (any consecutive Unicode whitespace -> single ASCII space)
    let mut collapsed = String::with_capacity(s.len());
    let mut prev_space = false;
    for ch in s.chars() {
        if ch.is_whitespace() {
            if !prev_space {
                collapsed.push(' ');
                prev_space = true;
            }
        } else {
            collapsed.push(ch);
            prev_space = false;
        }
    }
    collapsed.trim().to_string()
}

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

#[cfg(test)]
mod tests {
    use super::{normalize_project_name, normalize_tag_name};

    #[test]
    fn test_normalize_tag_name_basic() {
        assert_eq!(normalize_tag_name("Work"), "work");
        assert_eq!(normalize_tag_name("  Work  "), "work");
        assert_eq!(normalize_tag_name("#Work"), "work");
        assert_eq!(normalize_tag_name("##Work"), "work");
        assert_eq!(normalize_tag_name("wOrK"), "work");
    }

    #[test]
    fn test_normalize_tag_name_spaces() {
        assert_eq!(normalize_tag_name("Foo   Bar"), "foo bar");
        assert_eq!(normalize_tag_name("  Foo\t\tBar  "), "foo bar");
        assert_eq!(normalize_tag_name("Foo\nBar"), "foo bar");
    }

    #[test]
    fn test_normalize_tag_name_unicode_ws() {
        // non-breaking space and em-space should collapse
        let s = format!("Foo{}{}Bar", '\u{00A0}', '\u{2003}');
        assert_eq!(normalize_tag_name(s), "foo bar");
    }

    #[test]
    fn test_normalize_project_name_basic() {
        assert_eq!(normalize_project_name("My Project"), "My Project");
        assert_eq!(normalize_project_name("  My Project  "), "My Project");
        assert_eq!(normalize_project_name("MyProject"), "MyProject");
    }

    #[test]
    fn test_normalize_project_name_spaces() {
        assert_eq!(normalize_project_name("Foo   Bar"), "Foo Bar");
        assert_eq!(normalize_project_name("  Foo\t\tBar  "), "Foo Bar");
        assert_eq!(normalize_project_name("Foo\nBar"), "Foo Bar");
    }

    #[test]
    fn test_normalize_project_name_case_preserved() {
        assert_eq!(
            normalize_project_name("CamelCase Project"),
            "CamelCase Project"
        );
        assert_eq!(
            normalize_project_name("UPPERCASE project"),
            "UPPERCASE project"
        );
    }
}
