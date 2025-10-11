use crate::auth::Claims;
use crate::graphql::types::Project;
use async_graphql::{Context, Object};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct ProjectsQuery;

#[Object]
impl ProjectsQuery {
    async fn projects(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = false)] include_archived: bool,
        #[graphql(default = 0)] offset: i32,
        #[graphql(default = 50)] limit: i32,
    ) -> async_graphql::Result<Vec<Project>> {
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let username = &claims.sub;

        // First get the user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(username)
            .fetch_one(pool)
            .await?
            .0;

        // Build the query to get projects where user is owner or member
        let mut query = String::from(
            "SELECT DISTINCT p.id, p.name, p.owner_id, p.archived_at, p.created_at, p.updated_at \
             FROM projects p \
             LEFT JOIN project_members pm ON p.id = pm.project_id \
             WHERE (p.owner_id = ?1 OR pm.user_id = ?1)",
        );

        if !include_archived {
            query.push_str(" AND p.archived_at IS NULL");
        }

        query.push_str(" ORDER BY p.created_at DESC LIMIT ?2 OFFSET ?3");

        let projects =
            sqlx::query_as::<_, (String, String, String, Option<String>, String, String)>(&query)
                .bind(&user_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?;

        Ok(projects
            .into_iter()
            .map(
                |(id, name, owner_id, archived_at, created_at, updated_at)| Project {
                    id,
                    name,
                    owner_id,
                    archived_at,
                    created_at,
                    updated_at,
                },
            )
            .collect())
    }
}
