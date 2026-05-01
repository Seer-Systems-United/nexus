use std::{
    error::Error,
    io::{Error as IoError, ErrorKind},
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::Datelike;

type DynError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SimpleDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl SimpleDate {
    pub(crate) fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub(crate) fn parse_iso(input: &str) -> Option<Self> {
        let mut parts = input.trim().split('-');
        let year = parts.next()?.parse().ok()?;
        let month = parts.next()?.parse().ok()?;
        let day = parts.next()?.parse().ok()?;

        Some(Self::new(year, month, day))
    }

    pub(crate) fn today_utc() -> Result<Self, DynError> {
        Self::from_system_time(SystemTime::now())
    }

    pub(crate) fn from_system_time(time: SystemTime) -> Result<Self, DynError> {
        let days_since_epoch = time.duration_since(UNIX_EPOCH)?.as_secs() / 86_400;
        Ok(civil_from_days(days_since_epoch as i64))
    }

    pub(crate) fn days_prior(self, days: u32) -> Result<Self, DynError> {
        let date = chrono::NaiveDate::from_ymd_opt(self.year, self.month.into(), self.day.into())
            .ok_or_else(|| IoError::new(ErrorKind::InvalidData, "invalid simple date"))?;
        let prior = date - chrono::Duration::days(days.into());

        Ok(Self::new(
            prior.year(),
            prior.month() as u8,
            prior.day() as u8,
        ))
    }

    pub(crate) fn months_prior(self, months: u32) -> Self {
        let month_index = self.year * 12 + i32::from(self.month) - 1 - months as i32;
        let year = month_index.div_euclid(12);
        let month = month_index.rem_euclid(12) as u8 + 1;
        let max_day = days_in_month(year, month);

        Self::new(year, month, self.day.min(max_day))
    }

    pub(crate) fn years_prior(self, years: u32) -> Self {
        let year = self.year - years as i32;
        let max_day = days_in_month(year, self.month);

        Self::new(year, self.month, self.day.min(max_day))
    }

    pub(crate) fn format_iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

pub(crate) fn parse_month_name(input: &str) -> Option<u8> {
    let normalized = input
        .trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '+')
        .to_ascii_lowercase();

    match normalized.as_str() {
        "jan" | "january" => Some(1),
        "feb" | "february" => Some(2),
        "mar" | "march" => Some(3),
        "apr" | "april" => Some(4),
        "may" => Some(5),
        "jun" | "june" => Some(6),
        "jul" | "july" => Some(7),
        "aug" | "august" => Some(8),
        "sep" | "sept" | "september" => Some(9),
        "oct" | "october" => Some(10),
        "nov" | "november" => Some(11),
        "dec" | "december" => Some(12),
        _ => None,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 31,
    }
}

fn civil_from_days(days: i64) -> SimpleDate {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = (yoe as i32) + (era as i32) * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u8;
    let month = (mp + if mp < 10 { 3 } else { -9 }) as u8;

    if month <= 2 {
        year += 1;
    }

    SimpleDate::new(year, month, day)
}

#[cfg(test)]
mod tests {
    use super::{SimpleDate, civil_from_days, parse_month_name};

    #[test]
    fn parses_iso_dates() {
        let date = SimpleDate::parse_iso("2026-04-22").expect("date should parse");

        assert_eq!(date.year, 2026);
        assert_eq!(date.month, 4);
        assert_eq!(date.day, 22);
    }

    #[test]
    fn computes_day_month_and_year_offsets() {
        assert_eq!(
            SimpleDate::new(2026, 3, 1).days_prior(1).unwrap(),
            SimpleDate::new(2026, 2, 28)
        );
        assert_eq!(
            SimpleDate::new(2026, 3, 31).months_prior(1),
            SimpleDate::new(2026, 2, 28)
        );
        assert_eq!(
            SimpleDate::new(2024, 2, 29).years_prior(1),
            SimpleDate::new(2023, 2, 28)
        );
    }

    #[test]
    fn converts_days_since_epoch_to_calendar_dates() {
        assert_eq!(civil_from_days(0), SimpleDate::new(1970, 1, 1));
        assert_eq!(civil_from_days(20_200), SimpleDate::new(2025, 4, 22));
    }

    #[test]
    fn parses_text_month_names() {
        assert_eq!(parse_month_name("Apr"), Some(4));
        assert_eq!(parse_month_name("September"), Some(9));
        assert_eq!(parse_month_name("Oct."), Some(10));
    }
}
