use crate::auth::Claims;
use crate::auth::guard::{require_member, require_owner};
use crate::db::helpers::{normalize_project_name, normalize_tag_name};
use crate::error_codes::ErrorCode;
use crate::graphql::{Project, Tag};
use crate::tasks::Task;
use async_graphql::{Context, ErrorExtensions, InputObject, Object, SimpleObject};
use chrono::{NaiveDate, NaiveTime, TimeZone, Utc};
use chrono_tz::Tz;
use rrule::RRule;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(InputObject)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct CreateTaskInput {
    #[graphql(name = "projectId")]
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
    #[graphql(name = "assigneeId")]
    pub assignee_id: Option<String>,
    #[graphql(name = "scheduledDate")]
    pub scheduled_date: Option<String>,
    #[graphql(name = "scheduledTimeMinutes")]
    pub scheduled_time_minutes: Option<i32>,
    #[graphql(name = "deadlineDate")]
    pub deadline_date: Option<String>,
    #[graphql(name = "deadlineTimeMinutes")]
    pub deadline_time_minutes: Option<i32>,
    #[graphql(name = "tagIds")]
    pub tag_ids: Option<Vec<String>>,
}

#[derive(InputObject)]
pub struct UpdateTaskInput {
    pub title: Option<String>,
    pub description: Option<String>,
    #[graphql(name = "assigneeId")]
    pub assignee_id: Option<String>,
    #[graphql(name = "scheduledDate")]
    pub scheduled_date: Option<String>,
    #[graphql(name = "scheduledTimeMinutes")]
    pub scheduled_time_minutes: Option<i32>,
    #[graphql(name = "deadlineDate")]
    pub deadline_date: Option<String>,
    #[graphql(name = "deadlineTimeMinutes")]
    pub deadline_time_minutes: Option<i32>,
    #[graphql(name = "tagIds")]
    pub tag_ids: Option<Vec<String>>,
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

    async fn create_project(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> async_graphql::Result<Project> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let normalized_name = normalize_project_name(&name);

        // Validate name
        if normalized_name.is_empty() {
            let error = async_graphql::Error::new("Project name cannot be empty")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        if normalized_name.len() > 60 {
            let error = async_graphql::Error::new("Project name cannot exceed 60 characters")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Create new project
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO projects (id, name, owner_id) VALUES (?1, ?2, ?3)")
            .bind(&id)
            .bind(&normalized_name)
            .bind(&user_id)
            .execute(pool)
            .await?;

        // Fetch the created project
        let project = sqlx::query_as::<_, (String, String, String, Option<String>, String, String)>(
            "SELECT id, name, owner_id, archived_at, created_at, updated_at FROM projects WHERE id = ?1",
        )
        .bind(&id)
        .fetch_one(pool)
        .await?;

        Ok(Project {
            id: project.0,
            name: project.1,
            owner_id: project.2,
            archived_at: project.3,
            created_at: project.4,
            updated_at: project.5,
        })
    }

    async fn rename_project(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        name: String,
        last_known_updated_at: String,
    ) -> async_graphql::Result<Project> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;
        let normalized_name = normalize_project_name(&name);

        // Validate name
        if normalized_name.is_empty() {
            let error = async_graphql::Error::new("Project name cannot be empty")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        if normalized_name.len() > 60 {
            let error = async_graphql::Error::new("Project name cannot exceed 60 characters")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Check permission (only owner can rename)
        require_owner(pool, &user_id, &project_id).await?;

        // Get current project state
        let project = sqlx::query_as::<_, (String, String, String, Option<String>, String, String)>(
            "SELECT id, name, owner_id, archived_at, created_at, updated_at FROM projects WHERE id = ?1",
        )
        .bind(&project_id)
        .fetch_one(pool)
        .await?;

        // Check for stale write
        if project.5 != last_known_updated_at {
            let error = async_graphql::Error::new("Project has been modified by another user")
                .extend_with(|_, e| e.set("code", ErrorCode::ConflictStaleWrite.as_str()));
            return Err(error);
        }

        // Update the project
        sqlx::query("UPDATE projects SET name = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2")
            .bind(&normalized_name)
            .bind(&project_id)
            .execute(pool)
            .await?;

        // Fetch updated project
        let updated_project = sqlx::query_as::<_, (String, String, String, Option<String>, String, String)>(
            "SELECT id, name, owner_id, archived_at, created_at, updated_at FROM projects WHERE id = ?1",
        )
        .bind(&project_id)
        .fetch_one(pool)
        .await?;

        Ok(Project {
            id: updated_project.0,
            name: updated_project.1,
            owner_id: updated_project.2,
            archived_at: updated_project.3,
            created_at: updated_project.4,
            updated_at: updated_project.5,
        })
    }

    async fn archive_project(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        last_known_updated_at: String,
    ) -> async_graphql::Result<Project> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Check permission (only owner can archive)
        require_owner(pool, &user_id, &project_id).await?;

        // Get current project state
        let project = sqlx::query_as::<_, (String, String, String, Option<String>, String, String)>(
            "SELECT id, name, owner_id, archived_at, created_at, updated_at FROM projects WHERE id = ?1",
        )
        .bind(&project_id)
        .fetch_one(pool)
        .await?;

        // Check for stale write
        if project.5 != last_known_updated_at {
            let error = async_graphql::Error::new("Project has been modified by another user")
                .extend_with(|_, e| e.set("code", ErrorCode::ConflictStaleWrite.as_str()));
            return Err(error);
        }

        // Archive the project
        sqlx::query("UPDATE projects SET archived_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = ?1")
            .bind(&project_id)
            .execute(pool)
            .await?;

        // Fetch updated project
        let updated_project = sqlx::query_as::<_, (String, String, String, Option<String>, String, String)>(
            "SELECT id, name, owner_id, archived_at, created_at, updated_at FROM projects WHERE id = ?1",
        )
        .bind(&project_id)
        .fetch_one(pool)
        .await?;

        Ok(Project {
            id: updated_project.0,
            name: updated_project.1,
            owner_id: updated_project.2,
            archived_at: updated_project.3,
            created_at: updated_project.4,
            updated_at: updated_project.5,
        })
    }

    async fn unarchive_project(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        last_known_updated_at: String,
    ) -> async_graphql::Result<Project> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Check permission (only owner can unarchive)
        require_owner(pool, &user_id, &project_id).await?;

        // Get current project state
        let project = sqlx::query_as::<_, (String, String, String, Option<String>, String, String)>(
            "SELECT id, name, owner_id, archived_at, created_at, updated_at FROM projects WHERE id = ?1",
        )
        .bind(&project_id)
        .fetch_one(pool)
        .await?;

        // Check for stale write
        if project.5 != last_known_updated_at {
            let error = async_graphql::Error::new("Project has been modified by another user")
                .extend_with(|_, e| e.set("code", ErrorCode::ConflictStaleWrite.as_str()));
            return Err(error);
        }

        // Unarchive the project
        sqlx::query(
            "UPDATE projects SET archived_at = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
        )
        .bind(&project_id)
        .execute(pool)
        .await?;

        // Fetch updated project
        let updated_project = sqlx::query_as::<_, (String, String, String, Option<String>, String, String)>(
            "SELECT id, name, owner_id, archived_at, created_at, updated_at FROM projects WHERE id = ?1",
        )
        .bind(&project_id)
        .fetch_one(pool)
        .await?;

        Ok(Project {
            id: updated_project.0,
            name: updated_project.1,
            owner_id: updated_project.2,
            archived_at: updated_project.3,
            created_at: updated_project.4,
            updated_at: updated_project.5,
        })
    }

    async fn add_project_member_by_username(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        username: String,
    ) -> async_graphql::Result<bool> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get current user ID
        let current_user_id =
            sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
                .bind(&claims.sub)
                .fetch_one(pool)
                .await?
                .0;

        // Check permission (only owner can add members)
        require_owner(pool, &current_user_id, &project_id).await?;

        // Get project info for owner check later
        let project = sqlx::query_as::<_, (String, String)>(
            "SELECT id, owner_id FROM projects WHERE id = ?1",
        )
        .bind(&project_id)
        .fetch_one(pool)
        .await?;

        // Find the user to add
        let target_user =
            sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
                .bind(&username.to_lowercase())
                .fetch_one(pool)
                .await;

        let target_user_id = match target_user {
            Ok(u) => u.0,
            Err(_) => {
                let error = async_graphql::Error::new("User not found")
                    .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                return Err(error);
            }
        };

        // Check if user is already a member or owner
        let existing_membership = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM project_members WHERE project_id = ?1 AND user_id = ?2",
        )
        .bind(&project_id)
        .bind(&target_user_id)
        .fetch_one(pool)
        .await?;

        if existing_membership.0 > 0 || project.1 == target_user_id {
            let error = async_graphql::Error::new("User is already a member of this project")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Add the member
        sqlx::query("INSERT INTO project_members (project_id, user_id) VALUES (?1, ?2)")
            .bind(&project_id)
            .bind(&target_user_id)
            .execute(pool)
            .await?;

        Ok(true)
    }

    async fn create_recurring_series(
        &self,
        ctx: &Context<'_>,
        input: CreateSeriesInput,
    ) -> async_graphql::Result<RecurringSeries> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID from claims
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Validate deadlineOffsetMinutes bounds (0 to 525600 minutes = 365 days)
        if input.deadline_offset_minutes < 0 || input.deadline_offset_minutes > 525600 {
            let error =
                async_graphql::Error::new("deadlineOffsetMinutes must be between 0 and 525600")
                    .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
            return Err(error);
        }

        // Validate RRULE
        let _rrule = match input.rrule.parse::<RRule<rrule::Unvalidated>>() {
            Ok(rrule) => rrule,
            Err(_) => {
                let error = async_graphql::Error::new("Invalid RRULE format")
                    .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                return Err(error);
            }
        };

        // Parse and validate dtstart date
        let dtstart_date = match NaiveDate::parse_from_str(&input.dtstart_date, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                let error =
                    async_graphql::Error::new("Invalid dtstartDate format, expected YYYY-MM-DD")
                        .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                return Err(error);
            }
        };

        // Validate timezone
        let tz = match input.timezone.parse::<Tz>() {
            Ok(tz) => tz,
            Err(_) => {
                let error = async_graphql::Error::new("Invalid timezone")
                    .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                return Err(error);
            }
        };

        // Get current time in client timezone
        let now_in_tz = Utc::now().with_timezone(&tz);
        let today_in_tz = now_in_tz.date_naive();

        // Validate that first occurrence is today or future
        if dtstart_date < today_in_tz {
            let error =
                async_graphql::Error::new("First occurrence must be today or in the future")
                    .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
            return Err(error);
        }

        // If time is present, validate that first datetime is >= now (in client timezone)
        if let Some(time_minutes) = input.dtstart_time_minutes {
            if time_minutes < 0 || time_minutes >= 1440 {
                let error =
                    async_graphql::Error::new("dtstartTimeMinutes must be between 0 and 1439")
                        .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                return Err(error);
            }

            if dtstart_date == today_in_tz {
                let hours = time_minutes / 60;
                let minutes = time_minutes % 60;
                let dtstart_time = match NaiveTime::from_hms_opt(hours as u32, minutes as u32, 0) {
                    Some(time) => time,
                    None => {
                        let error = async_graphql::Error::new("Invalid dtstartTimeMinutes")
                            .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                        return Err(error);
                    }
                };

                let dtstart_datetime = dtstart_date.and_time(dtstart_time);
                let dtstart_in_tz = tz.from_local_datetime(&dtstart_datetime).single();

                if let Some(dtstart_in_tz) = dtstart_in_tz {
                    if dtstart_in_tz < now_in_tz {
                        let error = async_graphql::Error::new(
                            "First occurrence datetime must be >= now in client timezone",
                        )
                        .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                        return Err(error);
                    }
                } else {
                    let error = async_graphql::Error::new("Invalid datetime in client timezone")
                        .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                    return Err(error);
                }
            }
        }

        // Validate project exists and user has access
        require_member(pool, &user_id, &input.project_id).await?;

        // Validate assignee exists if provided
        if let Some(ref assignee_id) = input.assignee_id {
            let assignee_exists =
                sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM users WHERE id = ?1")
                    .bind(assignee_id)
                    .fetch_one(pool)
                    .await?;

            if assignee_exists.0 == 0 {
                let error = async_graphql::Error::new("Assignee not found")
                    .extend_with(|_, e| e.set("code", "NOT_FOUND"));
                return Err(error);
            }
        }

        // Validate and normalize defaultTagIds
        let default_tag_ids = input.default_tag_ids.unwrap_or_default();
        for tag_id in &default_tag_ids {
            let tag_exists = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tags WHERE id = ?1")
                .bind(tag_id)
                .fetch_one(pool)
                .await?;

            if tag_exists.0 == 0 {
                let error = async_graphql::Error::new("One or more tags not found")
                    .extend_with(|_, e| e.set("code", "NOT_FOUND"));
                return Err(error);
            }
        }

        // Create the recurring series
        let series_id = uuid::Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO recurring_series 
             (id, project_id, created_by, title, description, assignee_id, rrule, dtstart_date, dtstart_time_minutes, deadline_offset_minutes) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
        )
        .bind(&series_id)
        .bind(&input.project_id)
        .bind(&user_id)
        .bind(&input.title)
        .bind(&input.description)
        .bind(&input.assignee_id)
        .bind(&input.rrule)
        .bind(&input.dtstart_date)
        .bind(input.dtstart_time_minutes)
        .bind(input.deadline_offset_minutes)
        .execute(pool)
        .await?;

        // Insert default tags
        for tag_id in &default_tag_ids {
            sqlx::query("INSERT INTO recurring_series_tags (series_id, tag_id) VALUES (?1, ?2)")
                .bind(&series_id)
                .bind(tag_id)
                .execute(pool)
                .await?;
        }

        // Fetch the created series
        let series = sqlx::query_as::<_, (String, String, String, String, Option<String>, Option<String>, String, String, Option<i32>, i32, String, String)>(
            "SELECT id, project_id, created_by, title, description, assignee_id, rrule, dtstart_date, dtstart_time_minutes, deadline_offset_minutes, created_at, updated_at 
             FROM recurring_series WHERE id = ?1"
        )
        .bind(&series_id)
        .fetch_one(pool)
        .await?;

        Ok(RecurringSeries {
            id: series.0,
            project_id: series.1,
            created_by: series.2,
            title: series.3,
            description: series.4,
            assignee_id: series.5,
            rrule: series.6,
            dtstart_date: series.7,
            dtstart_time_minutes: series.8,
            deadline_offset_minutes: series.9,
            created_at: series.10,
            updated_at: series.11,
            default_tag_ids,
        })
    }

    async fn create_saved_view(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        name: String,
        filters: crate::graphql::SavedViewFiltersInput,
    ) -> async_graphql::Result<crate::graphql::SavedView> {
        use crate::db::helpers::normalize_project_name; // Reuse for general name normalization
        use crate::graphql::{SavedView, SavedViewFilters};

        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Check if user has access to this project
        require_member(pool, &user_id, &project_id).await?;

        // Validate and normalize name
        let normalized_name = normalize_project_name(&name); // Reuse existing normalization
        if normalized_name.is_empty() {
            let error = async_graphql::Error::new("Saved view name cannot be empty")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        if normalized_name.len() > 60 {
            let error = async_graphql::Error::new("Saved view name cannot exceed 60 characters")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Check for unique name per project (case-insensitive)
        let existing_count = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM saved_views WHERE project_id = ?1 AND LOWER(TRIM(name)) = LOWER(TRIM(?2))"
        )
        .bind(&project_id)
        .bind(&normalized_name)
        .fetch_one(pool)
        .await?;

        if existing_count.0 > 0 {
            let error = async_graphql::Error::new(
                "A saved view with this name already exists in this project",
            )
            .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Validate assignee exists if provided
        if let Some(ref assignee_id) = filters.assignee {
            let assignee_exists =
                sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM users WHERE id = ?1")
                    .bind(assignee_id)
                    .fetch_one(pool)
                    .await?;

            if assignee_exists.0 == 0 {
                let error = async_graphql::Error::new("Assignee not found")
                    .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                return Err(error);
            }
        }

        // Validate tag IDs exist
        for tag_id in &filters.tag_ids {
            let tag_exists = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tags WHERE id = ?1")
                .bind(tag_id)
                .fetch_one(pool)
                .await?;

            if tag_exists.0 == 0 {
                let error = async_graphql::Error::new("One or more tags not found")
                    .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                return Err(error);
            }
        }

        // Convert input filters to JSON
        let filters_obj = SavedViewFilters {
            statuses: filters.statuses,
            assignee: filters.assignee,
            include_unassigned: filters.include_unassigned,
            assigned_to_me: filters.assigned_to_me,
            tag_ids: filters.tag_ids,
        };

        let filters_json = serde_json::to_string(&filters_obj).map_err(|_| {
            async_graphql::Error::new("Failed to serialize filters")
                .extend_with(|_, e| e.set("code", ErrorCode::Internal.as_str()))
        })?;

        // Create the saved view
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO saved_views (id, project_id, name, filters, created_by) VALUES (?1, ?2, ?3, ?4, ?5)"
        )
        .bind(&id)
        .bind(&project_id)
        .bind(&normalized_name)
        .bind(&filters_json)
        .bind(&user_id)
        .execute(pool)
        .await?;

        // Fetch the created saved view
        let saved_view = sqlx::query_as::<_, (String, String, String, String, String, String, String)>(
            "SELECT id, project_id, name, filters, created_by, created_at, updated_at FROM saved_views WHERE id = ?1"
        )
        .bind(&id)
        .fetch_one(pool)
        .await?;

        Ok(SavedView {
            id: saved_view.0,
            project_id: saved_view.1,
            name: saved_view.2,
            filters: filters_obj,
            created_by: saved_view.4,
            created_at: saved_view.5,
            updated_at: saved_view.6,
        })
    }

    async fn update_saved_view(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: Option<String>,
        filters: Option<crate::graphql::SavedViewFiltersInput>,
        last_known_updated_at: String,
    ) -> async_graphql::Result<crate::graphql::SavedView> {
        use crate::db::helpers::normalize_project_name;
        use crate::graphql::{SavedView, SavedViewFilters};

        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Get current saved view
        let current = sqlx::query_as::<_, (String, String, String, String, String, String, String)>(
            "SELECT id, project_id, name, filters, created_by, created_at, updated_at FROM saved_views WHERE id = ?1"
        )
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            async_graphql::Error::new("Saved view not found")
                .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()))
        })?;

        // Check if user has access to this project
        require_member(pool, &user_id, &current.1).await?;

        // Check for stale write
        if current.6 != last_known_updated_at {
            let error = async_graphql::Error::new("Saved view has been modified by another user")
                .extend_with(|_, e| e.set("code", ErrorCode::ConflictStaleWrite.as_str()));
            return Err(error);
        }

        // Prepare updated values
        let updated_name = if let Some(name) = name {
            let normalized_name = normalize_project_name(&name);
            if normalized_name.is_empty() {
                let error = async_graphql::Error::new("Saved view name cannot be empty")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }

            if normalized_name.len() > 60 {
                let error =
                    async_graphql::Error::new("Saved view name cannot exceed 60 characters")
                        .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }

            // Check for unique name per project (case-insensitive), excluding current view
            let existing_count = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM saved_views WHERE project_id = ?1 AND LOWER(TRIM(name)) = LOWER(TRIM(?2)) AND id != ?3"
            )
            .bind(&current.1)
            .bind(&normalized_name)
            .bind(&id)
            .fetch_one(pool)
            .await?;

            if existing_count.0 > 0 {
                let error = async_graphql::Error::new(
                    "A saved view with this name already exists in this project",
                )
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }

            normalized_name
        } else {
            current.2.clone()
        };

        let updated_filters_json = if let Some(filters) = filters {
            // Validate assignee exists if provided
            if let Some(ref assignee_id) = filters.assignee {
                let assignee_exists =
                    sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM users WHERE id = ?1")
                        .bind(assignee_id)
                        .fetch_one(pool)
                        .await?;

                if assignee_exists.0 == 0 {
                    let error = async_graphql::Error::new("Assignee not found")
                        .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                    return Err(error);
                }
            }

            // Validate tag IDs exist
            for tag_id in &filters.tag_ids {
                let tag_exists =
                    sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tags WHERE id = ?1")
                        .bind(tag_id)
                        .fetch_one(pool)
                        .await?;

                if tag_exists.0 == 0 {
                    let error = async_graphql::Error::new("One or more tags not found")
                        .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                    return Err(error);
                }
            }

            let filters_obj = SavedViewFilters {
                statuses: filters.statuses,
                assignee: filters.assignee,
                include_unassigned: filters.include_unassigned,
                assigned_to_me: filters.assigned_to_me,
                tag_ids: filters.tag_ids,
            };

            serde_json::to_string(&filters_obj).map_err(|_| {
                async_graphql::Error::new("Failed to serialize filters")
                    .extend_with(|_, e| e.set("code", ErrorCode::Internal.as_str()))
            })?
        } else {
            current.3.clone()
        };

        // Update the saved view
        sqlx::query(
            "UPDATE saved_views SET name = ?1, filters = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?3"
        )
        .bind(&updated_name)
        .bind(&updated_filters_json)
        .bind(&id)
        .execute(pool)
        .await?;

        // Fetch updated saved view
        let updated = sqlx::query_as::<_, (String, String, String, String, String, String, String)>(
            "SELECT id, project_id, name, filters, created_by, created_at, updated_at FROM saved_views WHERE id = ?1"
        )
        .bind(&id)
        .fetch_one(pool)
        .await?;

        let filters: SavedViewFilters = serde_json::from_str(&updated.3).map_err(|_| {
            async_graphql::Error::new("Invalid saved view filters")
                .extend_with(|_, e| e.set("code", ErrorCode::Internal.as_str()))
        })?;

        Ok(SavedView {
            id: updated.0,
            project_id: updated.1,
            name: updated.2,
            filters,
            created_by: updated.4,
            created_at: updated.5,
            updated_at: updated.6,
        })
    }

    async fn delete_saved_view(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<bool> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Get saved view to check project access
        let saved_view = sqlx::query_as::<_, (String, String)>(
            "SELECT id, project_id FROM saved_views WHERE id = ?1",
        )
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            async_graphql::Error::new("Saved view not found")
                .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()))
        })?;

        // Check if user has access to this project
        require_member(pool, &user_id, &saved_view.1).await?;

        // Remove from default view if it's set as default
        sqlx::query("DELETE FROM project_default_view WHERE saved_view_id = ?1")
            .bind(&id)
            .execute(pool)
            .await?;

        // Delete the saved view
        let result = sqlx::query("DELETE FROM saved_views WHERE id = ?1")
            .bind(&id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn set_project_default_saved_view(
        &self,
        ctx: &Context<'_>,
        project_id: String,
        saved_view_id: Option<String>,
    ) -> async_graphql::Result<bool> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Check if user has access to this project
        require_member(pool, &user_id, &project_id).await?;

        if let Some(view_id) = saved_view_id {
            // Validate that the saved view exists and belongs to this project
            let view_exists = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM saved_views WHERE id = ?1 AND project_id = ?2",
            )
            .bind(&view_id)
            .bind(&project_id)
            .fetch_one(pool)
            .await?;

            if view_exists.0 == 0 {
                let error = async_graphql::Error::new("Saved view not found in this project")
                    .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                return Err(error);
            }

            // Set or update the default view
            sqlx::query(
                "INSERT OR REPLACE INTO project_default_view (project_id, saved_view_id) VALUES (?1, ?2)"
            )
            .bind(&project_id)
            .bind(&view_id)
            .execute(pool)
            .await?;
        } else {
            // Clear the default view
            sqlx::query("DELETE FROM project_default_view WHERE project_id = ?1")
                .bind(&project_id)
                .execute(pool)
                .await?;
        }

        Ok(true)
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

#[derive(SimpleObject)]
pub struct RecurringSeries {
    id: String,
    #[graphql(name = "projectId")]
    project_id: String,
    #[graphql(name = "createdBy")]
    created_by: String,
    title: String,
    description: Option<String>,
    #[graphql(name = "assigneeId")]
    assignee_id: Option<String>,
    rrule: String,
    #[graphql(name = "dtstartDate")]
    dtstart_date: String,
    #[graphql(name = "dtstartTimeMinutes")]
    dtstart_time_minutes: Option<i32>,
    #[graphql(name = "deadlineOffsetMinutes")]
    deadline_offset_minutes: i32,
    #[graphql(name = "createdAt")]
    created_at: String,
    #[graphql(name = "updatedAt")]
    updated_at: String,
    #[graphql(name = "defaultTagIds")]
    default_tag_ids: Vec<String>,
}

#[derive(InputObject)]
pub struct CreateSeriesInput {
    #[graphql(name = "projectId")]
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
    #[graphql(name = "assigneeId")]
    pub assignee_id: Option<String>,
    #[graphql(name = "defaultTagIds")]
    pub default_tag_ids: Option<Vec<String>>,
    pub rrule: String,
    #[graphql(name = "dtstartDate")]
    pub dtstart_date: String,
    #[graphql(name = "dtstartTimeMinutes")]
    pub dtstart_time_minutes: Option<i32>,
    #[graphql(name = "deadlineOffsetMinutes")]
    pub deadline_offset_minutes: i32,
    pub timezone: String,
}

impl AuthenticatedMutation {
    // Task mutations
    async fn create_task(
        &self,
        ctx: &Context<'_>,
        input: CreateTaskInput,
    ) -> async_graphql::Result<Task> {
        use crate::tasks::time_utils;

        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Check if user has access to this project and project is not archived
        require_member(pool, &user_id, &input.project_id).await?;

        // Check if project is archived (read-only)
        let project = sqlx::query_as::<_, (Option<String>,)>(
            "SELECT archived_at FROM projects WHERE id = ?1",
        )
        .bind(&input.project_id)
        .fetch_one(pool)
        .await?;

        if project.0.is_some() {
            let error = async_graphql::Error::new("Cannot create tasks in archived project")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Validate title
        if input.title.trim().is_empty() {
            let error = async_graphql::Error::new("Title cannot be empty")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        if input.title.len() > 120 {
            let error = async_graphql::Error::new("Title cannot exceed 120 characters")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Validate description
        if let Some(ref desc) = input.description {
            if desc.len() > 5000 {
                let error = async_graphql::Error::new("Description cannot exceed 5000 characters")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }

        // Validate time minutes are in bounds
        if let Some(minutes) = input.scheduled_time_minutes {
            if minutes < 0 || minutes >= 1440 {
                let error =
                    async_graphql::Error::new("scheduledTimeMinutes must be between 0 and 1439")
                        .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }

        if let Some(minutes) = input.deadline_time_minutes {
            if minutes < 0 || minutes >= 1440 {
                let error =
                    async_graphql::Error::new("deadlineTimeMinutes must be between 0 and 1439")
                        .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }

        // Validate assignee exists if provided, default to creator
        let assignee_id = if let Some(assignee_id) = input.assignee_id {
            let assignee_exists =
                sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM users WHERE id = ?1")
                    .bind(&assignee_id)
                    .fetch_one(pool)
                    .await?;

            if assignee_exists.0 == 0 {
                let error = async_graphql::Error::new("Assignee not found")
                    .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                return Err(error);
            }
            assignee_id
        } else {
            user_id.clone()
        };

        // Validate tag IDs exist
        let tag_ids = input.tag_ids.unwrap_or_default();
        for tag_id in &tag_ids {
            let tag_exists = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tags WHERE id = ?1")
                .bind(tag_id)
                .fetch_one(pool)
                .await?;

            if tag_exists.0 == 0 {
                let error = async_graphql::Error::new("One or more tags not found")
                    .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                return Err(error);
            }
        }

        // Create the task
        let task_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO tasks (id, project_id, author_id, assignee_id, title, description, status, scheduled_date, scheduled_time_minutes, deadline_date, deadline_time_minutes) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'todo', ?7, ?8, ?9, ?10)"
        )
        .bind(&task_id)
        .bind(&input.project_id)
        .bind(&user_id)
        .bind(&assignee_id)
        .bind(&input.title)
        .bind(&input.description)
        .bind(&input.scheduled_date)
        .bind(input.scheduled_time_minutes)
        .bind(&input.deadline_date)
        .bind(input.deadline_time_minutes)
        .execute(pool)
        .await?;

        // Insert task tags
        for tag_id in &tag_ids {
            sqlx::query("INSERT INTO task_tags (task_id, tag_id) VALUES (?1, ?2)")
                .bind(&task_id)
                .bind(tag_id)
                .execute(pool)
                .await?;
        }

        // Fetch the created task and return with derived fields (using UTC timezone for now)
        self.read_task_after_write(pool, &task_id).await
    }

    // Helper method to read task after write operations
    async fn read_task_after_write(
        &self,
        pool: &SqlitePool,
        task_id: &str,
    ) -> async_graphql::Result<Task> {
        use crate::tasks::{TaskBucket, TaskStatus, time_utils};

        // Default timezone for derived fields (this should ideally come from request context)
        let tz = time_utils::parse_timezone("UTC").unwrap();

        let row = sqlx::query(
            "SELECT id, project_id, author_id, assignee_id, series_id, title, description, 
                    status, scheduled_date, scheduled_time_minutes, deadline_date, deadline_time_minutes,
                    completed_at, completed_by, abandoned_at, abandoned_by, created_at, updated_at
             FROM tasks WHERE id = ?1"
        )
        .bind(task_id)
        .fetch_one(pool)
        .await?;

        let id: String = row.get("id");
        let project_id: String = row.get("project_id");
        let author_id: String = row.get("author_id");
        let assignee_id: Option<String> = row.get("assignee_id");
        let series_id: Option<String> = row.get("series_id");
        let title: String = row.get("title");
        let description: Option<String> = row.get("description");
        let status_str: String = row.get("status");
        let scheduled_date: Option<String> = row.get("scheduled_date");
        let scheduled_time_minutes: Option<i32> = row.get("scheduled_time_minutes");
        let deadline_date: Option<String> = row.get("deadline_date");
        let deadline_time_minutes: Option<i32> = row.get("deadline_time_minutes");
        let completed_at: Option<String> = row.get("completed_at");
        let completed_by: Option<String> = row.get("completed_by");
        let abandoned_at: Option<String> = row.get("abandoned_at");
        let abandoned_by: Option<String> = row.get("abandoned_by");
        let created_at: String = row.get("created_at");
        let updated_at: String = row.get("updated_at");

        let status = match status_str.as_str() {
            "todo" => TaskStatus::Todo,
            "done" => TaskStatus::Done,
            "abandoned" => TaskStatus::Abandoned,
            _ => TaskStatus::Todo,
        };

        // Compute derived fields
        let is_overdue = time_utils::is_task_overdue(
            scheduled_date.as_deref(),
            scheduled_time_minutes,
            deadline_date.as_deref(),
            deadline_time_minutes,
            tz,
        );

        let bucket = time_utils::get_task_bucket(
            scheduled_date.as_deref(),
            scheduled_time_minutes,
            deadline_date.as_deref(),
            deadline_time_minutes,
            tz,
        );

        Ok(Task {
            id,
            project_id,
            author_id,
            assignee_id,
            series_id,
            title,
            description,
            status,
            scheduled_date,
            scheduled_time_minutes,
            deadline_date,
            deadline_time_minutes,
            completed_at,
            completed_by,
            abandoned_at,
            abandoned_by,
            created_at,
            updated_at,
            is_overdue,
            bucket,
        })
    }

    async fn update_task(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateTaskInput,
        last_known_updated_at: String,
    ) -> async_graphql::Result<Task> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Get current task
        let current_task = sqlx::query_as::<_, (String, String, String, String, String)>(
            "SELECT id, project_id, status, updated_at, series_id FROM tasks WHERE id = ?1",
        )
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            async_graphql::Error::new("Task not found")
                .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()))
        })?;

        // Check if user has access to this project
        require_member(pool, &user_id, &current_task.1).await?;

        // Check if project is archived (read-only)
        let project = sqlx::query_as::<_, (Option<String>,)>(
            "SELECT archived_at FROM projects WHERE id = ?1",
        )
        .bind(&current_task.1)
        .fetch_one(pool)
        .await?;

        if project.0.is_some() {
            let error = async_graphql::Error::new("Cannot edit tasks in archived project")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Only allow editing todo tasks
        if current_task.2 != "todo" {
            let error = async_graphql::Error::new("Can only edit todo tasks")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Check for stale write
        if current_task.3 != last_known_updated_at {
            let error = async_graphql::Error::new("Task has been modified by another user")
                .extend_with(|_, e| e.set("code", ErrorCode::ConflictStaleWrite.as_str()));
            return Err(error);
        }

        // Validate title if provided
        if let Some(ref title) = input.title {
            if title.trim().is_empty() {
                let error = async_graphql::Error::new("Title cannot be empty")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }

            if title.len() > 120 {
                let error = async_graphql::Error::new("Title cannot exceed 120 characters")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }

        // Validate description if provided
        if let Some(ref desc) = input.description {
            if desc.len() > 5000 {
                let error = async_graphql::Error::new("Description cannot exceed 5000 characters")
                    .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }

        // Validate time minutes if provided
        if let Some(minutes) = input.scheduled_time_minutes {
            if minutes < 0 || minutes >= 1440 {
                let error =
                    async_graphql::Error::new("scheduledTimeMinutes must be between 0 and 1439")
                        .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }

        if let Some(minutes) = input.deadline_time_minutes {
            if minutes < 0 || minutes >= 1440 {
                let error =
                    async_graphql::Error::new("deadlineTimeMinutes must be between 0 and 1439")
                        .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
                return Err(error);
            }
        }

        // Validate assignee if provided
        if let Some(ref assignee_id) = input.assignee_id {
            let assignee_exists =
                sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM users WHERE id = ?1")
                    .bind(assignee_id)
                    .fetch_one(pool)
                    .await?;

            if assignee_exists.0 == 0 {
                let error = async_graphql::Error::new("Assignee not found")
                    .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                return Err(error);
            }
        }

        // Validate tag IDs if provided
        if let Some(ref tag_ids) = input.tag_ids {
            for tag_id in tag_ids {
                let tag_exists =
                    sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tags WHERE id = ?1")
                        .bind(tag_id)
                        .fetch_one(pool)
                        .await?;

                if tag_exists.0 == 0 {
                    let error = async_graphql::Error::new("One or more tags not found")
                        .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()));
                    return Err(error);
                }
            }
        }

        // Update fields individually to avoid complex parameter binding
        let mut any_updates = false;

        if let Some(title) = &input.title {
            sqlx::query(
                "UPDATE tasks SET title = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            )
            .bind(title)
            .bind(&id)
            .execute(pool)
            .await?;
            any_updates = true;
        }

        if let Some(description) = &input.description {
            sqlx::query(
                "UPDATE tasks SET description = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            )
            .bind(description)
            .bind(&id)
            .execute(pool)
            .await?;
            any_updates = true;
        }

        if let Some(assignee_id) = &input.assignee_id {
            sqlx::query(
                "UPDATE tasks SET assignee_id = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            )
            .bind(assignee_id)
            .bind(&id)
            .execute(pool)
            .await?;
            any_updates = true;
        }

        if let Some(scheduled_date) = &input.scheduled_date {
            sqlx::query("UPDATE tasks SET scheduled_date = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2")
                .bind(scheduled_date)
                .bind(&id)
                .execute(pool)
                .await?;
            any_updates = true;
        }

        if let Some(scheduled_time_minutes) = input.scheduled_time_minutes {
            sqlx::query("UPDATE tasks SET scheduled_time_minutes = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2")
                .bind(scheduled_time_minutes)
                .bind(&id)
                .execute(pool)
                .await?;
            any_updates = true;
        }

        if let Some(deadline_date) = &input.deadline_date {
            sqlx::query(
                "UPDATE tasks SET deadline_date = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            )
            .bind(deadline_date)
            .bind(&id)
            .execute(pool)
            .await?;
            any_updates = true;
        }

        if let Some(deadline_time_minutes) = input.deadline_time_minutes {
            sqlx::query("UPDATE tasks SET deadline_time_minutes = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2")
                .bind(deadline_time_minutes)
                .bind(&id)
                .execute(pool)
                .await?;
            any_updates = true;
        }

        // If no field updates but still want to update timestamp for consistency
        if !any_updates {
            sqlx::query("UPDATE tasks SET updated_at = CURRENT_TIMESTAMP WHERE id = ?1")
                .bind(&id)
                .execute(pool)
                .await?;
        }

        // Update tags if provided
        if let Some(tag_ids) = &input.tag_ids {
            // Delete existing tags
            sqlx::query("DELETE FROM task_tags WHERE task_id = ?1")
                .bind(&id)
                .execute(pool)
                .await?;

            // Insert new tags
            for tag_id in tag_ids {
                sqlx::query("INSERT INTO task_tags (task_id, tag_id) VALUES (?1, ?2)")
                    .bind(&id)
                    .bind(tag_id)
                    .execute(pool)
                    .await?;
            }
        }

        self.read_task_after_write(pool, &id).await
    }

    async fn complete_task(
        &self,
        ctx: &Context<'_>,
        id: String,
        last_known_updated_at: String,
    ) -> async_graphql::Result<Task> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Get current task
        let current_task = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, project_id, status, updated_at FROM tasks WHERE id = ?1",
        )
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            async_graphql::Error::new("Task not found")
                .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()))
        })?;

        // Check if user has access to this project
        require_member(pool, &user_id, &current_task.1).await?;

        // Only allow completing todo tasks
        if current_task.2 != "todo" {
            let error = async_graphql::Error::new("Can only complete todo tasks")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Check for stale write
        if current_task.3 != last_known_updated_at {
            let error = async_graphql::Error::new("Task has been modified by another user")
                .extend_with(|_, e| e.set("code", ErrorCode::ConflictStaleWrite.as_str()));
            return Err(error);
        }

        // Complete the task
        sqlx::query(
            "UPDATE tasks SET status = 'done', completed_at = CURRENT_TIMESTAMP, completed_by = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2"
        )
        .bind(&user_id)
        .bind(&id)
        .execute(pool)
        .await?;

        self.read_task_after_write(pool, &id).await
    }

    async fn abandon_task(
        &self,
        ctx: &Context<'_>,
        id: String,
        last_known_updated_at: String,
    ) -> async_graphql::Result<Task> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Get current task
        let current_task = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, project_id, status, updated_at FROM tasks WHERE id = ?1",
        )
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            async_graphql::Error::new("Task not found")
                .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()))
        })?;

        // Check if user has access to this project
        require_member(pool, &user_id, &current_task.1).await?;

        // Only allow abandoning todo tasks
        if current_task.2 != "todo" {
            let error = async_graphql::Error::new("Can only abandon todo tasks")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Check for stale write
        if current_task.3 != last_known_updated_at {
            let error = async_graphql::Error::new("Task has been modified by another user")
                .extend_with(|_, e| e.set("code", ErrorCode::ConflictStaleWrite.as_str()));
            return Err(error);
        }

        // Abandon the task
        sqlx::query(
            "UPDATE tasks SET status = 'abandoned', abandoned_at = CURRENT_TIMESTAMP, abandoned_by = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2"
        )
        .bind(&user_id)
        .bind(&id)
        .execute(pool)
        .await?;

        self.read_task_after_write(pool, &id).await
    }

    async fn restore_task(
        &self,
        ctx: &Context<'_>,
        id: String,
        last_known_updated_at: String,
    ) -> async_graphql::Result<Task> {
        // Require authentication
        let claims = match ctx.data_opt::<Arc<Claims>>() {
            Some(claims) => claims,
            None => {
                return Err(async_graphql::Error::new("Authentication required"));
            }
        };

        let pool = ctx.data::<SqlitePool>()?;

        // Get user ID
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;

        // Get current task
        let current_task = sqlx::query_as::<_, (String, String, String, String)>(
            "SELECT id, project_id, status, updated_at FROM tasks WHERE id = ?1",
        )
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            async_graphql::Error::new("Task not found")
                .extend_with(|_, e| e.set("code", ErrorCode::NotFound.as_str()))
        })?;

        // Check if user has access to this project
        require_member(pool, &user_id, &current_task.1).await?;

        // Only allow restoring abandoned tasks
        if current_task.2 != "abandoned" {
            let error = async_graphql::Error::new("Can only restore abandoned tasks")
                .extend_with(|_, e| e.set("code", ErrorCode::ValidationFailed.as_str()));
            return Err(error);
        }

        // Check for stale write
        if current_task.3 != last_known_updated_at {
            let error = async_graphql::Error::new("Task has been modified by another user")
                .extend_with(|_, e| e.set("code", ErrorCode::ConflictStaleWrite.as_str()));
            return Err(error);
        }

        // Restore the task (clear abandoned fields)
        sqlx::query(
            "UPDATE tasks SET status = 'todo', abandoned_at = NULL, abandoned_by = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ?1"
        )
        .bind(&id)
        .execute(pool)
        .await?;

        self.read_task_after_write(pool, &id).await
    }
}
