use crate::auth::Claims;
use crate::auth::guard::require_member;
use crate::graphql::types::{RecurringSeries, UpdateSeriesInput};
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
        last_known_updated_at: String,
    ) -> async_graphql::Result<RecurringSeries> {
        // Require authentication
        let claims = ctx
            .data_opt::<Arc<Claims>>()
            .ok_or_else(|| async_graphql::Error::new("Authentication required"))?;
        let pool = ctx.data::<SqlitePool>()?;

        // Load series and check membership
        let series_row = sqlx::query_as::<_, (String, String, String, Option<String>, Option<String>, String, String, Option<i32>, i32, String)>(
            "SELECT project_id, created_by, title, description, assignee_id, rrule, dtstart_date, dtstart_time_minutes, deadline_offset_minutes, updated_at FROM recurring_series WHERE id = ?1",
        )
        .bind(&id)
        .fetch_optional(pool)
        .await?;
        let Some((
            project_id,
            _created_by,
            mut title,
            mut description,
            mut assignee_id,
            mut rrule_str,
            mut dtstart_date,
            mut dtstart_time_minutes,
            mut deadline_offset_minutes,
            current_updated_at,
        )) = series_row
        else {
            return Err(async_graphql::Error::new("Series not found")
                .extend_with(|_, e| e.set("code", "NOT_FOUND")));
        };

        // Concurrency check
        if current_updated_at != last_known_updated_at {
            return Err(async_graphql::Error::new("Stale write")
                .extend_with(|_, e| e.set("code", "CONFLICT_STALE_WRITE")));
        }

        // Permission
        // Get user ID from claims and require membership in project
        let user_id = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE username = ?1")
            .bind(&claims.sub)
            .fetch_one(pool)
            .await?
            .0;
        require_member(pool, &user_id, &project_id).await?;

        // Apply updates with validation
        if let Some(t) = input.title {
            title = t;
        }
        if let Some(d) = input.description {
            description = Some(d);
        }
        if let Some(a) = input.assignee_id {
            assignee_id = a;
        }
        if let Some(v) = input.deadline_offset_minutes {
            if v < 0 || v > 525600 {
                return Err(async_graphql::Error::new(
                    "deadlineOffsetMinutes must be between 0 and 525600",
                )
                .extend_with(|_, e| e.set("code", "VALIDATION_FAILED")));
            }
            deadline_offset_minutes = v;
        }
        if let Some(rr) = input.rrule.clone() {
            // Validate RRULE
            if rr.parse::<RRule<rrule::Unvalidated>>().is_err() {
                return Err(async_graphql::Error::new("Invalid RRULE format")
                    .extend_with(|_, e| e.set("code", "VALIDATION_FAILED")));
            }
            rrule_str = rr;
        }
        if let Some(date) = input.dtstart_date.clone() {
            if NaiveDate::parse_from_str(&date, "%Y-%m-%d").is_err() {
                return Err(async_graphql::Error::new(
                    "Invalid dtstartDate format, expected YYYY-MM-DD",
                )
                .extend_with(|_, e| e.set("code", "VALIDATION_FAILED")));
            }
            dtstart_date = date;
        }
        if let Some(time_opt) = input.dtstart_time_minutes {
            match time_opt {
                Some(m) => {
                    if m < 0 || m >= 1440 {
                        return Err(async_graphql::Error::new(
                            "dtstartTimeMinutes must be between 0 and 1439",
                        )
                        .extend_with(|_, e| e.set("code", "VALIDATION_FAILED")));
                    }
                    // Validate parsable
                    let h = m / 60;
                    let mi = m % 60;
                    if NaiveTime::from_hms_opt(h as u32, mi as u32, 0).is_none() {
                        return Err(async_graphql::Error::new("Invalid dtstartTimeMinutes")
                            .extend_with(|_, e| e.set("code", "VALIDATION_FAILED")));
                    }
                    dtstart_time_minutes = Some(m);
                }
                None => {
                    dtstart_time_minutes = None;
                }
            }
        }

        // Normalize RRULE if time removed
        if input.dtstart_time_minutes == Some(None) {
            // strip time-based parts
            let parts: Vec<&str> = rrule_str.split(';').collect();
            let kept: Vec<&str> = parts
                .into_iter()
                .filter(|p| {
                    let up = p.to_ascii_uppercase();
                    !(up.starts_with("BYHOUR=")
                        || up.starts_with("BYMINUTE=")
                        || up.starts_with("BYSECOND="))
                })
                .collect();
            rrule_str = kept.join(";");
        }

        // Update default tags if provided (validate existence)
        if let Some(tag_ids) = input.default_tag_ids.clone() {
            for tag_id in &tag_ids {
                let exists = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tags WHERE id = ?1")
                    .bind(tag_id)
                    .fetch_one(pool)
                    .await?;
                if exists.0 == 0 {
                    return Err(async_graphql::Error::new("One or more tags not found")
                        .extend_with(|_, e| e.set("code", "NOT_FOUND")));
                }
            }
            // replace join table
            sqlx::query("DELETE FROM recurring_series_tags WHERE series_id = ?1")
                .bind(&id)
                .execute(pool)
                .await?;
            for tag_id in &tag_ids {
                sqlx::query("INSERT OR IGNORE INTO recurring_series_tags (series_id, tag_id) VALUES (?1, ?2)").bind(&id).bind(tag_id).execute(pool).await?;
            }
        }

        // Validate timezone
        let tz: Tz = input.timezone.parse().map_err(|_| {
            async_graphql::Error::new("Invalid timezone")
                .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"))
        })?;
        let now_in_tz = Utc::now().with_timezone(&tz);

        // Persist updates to series
        sqlx::query("UPDATE recurring_series SET title = ?1, description = ?2, assignee_id = ?3, rrule = ?4, dtstart_date = ?5, dtstart_time_minutes = ?6, deadline_offset_minutes = ?7 WHERE id = ?8")
            .bind(&title)
            .bind(&description)
            .bind(&assignee_id)
            .bind(&rrule_str)
            .bind(&dtstart_date)
            .bind(dtstart_time_minutes)
            .bind(deadline_offset_minutes)
            .bind(&id)
            .execute(pool)
            .await?;

        // Propagate core content to all todo tasks in the series
        sqlx::query("UPDATE tasks SET title = ?1, description = ?2, assignee_id = ?3 WHERE series_id = ?4 AND status = 'todo'")
            .bind(&title)
            .bind(&description)
            .bind(&assignee_id)
            .bind(&id)
            .execute(pool)
            .await?;

        // Regenerate schedule/deadline for future todo occurrences from now onward
        // Build rruleset with DTSTART respecting timezone and optional time
        let has_time = dtstart_time_minutes.is_some();
        let start_naive = {
            let date = chrono::NaiveDate::parse_from_str(&dtstart_date, "%Y-%m-%d")?;
            match dtstart_time_minutes {
                Some(m) => {
                    let h = m / 60;
                    let mi = m % 60;
                    let t = chrono::NaiveTime::from_hms_opt(h as u32, mi as u32, 0).ok_or_else(
                        || {
                            async_graphql::Error::new("Invalid time")
                                .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"))
                        },
                    )?;
                    date.and_time(t)
                }
                None => date.and_time(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            }
        };
        if let chrono::LocalResult::Single(start_in_tz) = tz.from_local_datetime(&start_naive) {
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
            let rrule_line = format!("RRULE:{}", rrule_str);
            let set_str = format!("{}\n{}", dtstart_line, rrule_line);
            let set: RRuleSet = set_str.parse().map_err(|_| {
                async_graphql::Error::new("Invalid RRULE/DTSTART combination")
                    .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"))
            })?;

            // Collect all todo tasks in series ordered by scheduled_date/time to map by occurrence date
            let mut tasks: Vec<(String, Option<String>, Option<i32>)> = sqlx::query_as(
                "SELECT id, scheduled_date, scheduled_time_minutes FROM tasks WHERE series_id = ?1 AND status = 'todo' ORDER BY scheduled_date ASC, scheduled_time_minutes ASC"
            )
            .bind(&id)
            .fetch_all(pool)
            .await?;

            // For each occurrence >= now, update matching future todo tasks in order
            let mut future_occ_dates: Vec<(String, Option<i32>, String, Option<i32>)> = Vec::new();
            for occ in set.into_iter() {
                let occ_dt = occ.with_timezone(&tz);
                if occ_dt < now_in_tz {
                    continue;
                }
                let scheduled_date = occ_dt.date_naive().format("%Y-%m-%d").to_string();
                let scheduled_time_minutes = dtstart_time_minutes;
                let deadline_dt =
                    occ_dt + chrono::Duration::minutes(deadline_offset_minutes as i64);
                let deadline_date = Some(deadline_dt.date_naive().format("%Y-%m-%d").to_string());
                let deadline_time_minutes = if has_time {
                    Some((deadline_dt.hour() as i32) * 60 + (deadline_dt.minute() as i32))
                } else {
                    None
                };
                future_occ_dates.push((
                    scheduled_date,
                    scheduled_time_minutes,
                    deadline_date.unwrap(),
                    deadline_time_minutes,
                ));
                if future_occ_dates.len() >= tasks.len() {
                    break;
                }
            }

            // Update tasks sequentially with regenerated schedule/deadline
            for (i, (task_id, _old_sched_date, _old_sched_time)) in tasks.drain(..).enumerate() {
                if i >= future_occ_dates.len() {
                    break;
                }
                let (sched_date, sched_time, deadline_date, deadline_time) = &future_occ_dates[i];
                sqlx::query("UPDATE tasks SET scheduled_date = ?1, scheduled_time_minutes = ?2, deadline_date = ?3, deadline_time_minutes = ?4 WHERE id = ?5")
                    .bind(sched_date)
                    .bind(*sched_time)
                    .bind(deadline_date)
                    .bind(*deadline_time)
                    .bind(&task_id)
                    .execute(pool)
                    .await?;
            }
        }

        // Fetch updated default tags
        let default_tag_ids: Vec<String> = sqlx::query_as::<_, (String,)>(
            "SELECT tag_id FROM recurring_series_tags WHERE series_id = ?1",
        )
        .bind(&id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|t| t.0)
        .collect();

        // Fetch the updated series
        let series = sqlx::query_as::<_, (String, String, String, String, Option<String>, Option<String>, String, String, Option<i32>, i32, String, String)>(
            "SELECT id, project_id, created_by, title, description, assignee_id, rrule, dtstart_date, dtstart_time_minutes, deadline_offset_minutes, created_at, updated_at FROM recurring_series WHERE id = ?1"
        )
        .bind(&id)
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
