use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::graphql::types::{SavedView, SavedViewFilters};
use async_graphql::{Context, Object};
use sqlx::{Row, SqlitePool};
use std::sync::Arc;

#[derive(Default)]
pub struct SavedViewsQuery;

#[Object]
impl SavedViewsQuery {
    async fn saved_views(
        &self,
        ctx: &Context<'_>,
        project_id: String,
    ) -> async_graphql::Result<Vec<SavedView>> {
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

        // Fetch saved views for the project
        let rows = sqlx::query(
            "SELECT id, project_id, name, filters, created_by, created_at, updated_at \
             FROM saved_views \
             WHERE project_id = ?1 \
             ORDER BY name",
        )
        .bind(&project_id)
        .fetch_all(pool)
        .await?;

        let mut saved_views = Vec::new();
        for row in rows {
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

            saved_views.push(SavedView {
                id,
                project_id,
                name,
                filters,
                created_by,
                created_at,
                updated_at,
            });
        }

        Ok(saved_views)
    }
}
