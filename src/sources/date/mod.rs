//! # Date handling utilities for polling sources
//!
//! Provides `SimpleDate` for lightweight date representation
//! and conversions to/from `SystemTime` and ISO 8601 strings.
//! Also includes month name parsing and calendar calculations.

mod calendar;
mod month;

pub use month::parse_month_name;

use chrono::Datelike;
use std::{
    error::Error,
    io::{Error as IoError, ErrorKind},
    time::{SystemTime, UNIX_EPOCH},
};

/// Boxed dynamic error type for source operations.
pub type DynError = Box<dyn Error + Send + Sync>;

/// A simple date type with year, month, and day fields.
///
/// # Fields
/// - `year`: The year (e.g., 2024).
/// - `month`: The month (1-12).
/// - `day`: The day of the month (1-31).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl SimpleDate {
    /// Create a new `SimpleDate` with the given year, month, and day.
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Parse an ISO 8601 date string (YYYY-MM-DD) into a `SimpleDate`.
    ///
    /// # Parameters
    /// - `input`: The date string to parse.
    ///
    /// # Returns
    /// - `Some(SimpleDate)` if parsing succeeds, `None` otherwise.
    pub fn parse_iso(input: &str) -> Option<Self> {
        let mut parts = input.trim().split('-');
        let year = parts.next()?.parse().ok()?;
        let month = parts.next()?.parse().ok()?;
        let day = parts.next()?.parse().ok()?;

        Some(Self::new(year, month, day))
    }

    /// Get today's date in UTC.
    ///
    /// # Returns
    /// - `Ok(SimpleDate)`: Today's date.
    ///
    /// # Errors
    /// - Returns an error if the system time cannot be converted.
    pub fn today_utc() -> Result<Self, DynError> {
        Self::from_system_time(SystemTime::now())
    }

    /// Convert a `SystemTime` into a `SimpleDate`.
    ///
    /// # Parameters
    /// - `time`: The system time to convert.
    ///
    /// # Returns
    /// - `Ok(SimpleDate)`: The converted date.
    ///
    /// # Errors
    /// - Returns an error if the time is before UNIX epoch.
    pub fn from_system_time(time: SystemTime) -> Result<Self, DynError> {
        let days_since_epoch = time.duration_since(UNIX_EPOCH)?.as_secs() / 86_400;
        Ok(calendar::civil_from_days(days_since_epoch as i64))
    }

    /// Calculate a date that is N days prior to this date.
    ///
    /// # Parameters
    /// - `days`: Number of days to go back.
    ///
    /// # Returns
    /// - `Ok(SimpleDate)`: The prior date.
    ///
    /// # Errors
    /// - Returns an error if the date calculation fails.
    pub fn days_prior(self, days: u32) -> Result<Self, DynError> {
        let date = chrono::NaiveDate::from_ymd_opt(self.year, self.month.into(), self.day.into())
            .ok_or_else(|| IoError::new(ErrorKind::InvalidData, "invalid simple date"))?;
        let prior = date - chrono::Duration::days(days.into());

        Ok(Self::new(
            prior.year(),
            prior.month() as u8,
            prior.day() as u8,
        ))
    }

    /// Calculate a date that is N months prior to this date.
    ///
    /// # Parameters
    /// - `months`: Number of months to go back.
    ///
    /// # Returns
    /// - `SimpleDate`: The prior date (day is clamped to max days in month).
    pub fn months_prior(self, months: u32) -> Self {
        let month_index = self.year * 12 + i32::from(self.month) - 1 - months as i32;
        let year = month_index.div_euclid(12);
        let month = month_index.rem_euclid(12) as u8 + 1;
        let max_day = calendar::days_in_month(year, month);

        Self::new(year, month, self.day.min(max_day))
    }

    /// Calculate a date that is N years prior to this date.
    ///
    /// # Parameters
    /// - `years`: Number of years to go back.
    ///
    /// # Returns
    /// - `SimpleDate`: The prior date (day is clamped to max days in month).
    pub fn years_prior(self, years: u32) -> Self {
        let year = self.year - years as i32;
        let max_day = calendar::days_in_month(year, self.month);

        Self::new(year, self.month, self.day.min(max_day))
    }

    /// Format this date as an ISO 8601 string (YYYY-MM-DD).
    pub fn format_iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
