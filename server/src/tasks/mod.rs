use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone, Utc};
use chrono_tz::Tz;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum TaskStatus {
    Todo,
    Done,
    Abandoned,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum TaskBucket {
    Overdue,
    Today,
    Tomorrow,
    Upcoming,
    NoDate,
}

#[derive(SimpleObject, Debug)]
pub struct Task {
    pub id: String,
    #[graphql(name = "projectId")]
    pub project_id: String,
    #[graphql(name = "authorId")]
    pub author_id: String,
    #[graphql(name = "assigneeId")]
    pub assignee_id: Option<String>,
    #[graphql(name = "seriesId")]
    pub series_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    #[graphql(name = "scheduledDate")]
    pub scheduled_date: Option<String>,
    #[graphql(name = "scheduledTimeMinutes")]
    pub scheduled_time_minutes: Option<i32>,
    #[graphql(name = "deadlineDate")]
    pub deadline_date: Option<String>,
    #[graphql(name = "deadlineTimeMinutes")]
    pub deadline_time_minutes: Option<i32>,
    #[graphql(name = "completedAt")]
    pub completed_at: Option<String>,
    #[graphql(name = "completedBy")]
    pub completed_by: Option<String>,
    #[graphql(name = "abandonedAt")]
    pub abandoned_at: Option<String>,
    #[graphql(name = "abandonedBy")]
    pub abandoned_by: Option<String>,
    #[graphql(name = "createdAt")]
    pub created_at: String,
    #[graphql(name = "updatedAt")]
    pub updated_at: String,
    // Derived fields
    #[graphql(name = "isOverdue")]
    pub is_overdue: bool,
    pub bucket: TaskBucket,
}

pub mod time_utils {
    use super::*;

    /// Parse a timezone string and return the Tz
    pub fn parse_timezone(tz_str: &str) -> Result<Tz, String> {
        tz_str
            .parse::<Tz>()
            .map_err(|_| format!("Invalid timezone: {}", tz_str))
    }

    /// Get current date and datetime in the specified timezone
    pub fn now_in_timezone(tz: Tz) -> (NaiveDate, DateTime<Tz>) {
        let now_utc = Utc::now();
        let now_local = now_utc.with_timezone(&tz);
        (now_local.date_naive(), now_local)
    }

    /// Combine date and optional time minutes into a datetime in the specified timezone
    /// Returns None if the date is invalid
    pub fn combine_date_time(
        date_str: &str,
        time_minutes: Option<i32>,
        tz: Tz,
    ) -> Option<DateTime<Tz>> {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()?;

        let time = if let Some(minutes) = time_minutes {
            let hours = minutes / 60;
            let mins = minutes % 60;
            NaiveTime::from_hms_opt(hours as u32, mins as u32, 0)?
        } else {
            NaiveTime::from_hms_opt(0, 0, 0)?
        };

        let naive_dt = date.and_time(time);

        // Handle timezone conversion carefully
        match tz.from_local_datetime(&naive_dt) {
            chrono::LocalResult::Single(dt) => Some(dt),
            chrono::LocalResult::Ambiguous(dt1, _) => Some(dt1), // Take first during DST transition
            chrono::LocalResult::None => None,                   // Invalid time during DST gap
        }
    }

    /// Determine if a task is overdue based on scheduled/deadline dates and times
    pub fn is_task_overdue(
        scheduled_date: Option<&str>,
        scheduled_time_minutes: Option<i32>,
        deadline_date: Option<&str>,
        deadline_time_minutes: Option<i32>,
        tz: Tz,
    ) -> bool {
        let (today, now) = now_in_timezone(tz);

        // Use scheduled first, fall back to deadline
        let (check_date, check_time) = if scheduled_date.is_some() {
            (scheduled_date, scheduled_time_minutes)
        } else if deadline_date.is_some() {
            (deadline_date, deadline_time_minutes)
        } else {
            return false; // No date = not overdue
        };

        let Some(date_str) = check_date else {
            return false;
        };

        if let Some(task_dt) = combine_date_time(date_str, check_time, tz) {
            if check_time.is_some() {
                // Has specific time - compare with current datetime
                task_dt < now
            } else {
                // Date-only - compare with today's date
                task_dt.date_naive() < today
            }
        } else {
            false // Invalid date
        }
    }

    /// Determine the bucket for a task based on scheduled/deadline dates
    pub fn get_task_bucket(
        scheduled_date: Option<&str>,
        scheduled_time_minutes: Option<i32>,
        deadline_date: Option<&str>,
        deadline_time_minutes: Option<i32>,
        tz: Tz,
    ) -> TaskBucket {
        let (today, now) = now_in_timezone(tz);

        // Use scheduled first, fall back to deadline
        let (check_date, check_time) = if scheduled_date.is_some() {
            (scheduled_date, scheduled_time_minutes)
        } else if deadline_date.is_some() {
            (deadline_date, deadline_time_minutes)
        } else {
            return TaskBucket::NoDate;
        };

        let Some(date_str) = check_date else {
            return TaskBucket::NoDate;
        };

        let Ok(task_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") else {
            return TaskBucket::NoDate; // Invalid date
        };

        // For datetime tasks, also check if overdue by time
        if let Some(_time_minutes) = check_time {
            if let Some(task_dt) = combine_date_time(date_str, check_time, tz) {
                if task_dt < now {
                    return TaskBucket::Overdue;
                }
            }
        }

        // Date-based bucketing
        let tomorrow = today.succ_opt().unwrap_or(today);

        if task_date < today {
            TaskBucket::Overdue
        } else if task_date == today {
            TaskBucket::Today
        } else if task_date == tomorrow {
            TaskBucket::Tomorrow
        } else {
            TaskBucket::Upcoming
        }
    }

    /// Generate a sort key for a task following the spec ordering:
    /// Scheduled (date/time), then Deadline (date/time), then Created
    /// For NoDate bucket, we'll sort by title separately
    pub fn get_task_sort_key(
        scheduled_date: Option<&str>,
        scheduled_time_minutes: Option<i32>,
        deadline_date: Option<&str>,
        deadline_time_minutes: Option<i32>,
        created_at: &str,
        tz: Tz,
    ) -> (i64, i64, String) {
        let scheduled_timestamp = if let Some(date) = scheduled_date {
            combine_date_time(date, scheduled_time_minutes, tz)
                .map(|dt| dt.timestamp())
                .unwrap_or(i64::MAX)
        } else {
            i64::MAX
        };

        let deadline_timestamp = if let Some(date) = deadline_date {
            combine_date_time(date, deadline_time_minutes, tz)
                .map(|dt| dt.timestamp())
                .unwrap_or(i64::MAX)
        } else {
            i64::MAX
        };

        (
            scheduled_timestamp,
            deadline_timestamp,
            created_at.to_string(),
        )
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_timezone() {
            assert!(parse_timezone("UTC").is_ok());
            assert!(parse_timezone("America/New_York").is_ok());
            assert!(parse_timezone("Europe/London").is_ok());
            assert!(parse_timezone("invalid").is_err());
        }

        #[test]
        fn test_combine_date_time() {
            let tz = chrono_tz::UTC;

            // Valid date without time
            let dt = combine_date_time("2024-01-15", None, tz);
            assert!(dt.is_some());

            // Valid date with time
            let dt = combine_date_time("2024-01-15", Some(540), tz); // 9:00 AM
            assert!(dt.is_some());

            // Invalid date
            let dt = combine_date_time("2024-13-45", None, tz);
            assert!(dt.is_none());

            // Invalid time
            let dt = combine_date_time("2024-01-15", Some(1500), tz);
            assert!(dt.is_none());
        }

        #[test]
        fn test_is_task_overdue() {
            let tz = chrono_tz::UTC;

            // No dates - not overdue
            assert!(!is_task_overdue(None, None, None, None, tz));

            // Yesterday date-only - overdue
            let yesterday = chrono::Utc::now()
                .date_naive()
                .pred_opt()
                .unwrap()
                .format("%Y-%m-%d")
                .to_string();
            assert!(is_task_overdue(Some(&yesterday), None, None, None, tz));

            // Tomorrow date-only - not overdue
            let tomorrow = chrono::Utc::now()
                .date_naive()
                .succ_opt()
                .unwrap()
                .format("%Y-%m-%d")
                .to_string();
            assert!(!is_task_overdue(Some(&tomorrow), None, None, None, tz));
        }

        #[test]
        fn test_get_task_bucket() {
            let tz = chrono_tz::UTC;
            let today = chrono::Utc::now().date_naive();
            let yesterday = today.pred_opt().unwrap();
            let tomorrow = today.succ_opt().unwrap();

            // No dates
            assert_eq!(
                get_task_bucket(None, None, None, None, tz),
                TaskBucket::NoDate
            );

            // Yesterday
            let yesterday_str = yesterday.format("%Y-%m-%d").to_string();
            assert_eq!(
                get_task_bucket(Some(&yesterday_str), None, None, None, tz),
                TaskBucket::Overdue
            );

            // Today
            let today_str = today.format("%Y-%m-%d").to_string();
            assert_eq!(
                get_task_bucket(Some(&today_str), None, None, None, tz),
                TaskBucket::Today
            );

            // Tomorrow
            let tomorrow_str = tomorrow.format("%Y-%m-%d").to_string();
            assert_eq!(
                get_task_bucket(Some(&tomorrow_str), None, None, None, tz),
                TaskBucket::Tomorrow
            );

            // Future date
            let future = today.checked_add_days(chrono::Days::new(5)).unwrap();
            let future_str = future.format("%Y-%m-%d").to_string();
            assert_eq!(
                get_task_bucket(Some(&future_str), None, None, None, tz),
                TaskBucket::Upcoming
            );
        }
    }
}
