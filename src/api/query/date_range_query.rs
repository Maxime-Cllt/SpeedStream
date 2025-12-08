use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::Deserialize;

/// Query parameters for filtering speed data by date range
#[derive(Debug, Deserialize)]
pub struct DateRangeQuery {
    pub start_date: String, // Expected format: "YYYY-MM-DD HH:MM:SS" or "YYYY-MM-DD"
    pub end_date: String,   // Expected format: "YYYY-MM-DD HH:MM:SS" or "YYYY-MM-DD"
}

impl DateRangeQuery {
    /// Parse a date string that can be either "YYYY-MM-DD" or "YYYY-MM-DD HH:MM:SS"
    /// Returns a DateTime<Utc>
    pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>, String> {
        // Try parsing as full datetime first (YYYY-MM-DD HH:MM:SS)
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc));
        }

        // Try parsing as date only (YYYY-MM-DD) - defaults to 00:00:00
        if let Ok(naive_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            let naive_dt = naive_date.and_hms_opt(0, 0, 0).unwrap();
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc));
        }

        // If neither format works, return an error
        Err(format!(
            "Invalid date format: '{}'. Expected 'YYYY-MM-DD' or 'YYYY-MM-DD HH:MM:SS'",
            date_str
        ))
    }

    /// Parse the start_date field
    pub fn parse_start_date(&self) -> Result<DateTime<Utc>, String> {
        Self::parse_date(&self.start_date)
    }

    /// Parse the end_date field
    /// If only a date is provided (no time), defaults to 23:59:59 to include the entire day
    pub fn parse_end_date(&self) -> Result<DateTime<Utc>, String> {
        // Try parsing as full datetime first
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(&self.end_date, "%Y-%m-%d %H:%M:%S") {
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc));
        }

        // If only date, set to end of day (23:59:59)
        if let Ok(naive_date) = NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d") {
            let naive_dt = naive_date.and_hms_opt(23, 59, 59).unwrap();
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc));
        }

        Err(format!(
            "Invalid date format: '{}'. Expected 'YYYY-MM-DD' or 'YYYY-MM-DD HH:MM:SS'",
            &self.end_date
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        let dt1 = DateRangeQuery::parse_date("2024-01-01 12:30:45").unwrap();
        assert_eq!(dt1.to_rfc3339(), "2024-01-01T12:30:45+00:00");

        let dt2 = DateRangeQuery::parse_date("2024-01-01").unwrap();
        assert_eq!(dt2.to_rfc3339(), "2024-01-01T00:00:00+00:00");

        let err = DateRangeQuery::parse_date("2024/01/01").unwrap_err();
        assert!(err.contains("Invalid date format"));
    }

    #[test]
    fn test_parse_start_date() {
        let query = DateRangeQuery {
            start_date: "2024-01-01 10:00:00".to_string(),
            end_date: "2024-01-02".to_string(),
        };
        let start_dt = query.parse_start_date().unwrap();
        assert_eq!(start_dt.to_rfc3339(), "2024-01-01T10:00:00+00:00");
    }

    #[test]
    fn test_parse_end_date() {
        let query = DateRangeQuery {
            start_date: "2024-01-01".to_string(),
            end_date: "2024-01-02".to_string(),
        };
        let end_dt = query.parse_end_date().unwrap();
        assert_eq!(end_dt.to_rfc3339(), "2024-01-02T23:59:59+00:00");
    }
}
