use sqlx::SqlitePool;
mod auth;

pub use crate::graphql::auth::{AuthenticatedMutation, UnauthenticatedMutation};
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

    async fn project_members(
        &self,
        ctx: &Context<'_>,
        project_id: String,
    ) -> async_graphql::Result<Vec<User>> {
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

        // Check if user has access to this project (is owner or member)
        let has_access = sqlx::query_as::<_, (i32,)>(
            "SELECT 1 FROM projects p 
             LEFT JOIN project_members pm ON p.id = pm.project_id 
             WHERE p.id = ?1 AND (p.owner_id = ?2 OR pm.user_id = ?2)
             LIMIT 1",
        )
        .bind(&project_id)
        .bind(&user_id)
        .fetch_optional(pool)
        .await?
        .is_some();

        if !has_access {
            return Err(async_graphql::Error::new("Permission denied"));
        }

        // Get all members including the owner
        let members = sqlx::query_as::<_, (String, Option<String>)>(
            "SELECT DISTINCT u.username, u.first_name 
             FROM users u
             WHERE u.id IN (
                 SELECT p.owner_id FROM projects p WHERE p.id = ?1
                 UNION
                 SELECT pm.user_id FROM project_members pm WHERE pm.project_id = ?1
             )
             ORDER BY u.username",
        )
        .bind(&project_id)
        .fetch_all(pool)
        .await?;

        Ok(members
            .into_iter()
            .map(|(username, first_name)| User {
                username,
                first_name,
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
