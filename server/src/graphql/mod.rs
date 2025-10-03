use sqlx::SqlitePool;
mod auth;
mod tests_recurring_series;

pub use crate::graphql::auth::{AuthenticatedMutation, RecurringSeries, UnauthenticatedMutation};
use async_graphql::{Context, EmptySubscription, Schema};
use async_graphql::{MergedObject, Object, SimpleObject};

use crate::auth::Claims;
use std::sync::Arc;

#[derive(SimpleObject)]
struct User {
    username: String,
    #[graphql(name = "firstName")]
    first_name: Option<String>,
}

#[derive(SimpleObject)]
struct Project {
    id: String,
    name: String,
    #[graphql(name = "ownerId")]
    owner_id: String,
    #[graphql(name = "archivedAt")]
    archived_at: Option<String>,
    #[graphql(name = "createdAt")]
    created_at: String,
    #[graphql(name = "updatedAt")]
    updated_at: String,
}

#[derive(SimpleObject)]
pub struct Tag {
    id: String,
    name: String,
    #[graphql(name = "createdAt")]
    created_at: String,
    #[graphql(name = "updatedAt")]
    updated_at: String,
}

// Define a query root
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        let username = &claims.sub;

        let user_data = sqlx::query_as::<_, (String, Option<String>)>(
            "SELECT username, first_name FROM users WHERE username = ?1",
        )
        .bind(username)
        .fetch_one(pool)
        .await?;

        Ok(User {
            username: user_data.0,
            first_name: user_data.1,
        })
    }

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
            "SELECT DISTINCT p.id, p.name, p.owner_id, p.archived_at, p.created_at, p.updated_at 
             FROM projects p 
             LEFT JOIN project_members pm ON p.id = pm.project_id 
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

    async fn tags(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] offset: i32,
        #[graphql(default = 200)] limit: i32,
    ) -> async_graphql::Result<Vec<Tag>> {
        // Require authentication
        let _claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        let tags = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, name, created_at, updated_at FROM tags ORDER BY name LIMIT ?1 OFFSET ?2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(tags
            .into_iter()
            .map(|(id, name, created_at, updated_at)| Tag {
                id,
                name,
                created_at,
                updated_at,
            })
            .collect())
    }
}

#[derive(MergedObject, Default)]
pub struct CombinedMutation(UnauthenticatedMutation, AuthenticatedMutation);

pub type AppSchema = Schema<QueryRoot, CombinedMutation, EmptySubscription>;

pub fn build(pool: SqlitePool) -> AppSchema {
    Schema::build(QueryRoot, CombinedMutation::default(), EmptySubscription)
        .data(pool)
        .limit_depth(5)
        .limit_complexity(50)
        .disable_introspection()
        .finish()
}
