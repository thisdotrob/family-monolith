use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::graphql::types::{CreateSeriesInput, RecurringSeries};
use async_graphql::{Context, ErrorExtensions, Object};
use chrono::{Datelike, NaiveDate, NaiveTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use rrule::{RRule, RRuleSet};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Default)]
pub struct CreateRecurringSeriesMutation;

#[Object]
impl CreateRecurringSeriesMutation {
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

        // Normalize RRULE for date-only series by stripping time-based parts (BYHOUR/BYMINUTE/BYSECOND)
        let has_time_input = input.dtstart_time_minutes.is_some();
        let normalized_rrule = if has_time_input {
            input.rrule.clone()
        } else {
            let parts: Vec<&str> = input.rrule.split(';').collect();
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
        .bind(&normalized_rrule)
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

        // Generate 5 future task occurrences linked to this series using RRULE iterator with chrono-tz adapter
        let dtstart_time_minutes = input.dtstart_time_minutes;
        let (start_naive, has_time) = {
            let date = chrono::NaiveDate::parse_from_str(&input.dtstart_date, "%Y-%m-%d")?;
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
                    occ_dt + chrono::Duration::minutes(input.deadline_offset_minutes as i64);
                let deadline_date = Some(deadline_dt.date_naive().format("%Y-%m-%d").to_string());
                let deadline_time_minutes = if has_time {
                    Some((deadline_dt.hour() as i32) * 60 + (deadline_dt.minute() as i32))
                } else {
                    None
                };

                sqlx::query("INSERT INTO tasks (id, project_id, author_id, assignee_id, series_id, title, description, status, scheduled_date, scheduled_time_minutes, deadline_date, deadline_time_minutes) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'todo', ?8, ?9, ?10, ?11)")
                    .bind(&task_id)
                    .bind(&input.project_id)
                    .bind(&user_id)
                    .bind(&input.assignee_id)
                    .bind(&series_id)
                    .bind(&input.title)
                    .bind(&input.description)
                    .bind(&scheduled_date)
                    .bind(&scheduled_time_minutes)
                    .bind(&deadline_date)
                    .bind(&deadline_time_minutes)
                    .execute(pool)
                    .await?;

                for tag_id in &default_tag_ids {
                    sqlx::query(
                        "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
                    )
                    .bind(&task_id)
                    .bind(tag_id)
                    .execute(pool)
                    .await?;
                }

                created += 1;
                if created >= 5 {
                    break;
                }
            }
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
}
