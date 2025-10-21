use async_graphql::MergedObject;

pub mod mutations;
pub mod queries;
pub mod types;

#[cfg(test)]
pub mod tests;

pub use queries::HistoryQuery;
pub use queries::ProjectDefaultSavedViewQuery;
pub use queries::ProjectsQuery;
pub use queries::SavedViewsQuery;
pub use queries::TagsQuery;
pub use queries::TasksQuery;

pub use mutations::create_recurring_series::CreateRecurringSeriesMutation;
pub use mutations::create_saved_view::CreateSavedViewMutation;
pub use mutations::delete_saved_view::DeleteSavedViewMutation;
pub use mutations::set_project_default_saved_view::SetProjectDefaultSavedViewMutation;
pub use mutations::update_saved_view::UpdateSavedViewMutation;

use mutations::abandon_task::AbandonTaskMutation;
use mutations::add_project_member_by_username::AddProjectMemberByUsernameMutation;
use mutations::archive_project::ArchiveProjectMutation;
use mutations::complete_task::CompleteTaskMutation;
use mutations::create_project::CreateProjectMutation;
use mutations::create_tag::CreateTagMutation;
use mutations::create_task::CreateTaskMutation;
use mutations::delete_tag::DeleteTagMutation;
use mutations::rename_project::RenameProjectMutation;
use mutations::rename_tag::RenameTagMutation;
use mutations::restore_task::RestoreTaskMutation;
use mutations::unarchive_project::UnarchiveProjectMutation;
use mutations::update_recurring_series::UpdateRecurringSeriesMutation;
use mutations::update_task::UpdateTaskMutation;

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
    UpdateRecurringSeriesMutation,
    CreateSavedViewMutation,
    UpdateSavedViewMutation,
    DeleteSavedViewMutation,
    SetProjectDefaultSavedViewMutation,
    CreateTaskMutation,
    UpdateTaskMutation,
    CompleteTaskMutation,
    AbandonTaskMutation,
    RestoreTaskMutation,
);
