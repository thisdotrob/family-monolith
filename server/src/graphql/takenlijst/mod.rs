use async_graphql::MergedObject;

pub mod projects;
pub mod tags;

mod history_query;
mod project_default_saved_view_query;
mod projects_query;
mod saved_views_query;
mod tags_query;
mod tasks_query;

mod create_recurring_series;
mod create_saved_view;
mod delete_saved_view;
mod set_project_default_saved_view;
mod update_saved_view;

pub use history_query::HistoryQuery;
pub use project_default_saved_view_query::ProjectDefaultSavedViewQuery;
pub use projects_query::ProjectsQuery;
pub use saved_views_query::SavedViewsQuery;
pub use tags_query::TagsQuery;
pub use tasks_query::TasksQuery;

pub use projects::ProjectsMutation;
pub use tags::TagsMutation;

pub use create_recurring_series::CreateRecurringSeriesMutation;
pub use create_saved_view::CreateSavedViewMutation;
pub use delete_saved_view::DeleteSavedViewMutation;
pub use set_project_default_saved_view::SetProjectDefaultSavedViewMutation;
pub use update_saved_view::UpdateSavedViewMutation;

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
pub struct TakenlijstMutation(
    ProjectsMutation,
    TagsMutation,
    CreateRecurringSeriesMutation,
    CreateSavedViewMutation,
    UpdateSavedViewMutation,
    DeleteSavedViewMutation,
    SetProjectDefaultSavedViewMutation,
);
