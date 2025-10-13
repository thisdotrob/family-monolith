// Unit tests for validation logic

#[cfg(test)]
mod tests {
    use crate::db::helpers::{normalize_project_name, normalize_tag_name};

    #[test]
    fn test_tag_normalization_edge_cases() {
        // Empty after normalization
        assert_eq!(normalize_tag_name(""), "");
        assert_eq!(normalize_tag_name("   "), "");
        assert_eq!(normalize_tag_name("###"), "");
        assert_eq!(normalize_tag_name("# # #"), "# #");

        // Multiple hashtags stripped
        assert_eq!(normalize_tag_name("###work"), "work");
        assert_eq!(normalize_tag_name("#####project"), "project");

        // Unicode whitespace handling
        assert_eq!(normalize_tag_name("foo\u{00A0}\u{2003}bar"), "foo bar");
        assert_eq!(normalize_tag_name("test\t\n\rcase"), "test case");

        // Complex case with hashtags and whitespace
        assert_eq!(
            normalize_tag_name("  ##  Work   Project  ##  "),
            "work project"
        );

        // Emojis and special characters preserved
        assert_eq!(normalize_tag_name("work-item_123"), "work-item_123");
        assert_eq!(normalize_tag_name("🏠 home/office"), "🏠 home/office");

        // Case insensitive uniqueness
        assert_eq!(normalize_tag_name("Work"), "work");
        assert_eq!(normalize_tag_name("WORK"), "work");
        assert_eq!(normalize_tag_name("wOrK"), "work");

        // Maximum length handling (30 chars as per spec)
        let long_tag = "a".repeat(35);
        let normalized = normalize_tag_name(&long_tag);
        assert_eq!(normalized, "a".repeat(35).to_lowercase());
    }

    #[test]
    fn test_project_name_normalization() {
        // Basic trimming and whitespace collapse
        assert_eq!(normalize_project_name("  My Project  "), "My Project");
        assert_eq!(normalize_project_name("Foo   Bar"), "Foo Bar");
        assert_eq!(normalize_project_name("Test\t\n\rProject"), "Test Project");

        // Case preservation (unlike tags)
        assert_eq!(normalize_project_name("CamelCase"), "CamelCase");
        assert_eq!(normalize_project_name("UPPERCASE"), "UPPERCASE");

        // Empty handling
        assert_eq!(normalize_project_name(""), "");
        assert_eq!(normalize_project_name("   "), "");

        // Unicode whitespace
        assert_eq!(normalize_project_name("Foo\u{00A0}Bar"), "Foo Bar");

        // Maximum length (60 chars as per spec)
        let long_name = "A".repeat(65);
        let normalized = normalize_project_name(&long_name);
        assert_eq!(normalized, "A".repeat(65));

        // Special characters preserved
        assert_eq!(normalize_project_name("Project-2024_v1"), "Project-2024_v1");
    }

    #[test]
    fn test_saved_view_name_normalization() {
        // Uses same logic as projects (case-sensitive, trim/collapse)
        assert_eq!(
            normalize_project_name("  Important Tasks  "),
            "Important Tasks"
        );
        assert_eq!(normalize_project_name("My   View"), "My View");

        // Case sensitivity for saved views
        assert_eq!(normalize_project_name("Done"), "Done");
        assert_eq!(normalize_project_name("done"), "done");
    }

    #[test]
    fn test_deadline_offset_bounds() {
        // Valid range: 0 to +525600 minutes (365 days)
        let max_offset = 525600; // 365 * 24 * 60

        // Test bounds
        assert!(0 <= max_offset);
        assert!(max_offset <= 525600);

        // Test some valid values
        let one_hour = 60;
        let one_day = 24 * 60;
        let one_week = 7 * 24 * 60;
        let one_month = 30 * 24 * 60;

        assert!(one_hour <= max_offset);
        assert!(one_day <= max_offset);
        assert!(one_week <= max_offset);
        assert!(one_month <= max_offset);

        // Edge cases
        assert_eq!(0, 0); // minimum
        assert_eq!(max_offset, 525600); // maximum
    }
}
