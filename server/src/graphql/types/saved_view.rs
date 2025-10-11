use async_graphql::SimpleObject;

use super::SavedViewFilters;

#[derive(SimpleObject)]
pub struct SavedView {
    pub id: String,
    #[graphql(name = "projectId")]
    pub project_id: String,
    pub name: String,
    pub filters: SavedViewFilters,
    #[graphql(name = "createdBy")]
    pub created_by: String,
    #[graphql(name = "createdAt")]
    pub created_at: String,
    #[graphql(name = "updatedAt")]
    pub updated_at: String,
}
