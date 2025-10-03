use crate::auth::Claims;
use crate::db::helpers::normalize_tag_name;
use crate::graphql::Tag;
use async_graphql::{Context, ErrorExtensions, InputObject, Object, SimpleObject};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(InputObject)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct LoginPayload {
    pub success: bool,
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub errors: Vec<String>,
}

#[derive(Default)]
pub struct UnauthenticatedMutation;

#[Object]
impl UnauthenticatedMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> LoginPayload {
        let pool = ctx.data::<sqlx::SqlitePool>().unwrap();
        let user_result = sqlx::query_as::<_, (String, String, String)>(
            "SELECT id, username, password FROM users WHERE username = ?1",
        )
        .bind(&input.username.to_lowercase())
        .fetch_one(pool)
        .await;

        if let Ok(user) = user_result {
            if crate::auth::verify(&user.2, &input.password).await {
                let token = crate::auth::encode(&user.1, 5).unwrap();
                let refresh = crate::auth::refresh::create(pool, &user.0).await.unwrap();
                return LoginPayload {
                    success: true,
                    token: Some(token),
                    refresh_token: Some(refresh),
                    errors: vec![],
                };
            }
        }
        LoginPayload {
            success: false,
            token: None,
            refresh_token: None,
            errors: vec!["INVALID_CREDENTIALS".into()],
        }
    }

    async fn refresh_token(&self, ctx: &Context<'_>, input: RefreshInput) -> RefreshPayload {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let refresh_token = input.refresh_token;

        // Since we can't access the private refresh module directly,
        // we need to implement the rotation logic here

        // First, try to find the user_id associated with this token
        let user_id_result = crate::db::helpers::fetch_one::<(String,)>(
            pool,
            "SELECT user_id FROM refresh_tokens WHERE token = ?1",
            &[&refresh_token],
        )
        .await;

        if let Ok((user_id,)) = user_id_result {
            // Delete the old token
            let _ = crate::db::helpers::execute(
                pool,
                "DELETE FROM refresh_tokens WHERE token = ?1",
                &[&refresh_token],
            )
            .await;

            // Create a new token for the same user
            if let Ok(new_rt) = crate::auth::refresh::create(pool, &user_id).await {
                // Get the username to embed in JWT
                if let Ok((username,)) = crate::db::helpers::fetch_one::<(String,)>(
                    pool,
                    "SELECT username FROM users WHERE id = ?1",
                    &[&user_id],
                )
                .await
                {
                    let token = crate::auth::encode(&username, 5).unwrap();
                    return RefreshPayload {
                        success: true,
                        token: Some(token),
                        refresh_token: Some(new_rt),
                        errors: vec![],
                    };
                }
            }
        }

        RefreshPayload {
            success: false,
            token: None,
            refresh_token: None,
            errors: vec!["TOKEN_INVALID".into()],
        }
    }
}

#[derive(Default)]
pub struct AuthenticatedMutation;

#[Object]
impl AuthenticatedMutation {
    async fn logout(&self, ctx: &Context<'_>, input: LogoutInput) -> LogoutPayload {
        // Require valid claims for logout
        let _claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(c) => c,
            None => {
                return LogoutPayload { success: false };
            }
        };

        let pool = ctx.data::<SqlitePool>().unwrap();
        let rows = crate::auth::refresh::delete(pool, &input.refresh_token)
            .await
            .unwrap_or(0);
        LogoutPayload { success: rows > 0 }
    }

    async fn create_tag(&self, ctx: &Context<'_>, name: String) -> async_graphql::Result<Tag> {
        // Require authentication
        let _claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let normalized_name = normalize_tag_name(&name);

        if normalized_name.is_empty() {
            let error = async_graphql::Error::new("Tag name cannot be empty after normalization")
                .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
            return Err(error);
        }

        // Check if tag already exists (normalized comparison)
        if let Ok(existing_tag) = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, name, created_at, updated_at FROM tags WHERE name = ?1",
        )
        .bind(&normalized_name)
        .fetch_one(pool)
        .await
        {
            // Return existing tag
            return Ok(Tag {
                id: existing_tag.0,
                name: existing_tag.1,
                created_at: existing_tag.2,
                updated_at: existing_tag.3,
            });
        }

        // Create new tag
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO tags (id, name) VALUES (?1, ?2)")
            .bind(&id)
            .bind(&normalized_name)
            .execute(pool)
            .await?;

        // Fetch the created tag
        let tag = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, name, created_at, updated_at FROM tags WHERE id = ?1",
        )
        .bind(&id)
        .fetch_one(pool)
        .await?;

        Ok(Tag {
            id: tag.0,
            name: tag.1,
            created_at: tag.2,
            updated_at: tag.3,
        })
    }

    async fn rename_tag(
        &self,
        ctx: &Context<'_>,
        tag_id: String,
        new_name: String,
    ) -> async_graphql::Result<Tag> {
        // Require authentication
        let _claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let normalized_name = normalize_tag_name(&new_name);

        if normalized_name.is_empty() {
            let error = async_graphql::Error::new("Tag name cannot be empty after normalization")
                .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
            return Err(error);
        }

        // Check if the tag to rename exists
        let existing_tag = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, name, created_at, updated_at FROM tags WHERE id = ?1",
        )
        .bind(&tag_id)
        .fetch_one(pool)
        .await;

        if existing_tag.is_err() {
            let error = async_graphql::Error::new("Tag not found")
                .extend_with(|_, e| e.set("code", "NOT_FOUND"));
            return Err(error);
        }

        // Check if a tag with the new name already exists
        if let Ok(collision_tag) = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, name, created_at, updated_at FROM tags WHERE name = ?1",
        )
        .bind(&normalized_name)
        .fetch_one(pool)
        .await
        {
            // If it's the same tag, just return it
            if collision_tag.0 == tag_id {
                return Ok(Tag {
                    id: collision_tag.0,
                    name: collision_tag.1,
                    created_at: collision_tag.2,
                    updated_at: collision_tag.3,
                });
            }

            // Merge: move any task_tags from old tag to existing tag (when task_tags table exists)
            // For now, we'll just delete the old tag since task_tags doesn't exist yet
            // TODO: When task_tags table exists, implement proper merge:
            // UPDATE task_tags SET tag_id = ?1 WHERE tag_id = ?2
            sqlx::query("DELETE FROM tags WHERE id = ?1")
                .bind(&tag_id)
                .execute(pool)
                .await?;

            return Ok(Tag {
                id: collision_tag.0,
                name: collision_tag.1,
                created_at: collision_tag.2,
                updated_at: collision_tag.3,
            });
        }

        // No collision, just rename
        sqlx::query("UPDATE tags SET name = ?1 WHERE id = ?2")
            .bind(&normalized_name)
            .bind(&tag_id)
            .execute(pool)
            .await?;

        // Fetch updated tag
        let tag = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, name, created_at, updated_at FROM tags WHERE id = ?1",
        )
        .bind(&tag_id)
        .fetch_one(pool)
        .await?;

        Ok(Tag {
            id: tag.0,
            name: tag.1,
            created_at: tag.2,
            updated_at: tag.3,
        })
    }

    async fn delete_tag(&self, ctx: &Context<'_>, tag_id: String) -> async_graphql::Result<bool> {
        // Require authentication
        let _claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Check if tag exists
        let tag_exists = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tags WHERE id = ?1")
            .bind(&tag_id)
            .fetch_one(pool)
            .await?;

        if tag_exists.0 == 0 {
            let error = async_graphql::Error::new("Tag not found")
                .extend_with(|_, e| e.set("code", "NOT_FOUND"));
            return Err(error);
        }

        // Check if tag is in use by any tasks (when task_tags table exists)
        // For now, we'll allow deletion since task_tags doesn't exist yet
        // TODO: When task_tags table exists, implement proper check:
        // let usage_count = sqlx::query_as::<_, (i64,)>(
        //     "SELECT COUNT(*) FROM task_tags WHERE tag_id = ?1"
        // )
        // .bind(&tag_id)
        // .fetch_one(pool)
        // .await?;
        //
        // if usage_count.0 > 0 {
        //     let mut error = async_graphql::Error::new("Cannot delete tag that is in use by tasks");
        //     error.extensions.insert("code", "VALIDATION_FAILED".into());
        //     return Err(error);
        // }

        // Delete the tag
        let result = sqlx::query("DELETE FROM tags WHERE id = ?1")
            .bind(&tag_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[derive(InputObject)]
struct RefreshInput {
    refresh_token: String,
}

#[derive(SimpleObject)]
struct RefreshPayload {
    success: bool,
    token: Option<String>,
    refresh_token: Option<String>,
    errors: Vec<String>,
}

#[derive(InputObject)]
struct LogoutInput {
    refresh_token: String,
}

#[derive(SimpleObject)]
struct LogoutPayload {
    success: bool,
}
