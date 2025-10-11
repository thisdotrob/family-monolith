use async_graphql::MergedObject;

pub mod projects;
pub mod tags;

mod history_query;
mod project_default_saved_view_query;
mod projects_query;
mod saved_views_query;
mod tags_query;
mod tasks_query;

pub use history_query::HistoryQuery;
pub use project_default_saved_view_query::ProjectDefaultSavedViewQuery;
pub use projects_query::ProjectsQuery;
pub use saved_views_query::SavedViewsQuery;
pub use tags_query::TagsQuery;
pub use tasks_query::TasksQuery;

pub use projects::ProjectsMutation;
pub use tags::TagsMutation;

#[derive(MergedObject, Default)]
pub struct TakenlijstQuery(
    ProjectsQuery,
    TagsQuery,
    TasksQuery,
    HistoryQuery,
    SavedViewsQuery,
    ProjectDefaultSavedViewQuery,
);

#[derive(MergedObject, Default)]
pub struct TakenlijstMutation(ProjectsMutation, TagsMutation);
