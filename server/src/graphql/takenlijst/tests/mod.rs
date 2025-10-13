// Tests for takenlijst GraphQL resolvers

pub mod add_project_member_by_username;
pub mod archive_project;
pub mod create_project;
pub mod create_recurring_series;
pub mod create_saved_view;
pub mod create_tag;
pub mod delete_saved_view;
pub mod delete_tag;
pub mod history_query;
pub mod project_default_saved_view_query;
pub mod projects_query;
pub mod rename_project;
pub mod rename_tag;
pub mod saved_views_query;
pub mod set_project_default_saved_view;
pub mod tags_query;
pub mod tasks_query;
pub mod unarchive_project;
pub mod update_saved_view;

// Additional unit tests for validation, permissions, concurrency, and recurrence
pub mod simple_validation;
