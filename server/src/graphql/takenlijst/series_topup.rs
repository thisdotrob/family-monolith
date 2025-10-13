use async_graphql::ErrorExtensions;
use chrono::{Datelike, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use rrule::RRuleSet;
use sqlx::{Row, SqlitePool};

pub async fn top_up_series(
    pool: &SqlitePool,
    series_id: &str,
    timezone: &str,
) -> async_graphql::Result<()> {
    // Parse timezone
    let tz: Tz = timezone.parse().map_err(|_| {
        async_graphql::Error::new("Invalid timezone")
            .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"))
    })?;

    // Load series
    let row = sqlx::query(
        "SELECT project_id, created_by, title, description, assignee_id, rrule, dtstart_date, dtstart_time_minutes, deadline_offset_minutes FROM recurring_series WHERE id = ?1",
    )
    .bind(series_id)
    .fetch_one(pool)
    .await
    .map_err(|_| async_graphql::Error::new("Series not found").extend_with(|_, e| e.set("code", "NOT_FOUND")))?;

    let project_id: String = row.get("project_id");
    let created_by: String = row.get("created_by");
    let title: String = row.get("title");
    let description: Option<String> = row.get("description");
    let assignee_id: Option<String> = row.get("assignee_id");
    let rrule: String = row.get("rrule");
    let dtstart_date: String = row.get("dtstart_date");
    let dtstart_time_minutes: Option<i32> = row.get("dtstart_time_minutes");
    let deadline_offset_minutes: i32 = row.get("deadline_offset_minutes");

    // Load default tags
    let tag_rows = sqlx::query("SELECT tag_id FROM recurring_series_tags WHERE series_id = ?1")
        .bind(series_id)
        .fetch_all(pool)
        .await?;
    let default_tag_ids: Vec<String> = tag_rows
        .into_iter()
        .map(|r| r.get::<String, _>("tag_id"))
        .collect();

    // Build DTSTART in timezone
    let (start_naive, has_time) = {
        let date = chrono::NaiveDate::parse_from_str(&dtstart_date, "%Y-%m-%d").map_err(|_| {
            async_graphql::Error::new("Invalid series dtstart_date")
                .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"))
        })?;
        match dtstart_time_minutes {
            Some(m) => {
                let h = m / 60;
                let min = m % 60;
                let t =
                    chrono::NaiveTime::from_hms_opt(h as u32, min as u32, 0).ok_or_else(|| {
                        async_graphql::Error::new("Invalid series dtstart time")
                            .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"))
                    })?;
                (date.and_time(t), true)
            }
            None => {
                let t = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();
                (date.and_time(t), false)
            }
        }
    };

    let (dtstart_line, now_in_tz) =
        if let chrono::LocalResult::Single(start_in_tz) = tz.from_local_datetime(&start_naive) {
            let y = start_in_tz.year();
            let mo = start_in_tz.month();
            let d = start_in_tz.day();
            let line = if has_time {
                let h = start_in_tz.hour();
                let mi = start_in_tz.minute();
                let s = start_in_tz.second();
                format!(
                    "DTSTART;TZID={}:{}{:02}{:02}T{:02}{:02}{:02}",
                    timezone, y, mo, d, h, mi, s
                )
            } else {
                format!(
                    "DTSTART;VALUE=DATE;TZID={}:{}{:02}{:02}",
                    timezone, y, mo, d
                )
            };
            (line, Utc::now().with_timezone(&tz))
        } else {
            return Ok(()); // Cannot resolve DTSTART in TZ; nothing to do
        };

    // Count existing future TODO tasks and collect existing occurrence keys to avoid duplicates
    let existing_rows = sqlx::query(
        "SELECT scheduled_date, scheduled_time_minutes, status FROM tasks WHERE series_id = ?1",
    )
    .bind(series_id)
    .fetch_all(pool)
    .await?;

    use std::collections::HashSet;
    let mut existing_keys: HashSet<(String, Option<i32>)> = HashSet::new();
    let mut future_todo_count = 0usize;

    for r in &existing_rows {
        let date: Option<String> = r.get("scheduled_date");
        let time_mins: Option<i32> = r.get("scheduled_time_minutes");
        let status: String = r.get("status");
        if let Some(date_str) = date {
            existing_keys.insert((date_str.clone(), time_mins));

            // Determine if future relative to now_in_tz
            if let Ok(day) = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                let occ_dt = if let Some(m) = time_mins {
                    let h = m / 60;
                    let mi = m % 60;
                    let t = chrono::NaiveTime::from_hms_opt(h as u32, mi as u32, 0);
                    if let Some(t) = t {
                        tz.from_local_datetime(&day.and_time(t)).single()
                    } else {
                        None
                    }
                } else {
                    tz.from_local_datetime(&day.and_hms_opt(0, 0, 0).unwrap())
                        .single()
                };
                if let Some(occ_dt) = occ_dt {
                    if occ_dt >= now_in_tz && status == "todo" {
                        future_todo_count += 1;
                    }
                }
            }
        }
    }

    let target = 5usize;
    if future_todo_count >= target {
        return Ok(());
    }
    let needed = target - future_todo_count;

    // Build RRULE set from stored rrule and constructed DTSTART
    let rrule_line = format!("RRULE:{}", rrule);
    let set_str = format!("{}\n{}", dtstart_line, rrule_line);
    let set: RRuleSet = set_str.parse().map_err(|_| {
        async_graphql::Error::new("Invalid RRULE/DTSTART combination")
            .extend_with(|_, e| e.set("code", "VALIDATION_FAILED"))
    })?;

    let mut created = 0usize;
    for occ in set.into_iter() {
        let occ_dt = occ.with_timezone(&tz);
        if occ_dt < now_in_tz {
            continue;
        }

        let scheduled_date = occ_dt.date_naive().format("%Y-%m-%d").to_string();
        let scheduled_time_minutes = dtstart_time_minutes; // same as series start

        // Skip if an occurrence already exists (any status)
        if existing_keys.contains(&(scheduled_date.clone(), scheduled_time_minutes)) {
            continue;
        }

        let deadline_dt = occ_dt + chrono::Duration::minutes(deadline_offset_minutes as i64);
        let deadline_date = Some(deadline_dt.date_naive().format("%Y-%m-%d").to_string());
        let deadline_time_minutes = if has_time {
            Some((deadline_dt.hour() as i32) * 60 + (deadline_dt.minute() as i32))
        } else {
            None
        };

        let task_id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO tasks (id, project_id, author_id, assignee_id, series_id, title, description, status, scheduled_date, scheduled_time_minutes, deadline_date, deadline_time_minutes) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'todo', ?8, ?9, ?10, ?11)")
            .bind(&task_id)
            .bind(&project_id)
            .bind(&created_by)
            .bind(&assignee_id)
            .bind(series_id)
            .bind(&title)
            .bind(&description)
            .bind(&scheduled_date)
            .bind(&scheduled_time_minutes)
            .bind(&deadline_date)
            .bind(&deadline_time_minutes)
            .execute(pool)
            .await?;

        for tag_id in &default_tag_ids {
            sqlx::query("INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)")
                .bind(&task_id)
                .bind(tag_id)
                .execute(pool)
                .await?;
        }

        existing_keys.insert((scheduled_date, scheduled_time_minutes));
        created += 1;
        if created >= needed {
            break;
        }
    }

    Ok(())
}
