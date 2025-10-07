#[cfg(test)]
mod tests {
    use crate::graphql::SavedViewFilters;
    use crate::tasks::TaskStatus;
    use serde_json;

    #[test]
    fn test_saved_view_filters_serialization() {
        let filters = SavedViewFilters {
            statuses: vec![TaskStatus::Todo, TaskStatus::Done],
            assignee: Some("user123".to_string()),
            include_unassigned: false,
            assigned_to_me: true,
            tag_ids: vec!["tag1".to_string(), "tag2".to_string()],
        };

        // Test serialization
        let json = serde_json::to_string(&filters).expect("Should serialize");

        // Test deserialization
        let parsed: SavedViewFilters = serde_json::from_str(&json).expect("Should deserialize");

        // Verify round-trip
        assert_eq!(parsed.statuses.len(), 2);
        assert_eq!(parsed.assignee, Some("user123".to_string()));
        assert_eq!(parsed.include_unassigned, false);
        assert_eq!(parsed.assigned_to_me, true);
        assert_eq!(parsed.tag_ids.len(), 2);
    }

    #[test]
    fn test_task_status_serialization() {
        // Test each status value serializes correctly
        let todo_json = serde_json::to_string(&TaskStatus::Todo).expect("Should serialize Todo");
        let done_json = serde_json::to_string(&TaskStatus::Done).expect("Should serialize Done");
        let abandoned_json =
            serde_json::to_string(&TaskStatus::Abandoned).expect("Should serialize Abandoned");

        assert_eq!(todo_json, "\"todo\"");
        assert_eq!(done_json, "\"done\"");
        assert_eq!(abandoned_json, "\"abandoned\"");

        // Test deserialization
        let todo: TaskStatus = serde_json::from_str("\"todo\"").expect("Should deserialize todo");
        let done: TaskStatus = serde_json::from_str("\"done\"").expect("Should deserialize done");
        let abandoned: TaskStatus =
            serde_json::from_str("\"abandoned\"").expect("Should deserialize abandoned");

        assert_eq!(todo, TaskStatus::Todo);
        assert_eq!(done, TaskStatus::Done);
        assert_eq!(abandoned, TaskStatus::Abandoned);
    }
}
