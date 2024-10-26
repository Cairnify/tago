use chrono::{DateTime, Utc};
use chrono_humanize::{Accuracy, Tense};

pub fn print_elapsed_time(start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> String {
    let diff = end_date.signed_duration_since(start_date);

    let ht = chrono_humanize::HumanTime::from(diff);

    ht.to_text_en(Accuracy::Precise, Tense::Present)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};

    #[test]
    fn test_zero_elapsed_time() {
        let naive_date = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date, Utc);

        let elapsed_time = print_elapsed_time(&specific_datetime, &specific_datetime);
        assert_eq!(elapsed_time, "0 seconds");
    }

    #[test]
    fn test_one_second_elapsed() {
        let naive_date_start = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let start_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_start, Utc);

        let naive_date_end = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 1)
            .unwrap();
        let end_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_end, Utc);

        let elapsed_time = print_elapsed_time(&start_specific_datetime, &end_specific_datetime);
        assert_eq!(elapsed_time, "1 second");
    }

    #[test]
    fn test_multiple_seconds_elapsed() {
        let naive_date_start = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let start_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_start, Utc);

        let naive_date_end = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 10)
            .unwrap();
        let end_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_end, Utc);

        let elapsed_time = print_elapsed_time(&start_specific_datetime, &end_specific_datetime);
        assert_eq!(elapsed_time, "10 seconds");
    }

    #[test]
    fn test_one_minute_elapsed() {
        let naive_date_start = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let start_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_start, Utc);

        let naive_date_end = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 1, 0)
            .unwrap();
        let end_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_end, Utc);

        let elapsed_time = print_elapsed_time(&start_specific_datetime, &end_specific_datetime);
        assert_eq!(elapsed_time, "1 minute");
    }

    #[test]
    fn test_multiple_minutes_elapsed() {
        let naive_date_start = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let start_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_start, Utc);

        let naive_date_end = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 5, 0)
            .unwrap();
        let end_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_end, Utc);

        let elapsed_time = print_elapsed_time(&start_specific_datetime, &end_specific_datetime);
        assert_eq!(elapsed_time, "5 minutes");
    }

    #[test]
    fn test_one_hour_elapsed() {
        let naive_date_start = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let start_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_start, Utc);

        let naive_date_end = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(1, 0, 0)
            .unwrap();
        let end_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_end, Utc);

        let elapsed_time = print_elapsed_time(&start_specific_datetime, &end_specific_datetime);
        assert_eq!(elapsed_time, "1 hour");
    }

    #[test]
    fn test_one_day_elapsed() {
        let naive_date_start = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let start_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_start, Utc);

        let naive_date_end = NaiveDate::from_ymd_opt(2023, 10, 26)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let end_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_end, Utc);

        let elapsed_time = print_elapsed_time(&start_specific_datetime, &end_specific_datetime);
        assert_eq!(elapsed_time, "1 day");
    }

    #[test]
    fn test_one_month_elapsed() {
        let naive_date_start = NaiveDate::from_ymd_opt(2023, 11, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let start_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_start, Utc);

        let naive_date_end = NaiveDate::from_ymd_opt(2023, 10, 26)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let end_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_end, Utc);

        let elapsed_time = print_elapsed_time(&start_specific_datetime, &end_specific_datetime);
        assert_eq!(elapsed_time, "1 month");
    }

    #[test]
    fn test_one_year_elapsed() {
        let naive_date_start = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let start_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_start, Utc);

        let naive_date_end = NaiveDate::from_ymd_opt(2024, 10, 24)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let end_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_end, Utc);

        let elapsed_time = print_elapsed_time(&start_specific_datetime, &end_specific_datetime);
        assert_eq!(elapsed_time, "1 year");
    }

    #[test]
    fn test_multiple_time_units() {
        let naive_date_start = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let start_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_start, Utc);

        let naive_date_end = NaiveDate::from_ymd_opt(2024, 11, 26)
            .unwrap()
            .and_hms_opt(0, 0, 12)
            .unwrap();
        let end_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_end, Utc);

        let elapsed_time = print_elapsed_time(&start_specific_datetime, &end_specific_datetime);
        assert_eq!(elapsed_time, "1 year, 1 month, 3 days and 12 seconds");
    }

    #[test]
    fn test_edge_case_month_day() {
        let naive_date_start = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let start_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_start, Utc);

        let naive_date_end = NaiveDate::from_ymd_opt(2023, 11, 26)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let end_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_end, Utc);

        let elapsed_time = print_elapsed_time(&start_specific_datetime, &end_specific_datetime);
        assert_eq!(elapsed_time, "1 month and 2 days");
    }

    #[test]
    fn test_edge_case_hour_minute() {
        let naive_date_start = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let start_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_start, Utc);

        let naive_date_end = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(1, 30, 0)
            .unwrap();
        let end_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_end, Utc);

        let elapsed_time = print_elapsed_time(&start_specific_datetime, &end_specific_datetime);
        assert_eq!(elapsed_time, "1 hour and 30 minutes");
    }

    #[test]
    fn test_different_years() {
        let naive_date_start = NaiveDate::from_ymd_opt(2023, 10, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let start_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_start, Utc);

        let naive_date_end = NaiveDate::from_ymd_opt(2025, 10, 24)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let end_specific_datetime: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_date_end, Utc);

        let elapsed_time = print_elapsed_time(&start_specific_datetime, &end_specific_datetime);
        assert_eq!(elapsed_time, "2 years");
    }
}
