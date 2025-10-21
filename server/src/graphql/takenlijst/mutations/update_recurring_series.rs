use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::graphql::takenlijst::types::{RecurringSeries, UpdateSeriesInput};
use async_graphql::{Context, ErrorExtensions, Object};
use chrono::{Datelike, NaiveDate, NaiveTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use rrule::{RRule, RRuleSet};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct UpdateRecurringSeriesMutation;

#[Object]
impl UpdateRecurringSeriesMutation {
    async fn update_recurring_series(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateSeriesInput,
        #[graphql(name = "lastKnownUpdatedAt")] last_known_updated_at: String,
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

        // Validate timezone
        let tz = match input.timezone.parse::<Tz>() {
            Ok(tz) => tz,
            Err(_) => {
                let error = async_graphql::Error::new("Invalid timezone")
                    .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                return Err(error);
            }
        };

        // Fetch the existing series and check concurrency
        let existing_series = sqlx::query_as::<_, (String, String, String, String, Option<String>, Option<String>, String, String, Option<i32>, i32, String, String)>(
            "SELECT id, project_id, created_by, title, description, assignee_id, rrule, dtstart_date, dtstart_time_minutes, deadline_offset_minutes, created_at, updated_at 
             FROM recurring_series WHERE id = ?1"
        )
        .bind(&id)
        .fetch_optional(pool)
        .await?;

        let existing_series = match existing_series {
            Some(series) => series,
            None => {
                let error = async_graphql::Error::new("Recurring series not found")
                    .extend_with(|_, e| e.set("code", "NOT_FOUND"));
                return Err(error);
            }
        };

        // Check stale write protection
        if existing_series.11 != last_known_updated_at {
            let error = async_graphql::Error::new("Stale write detected")
                .extend_with(|_, e| e.set("code", "CONFLICT_STALE_WRITE"));
            return Err(error);
        }

        // Validate project access
        require_member(pool, &user_id, &existing_series.1).await?;

        // Prepare updated values, using existing values where input is None
        let new_title = input
            .title
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or(existing_series.3.clone());
        let new_description = input
            .description
            .as_ref()
            .map(|s| s.clone())
            .or(existing_series.4.clone());
        let new_assignee_id = input
            .assignee_id
            .as_ref()
            .map(|s| s.clone())
            .or(existing_series.5.clone());
        let new_rrule = input
            .rrule
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or(existing_series.6.clone());
        let new_dtstart_date = input
            .dtstart_date
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or(existing_series.7.clone());
        let new_dtstart_time_minutes = input.dtstart_time_minutes.or(existing_series.8);
        let new_deadline_offset_minutes =
            input.deadline_offset_minutes.unwrap_or(existing_series.9);

        // Validate new values
        if let Some(deadline_offset) = input.deadline_offset_minutes {
            if deadline_offset < 0 || deadline_offset > 525600 {
                let error =
                    async_graphql::Error::new("deadlineOffsetMinutes must be between 0 and 525600")
                        .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                return Err(error);
            }
        }

        // Validate RRULE if provided
        if input.rrule.is_some() {
            let _rrule = match new_rrule.parse::<RRule<rrule::Unvalidated>>() {
                Ok(rrule) => rrule,
                Err(_) => {
                    let error = async_graphql::Error::new("Invalid RRULE format")
                        .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                    return Err(error);
                }
            };
        }

        // Parse and validate dtstart date if provided
        if input.dtstart_date.is_some() {
            let dtstart_date = match NaiveDate::parse_from_str(&new_dtstart_date, "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {
                    let error = async_graphql::Error::new(
                        "Invalid dtstartDate format, expected YYYY-MM-DD",
                    )
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
            if let Some(time_minutes) = new_dtstart_time_minutes {
                if time_minutes < 0 || time_minutes >= 1440 {
                    let error =
                        async_graphql::Error::new("dtstartTimeMinutes must be between 0 and 1439")
                            .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                    return Err(error);
                }

                if dtstart_date == today_in_tz {
                    let hours = time_minutes / 60;
                    let minutes = time_minutes % 60;
                    let dtstart_time =
                        match NaiveTime::from_hms_opt(hours as u32, minutes as u32, 0) {
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
                        let error =
                            async_graphql::Error::new("Invalid datetime in client timezone")
                                .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"));
                        return Err(error);
                    }
                }
            }
        }

        // Validate assignee exists if provided
        if let Some(ref assignee_id) = new_assignee_id {
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

        // Validate and normalize defaultTagIds if provided
        let new_default_tag_ids = input
            .default_tag_ids
            .as_ref()
            .map(|v| v.clone())
            .unwrap_or_else(|| {
                // Get existing tag IDs
                Vec::new() // We'll populate this below
            });

        if input.default_tag_ids.is_some() {
            for tag_id in &new_default_tag_ids {
                let tag_exists =
                    sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tags WHERE id = ?1")
                        .bind(tag_id)
                        .fetch_one(pool)
                        .await?;

                if tag_exists.0 == 0 {
                    let error = async_graphql::Error::new("One or more tags not found")
                        .extend_with(|_, e| e.set("code", "NOT_FOUND"));
                    return Err(error);
                }
            }
        }

        // Get existing default tag IDs if not provided in input
        let final_default_tag_ids = if input.default_tag_ids.is_some() {
            new_default_tag_ids
        } else {
            sqlx::query_as::<_, (String,)>(
                "SELECT tag_id FROM recurring_series_tags WHERE series_id = ?1",
            )
            .bind(&id)
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|row| row.0)
            .collect()
        };

        // Start a transaction
        let mut tx = pool.begin().await?;

        // Normalize RRULE for date-only series by stripping time-based parts (BYHOUR/BYMINUTE/BYSECOND)
        let has_time_input = new_dtstart_time_minutes.is_some();
        let normalized_rrule = if has_time_input {
            new_rrule.clone()
        } else {
            let parts: Vec<&str> = new_rrule.split(';').collect();
            let kept: Vec<&str> = parts
                .into_iter()
                .filter(|p| {
                    let up = p.to_ascii_uppercase();
                    !(up.starts_with("BYHOUR=")
                        || up.starts_with("BYMINUTE=")
                        || up.starts_with("BYSECOND="))
                })
                .collect();
            kept.join(";")
        };

        // Update the recurring series
        sqlx::query(
            "UPDATE recurring_series 
             SET title = ?1, description = ?2, assignee_id = ?3, rrule = ?4, dtstart_date = ?5, dtstart_time_minutes = ?6, deadline_offset_minutes = ?7, updated_at = (strftime('%Y-%m-%d %H:%M:%f','now'))
             WHERE id = ?8"
        )
        .bind(&new_title)
        .bind(&new_description)
        .bind(&new_assignee_id)
        .bind(&normalized_rrule)
        .bind(&new_dtstart_date)
        .bind(new_dtstart_time_minutes)
        .bind(new_deadline_offset_minutes)
        .bind(&id)
        .execute(&mut *tx)
        .await?;

        // Update default tags if provided
        if input.default_tag_ids.is_some() {
            // Delete existing tags
            sqlx::query("DELETE FROM recurring_series_tags WHERE series_id = ?1")
                .bind(&id)
                .execute(&mut *tx)
                .await?;

            // Insert new tags
            for tag_id in &final_default_tag_ids {
                sqlx::query(
                    "INSERT INTO recurring_series_tags (series_id, tag_id) VALUES (?1, ?2)",
                )
                .bind(&id)
                .bind(tag_id)
                .execute(&mut *tx)
                .await?;
            }
        }

        // Check if we need to regenerate occurrences (if rrule, dtstart, or deadline offset changed)
        let rrule_changed = input.rrule.is_some();
        let dtstart_date_changed = input.dtstart_date.is_some();
        let dtstart_time_changed = input.dtstart_time_minutes.is_some();
        let deadline_offset_changed = input.deadline_offset_minutes.is_some();
        let needs_regeneration = rrule_changed
            || dtstart_date_changed
            || dtstart_time_changed
            || deadline_offset_changed;

        if needs_regeneration {
            // Delete future todo occurrences (not completed/abandoned) from now onward
            let now_in_tz = Utc::now().with_timezone(&tz);
            let today_str = now_in_tz.date_naive().format("%Y-%m-%d").to_string();
            let current_time_minutes = now_in_tz.hour() as i32 * 60 + now_in_tz.minute() as i32;

            sqlx::query(
                "DELETE FROM task_tags WHERE task_id IN (
                    SELECT id FROM tasks 
                    WHERE series_id = ?1 
                    AND status = 'todo'
                    AND (
                        scheduled_date > ?2 
                        OR (scheduled_date = ?2 AND scheduled_time_minutes IS NOT NULL AND scheduled_time_minutes >= ?3)
                        OR (scheduled_date = ?2 AND scheduled_time_minutes IS NULL)
                    )
                )"
            )
            .bind(&id)
            .bind(&today_str)
            .bind(current_time_minutes)
            .execute(&mut *tx)
            .await?;

            sqlx::query(
                "DELETE FROM tasks 
                 WHERE series_id = ?1 
                 AND status = 'todo'
                 AND (
                     scheduled_date > ?2 
                     OR (scheduled_date = ?2 AND scheduled_time_minutes IS NOT NULL AND scheduled_time_minutes >= ?3)
                     OR (scheduled_date = ?2 AND scheduled_time_minutes IS NULL)
                 )"
            )
            .bind(&id)
            .bind(&today_str)
            .bind(current_time_minutes)
            .execute(&mut *tx)
            .await?;

            // Generate new future occurrences using the updated template
            let dtstart_time_minutes = new_dtstart_time_minutes;
            let (start_naive, has_time) = {
                let date = chrono::NaiveDate::parse_from_str(&new_dtstart_date, "%Y-%m-%d")?;
                match dtstart_time_minutes {
                    Some(m) => {
                        let h = m / 60;
                        let min = m % 60;
                        let t = chrono::NaiveTime::from_hms_opt(h as u32, min as u32, 0)
                            .ok_or_else(|| anyhow::anyhow!("Invalid time"))?;
                        (date.and_time(t), true)
                    }
                    None => {
                        let t = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();
                        (date.and_time(t), false)
                    }
                }
            };

            if let chrono::LocalResult::Single(start_in_tz) = tz.from_local_datetime(&start_naive) {
                // Build an RRuleSet by composing a DTSTART (with TZID/DATE semantics) and the RRULE line.
                let y = start_in_tz.year();
                let mo = start_in_tz.month();
                let d = start_in_tz.day();
                let dtstart_line = if has_time {
                    let h = start_in_tz.hour();
                    let mi = start_in_tz.minute();
                    let s = start_in_tz.second();
                    format!(
                        "DTSTART;TZID={}:{}{:02}{:02}T{:02}{:02}{:02}",
                        input.timezone, y, mo, d, h, mi, s
                    )
                } else {
                    format!(
                        "DTSTART;VALUE=DATE;TZID={}:{}{:02}{:02}",
                        input.timezone, y, mo, d
                    )
                };
                let rrule_line = format!("RRULE:{}", normalized_rrule);
                let set_str = format!("{}\n{}", dtstart_line, rrule_line);
                let set: RRuleSet = set_str.parse().map_err(|_| {
                    async_graphql::Error::new("Invalid RRULE/DTSTART combination")
                        .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"))
                })?;

                let mut created = 0usize;
                for occ in set.into_iter() {
                    // occ is expected to be in the correct local timezone per TZID in DTSTART
                    let occ_dt = occ.with_timezone(&tz);
                    if occ_dt < now_in_tz {
                        continue;
                    }

                    let task_id = uuid::Uuid::new_v4().to_string();
                    let scheduled_date = occ_dt.date_naive().format("%Y-%m-%d").to_string();
                    let scheduled_time_minutes = dtstart_time_minutes;

                    let deadline_dt =
                        occ_dt + chrono::Duration::minutes(new_deadline_offset_minutes as i64);
                    let deadline_date =
                        Some(deadline_dt.date_naive().format("%Y-%m-%d").to_string());
                    let deadline_time_minutes = if has_time {
                        Some((deadline_dt.hour() as i32) * 60 + (deadline_dt.minute() as i32))
                    } else {
                        None
                    };

                    sqlx::query("INSERT INTO tasks (id, project_id, author_id, assignee_id, series_id, title, description, status, scheduled_date, scheduled_time_minutes, deadline_date, deadline_time_minutes) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'todo', ?8, ?9, ?10, ?11)")
                        .bind(&task_id)
                        .bind(&existing_series.1) // project_id
                        .bind(&user_id)
                        .bind(&new_assignee_id)
                        .bind(&id) // series_id
                        .bind(&new_title)
                        .bind(&new_description)
                        .bind(&scheduled_date)
                        .bind(&scheduled_time_minutes)
                        .bind(&deadline_date)
                        .bind(&deadline_time_minutes)
                        .execute(&mut *tx)
                        .await?;

                    for tag_id in &final_default_tag_ids {
                        sqlx::query(
                            "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
                        )
                        .bind(&task_id)
                        .bind(tag_id)
                        .execute(&mut *tx)
                        .await?;
                    }

                    created += 1;
                    if created >= 5 {
                        break;
                    }
                }
            }
        } else {
            // If no regeneration needed, but core template fields changed, update existing todo occurrences
            let title_changed = input.title.is_some();
            let description_changed = input.description.is_some();
            let assignee_changed = input.assignee_id.is_some();
            let tags_changed = input.default_tag_ids.is_some();
            let template_changed =
                title_changed || description_changed || assignee_changed || tags_changed;

            if template_changed {
                // Update core content (title, description, assignee) of all todo occurrences
                sqlx::query(
                    "UPDATE tasks 
                     SET title = ?1, description = ?2, assignee_id = ?3, updated_at = (strftime('%Y-%m-%d %H:%M:%f','now'))
                     WHERE series_id = ?4 AND status = 'todo'"
                )
                .bind(&new_title)
                .bind(&new_description)
                .bind(&new_assignee_id)
                .bind(&id)
                .execute(&mut *tx)
                .await?;

                // Update tags if changed
                if tags_changed {
                    // Get all todo task IDs for this series
                    let todo_task_ids: Vec<String> = sqlx::query_as::<_, (String,)>(
                        "SELECT id FROM tasks WHERE series_id = ?1 AND status = 'todo'",
                    )
                    .bind(&id)
                    .fetch_all(&mut *tx)
                    .await?
                    .into_iter()
                    .map(|row| row.0)
                    .collect();

                    // Update tags for each todo task
                    for task_id in todo_task_ids {
                        // Delete existing tags
                        sqlx::query("DELETE FROM task_tags WHERE task_id = ?1")
                            .bind(&task_id)
                            .execute(&mut *tx)
                            .await?;

                        // Insert new tags
                        for tag_id in &final_default_tag_ids {
                            sqlx::query(
                                "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
                            )
                            .bind(&task_id)
                            .bind(tag_id)
                            .execute(&mut *tx)
                            .await?;
                        }
                    }
                }
            }
        }

        // Commit the transaction
        tx.commit().await?;

        // Fetch the updated series
        let updated_series = sqlx::query_as::<_, (String, String, String, String, Option<String>, Option<String>, String, String, Option<i32>, i32, String, String)>(
            "SELECT id, project_id, created_by, title, description, assignee_id, rrule, dtstart_date, dtstart_time_minutes, deadline_offset_minutes, created_at, updated_at 
             FROM recurring_series WHERE id = ?1"
        )
        .bind(&id)
        .fetch_one(pool)
        .await?;

        Ok(RecurringSeries {
            id: updated_series.0,
            project_id: updated_series.1,
            created_by: updated_series.2,
            title: updated_series.3,
            description: updated_series.4,
            assignee_id: updated_series.5,
            rrule: updated_series.6,
            dtstart_date: updated_series.7,
            dtstart_time_minutes: updated_series.8,
            deadline_offset_minutes: updated_series.9,
            created_at: updated_series.10,
            updated_at: updated_series.11,
            default_tag_ids: final_default_tag_ids,
        })
    }
}
