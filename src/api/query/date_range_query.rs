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
        Err(format!("Invalid date format: '{}'. Expected 'YYYY-MM-DD' or 'YYYY-MM-DD HH:MM:SS'", date_str))
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

        Err(format!("Invalid date format: '{}'. Expected 'YYYY-MM-DD' or 'YYYY-MM-DD HH:MM:SS'", &self.end_date))
    }
}
