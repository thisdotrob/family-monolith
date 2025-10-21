use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::graphql::takenlijst::types::{SavedView, SavedViewFilters};
use async_graphql::{Context, Object};
use sqlx::{Row, SqlitePool};
use std::sync::Arc;

#[derive(Default)]
pub struct ProjectDefaultSavedViewQuery;

#[Object]
impl ProjectDefaultSavedViewQuery {
    async fn project_default_saved_view(
        &self,
        ctx: &Context<'_>,
        project_id: String,
    ) -> async_graphql::Result<Option<SavedView>> {
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let username = &claims.sub;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(username)
            .fetch_one(pool)
            .await?
            .0;

        // Check if user has access to this project
        require_member(pool, &user_id, &project_id).await?;

        // Fetch default saved view for the project
        let row_result = sqlx::query(
            "SELECT sv.id, sv.project_id, sv.name, sv.filters, sv.created_by, sv.created_at, sv.updated_at \
             FROM saved_views sv\n             INNER JOIN project_default_view pdv ON sv.id = pdv.saved_view_id\n             WHERE pdv.project_id = ?1"
        )
        .bind(&project_id)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row_result {
            let id: String = row.get("id");
            let project_id: String = row.get("project_id");
            let name: String = row.get("name");
            let filters_json: String = row.get("filters");
            let created_by: String = row.get("created_by");
            let created_at: String = row.get("created_at");
            let updated_at: String = row.get("updated_at");

            // Parse filters from JSON
            let filters: SavedViewFilters = match serde_json::from_str(&filters_json) {
                Ok(filters) => filters,
                Err(_) => {
                    return Err(async_graphql::Error::new("Invalid saved view filters"));
                }
            };

            Ok(Some(SavedView {
                id,
                project_id,
                name,
                filters,
                created_by,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }
}
