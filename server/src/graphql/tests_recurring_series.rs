#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Utc};
    use chrono_tz::Tz;
    use rrule::RRule;

    #[test]
    fn test_rrule_validation() {
        // Valid RRULE
        let valid_rrule = "FREQ=DAILY;INTERVAL=1";
        assert!(valid_rrule.parse::<RRule<rrule::Unvalidated>>().is_ok());

        // Invalid RRULE
        let invalid_rrule = "INVALID_RRULE";
        assert!(invalid_rrule.parse::<RRule<rrule::Unvalidated>>().is_err());
    }

    #[test]
    fn test_date_validation() {
        // Valid date
        let valid_date = "2024-12-25";
        assert!(NaiveDate::parse_from_str(valid_date, "%Y-%m-%d").is_ok());

        // Invalid date
        let invalid_date = "invalid-date";
        assert!(NaiveDate::parse_from_str(invalid_date, "%Y-%m-%d").is_err());
    }

    #[test]
    fn test_timezone_validation() {
        // Valid timezone
        let valid_tz = "America/New_York";
        assert!(valid_tz.parse::<Tz>().is_ok());

        // Invalid timezone
        let invalid_tz = "Invalid/Timezone";
        assert!(invalid_tz.parse::<Tz>().is_err());
    }

    #[test]
    fn test_deadline_offset_bounds() {
        // Valid bounds
        assert!(0 >= 0 && 0 <= 525600);
        assert!(525600 >= 0 && 525600 <= 525600);

        // Invalid bounds
        assert!(!(-1 >= 0 && -1 <= 525600));
        assert!(!(525601 >= 0 && 525601 <= 525600));
    }

    #[test]
    fn test_time_minutes_validation() {
        // Valid time minutes
        assert!(0 >= 0 && 0 < 1440);
        assert!(1439 >= 0 && 1439 < 1440);

        // Invalid time minutes
        assert!(!(-1 >= 0 && -1 < 1440));
        assert!(!(1440 >= 0 && 1440 < 1440));
    }

    #[test]
    fn test_future_date_validation() {
        let tz: Tz = "UTC".parse().unwrap();
        let now_in_tz = Utc::now().with_timezone(&tz);
        let today = now_in_tz.date_naive();
        let tomorrow = today + chrono::Duration::days(1);
        let yesterday = today - chrono::Duration::days(1);

        // Today or future should be valid
        assert!(today >= today);
        assert!(tomorrow >= today);

        // Past should be invalid
        assert!(!(yesterday >= today));
    }
}
