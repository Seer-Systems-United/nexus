mod calendar;
mod month;

pub use month::parse_month_name;

use chrono::Datelike;
use std::{
    error::Error,
    io::{Error as IoError, ErrorKind},
    time::{SystemTime, UNIX_EPOCH},
};

pub type DynError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl SimpleDate {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn parse_iso(input: &str) -> Option<Self> {
        let mut parts = input.trim().split('-');
        let year = parts.next()?.parse().ok()?;
        let month = parts.next()?.parse().ok()?;
        let day = parts.next()?.parse().ok()?;

        Some(Self::new(year, month, day))
    }

    pub fn today_utc() -> Result<Self, DynError> {
        Self::from_system_time(SystemTime::now())
    }

    pub fn from_system_time(time: SystemTime) -> Result<Self, DynError> {
        let days_since_epoch = time.duration_since(UNIX_EPOCH)?.as_secs() / 86_400;
        Ok(calendar::civil_from_days(days_since_epoch as i64))
    }

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

    pub fn months_prior(self, months: u32) -> Self {
        let month_index = self.year * 12 + i32::from(self.month) - 1 - months as i32;
        let year = month_index.div_euclid(12);
        let month = month_index.rem_euclid(12) as u8 + 1;
        let max_day = calendar::days_in_month(year, month);

        Self::new(year, month, self.day.min(max_day))
    }

    pub fn years_prior(self, years: u32) -> Self {
        let year = self.year - years as i32;
        let max_day = calendar::days_in_month(year, self.month);

        Self::new(year, self.month, self.day.min(max_day))
    }

    pub fn format_iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
