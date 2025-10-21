use async_graphql::MergedObject;

pub mod queries;
pub mod types;

mod add_project_member_by_username;
mod archive_project;
mod create_project;
mod create_tag;
mod delete_tag;
mod rename_project;
mod rename_tag;
mod unarchive_project;

mod create_recurring_series;
mod create_saved_view;
mod delete_saved_view;
mod set_project_default_saved_view;
mod update_saved_view;

// Task mutations
mod abandon_task;
mod complete_task;
mod create_task;
mod restore_task;
mod update_task;

#[cfg(test)]
pub mod tests;

pub use queries::HistoryQuery;
pub use queries::ProjectDefaultSavedViewQuery;
pub use queries::ProjectsQuery;
pub use queries::SavedViewsQuery;
pub use queries::TagsQuery;
pub use queries::TasksQuery;

pub use create_recurring_series::CreateRecurringSeriesMutation;
pub use create_saved_view::CreateSavedViewMutation;
pub use delete_saved_view::DeleteSavedViewMutation;
pub use set_project_default_saved_view::SetProjectDefaultSavedViewMutation;
pub use update_saved_view::UpdateSavedViewMutation;

use add_project_member_by_username::AddProjectMemberByUsernameMutation;
use archive_project::ArchiveProjectMutation;
use create_project::CreateProjectMutation;
use create_tag::CreateTagMutation;
use delete_tag::DeleteTagMutation;
use rename_project::RenameProjectMutation;
use rename_tag::RenameTagMutation;
use unarchive_project::UnarchiveProjectMutation;

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
pub struct ProjectsMutation(
    CreateProjectMutation,
    RenameProjectMutation,
    ArchiveProjectMutation,
    UnarchiveProjectMutation,
    AddProjectMemberByUsernameMutation,
);

#[derive(MergedObject, Default)]
pub struct TagsMutation(CreateTagMutation, RenameTagMutation, DeleteTagMutation);

#[derive(MergedObject, Default)]
pub struct TakenlijstMutation(
    ProjectsMutation,
    TagsMutation,
    CreateRecurringSeriesMutation,
    CreateSavedViewMutation,
    UpdateSavedViewMutation,
    DeleteSavedViewMutation,
    SetProjectDefaultSavedViewMutation,
    create_task::CreateTaskMutation,
    update_task::UpdateTaskMutation,
    complete_task::CompleteTaskMutation,
    abandon_task::AbandonTaskMutation,
    restore_task::RestoreTaskMutation,
);
