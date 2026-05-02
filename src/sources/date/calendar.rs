//! # Calendar utilities
//!
//! Provides low-level calendar functions for date calculations,
//! including leap year detection and day-of-month calculations
//! using the proleptic Gregorian calendar.

use super::SimpleDate;

/// Check if a given year is a leap year.
///
/// # Parameters
/// - `year`: The year to check.
///
/// # Returns
/// - `true` if the year is a leap year, `false` otherwise.
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Get the number of days in a given month and year.
///
/// # Parameters
/// - `year`: The year.
/// - `month`: The month (1-12).
///
/// # Returns
/// - Number of days in the month (28-31).
pub(super) fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 31,
    }
}

/// Convert a number of days since UNIX epoch into a `SimpleDate`.
///
/// Uses the algorithm from:
/// https://howardhinnant.github.io/date_algorithms.html#civil_from_days
///
/// # Parameters
/// - `days`: Number of days since UNIX epoch (1970-01-01).
///
/// # Returns
/// - `SimpleDate`: The corresponding calendar date.
pub(super) fn civil_from_days(days: i64) -> SimpleDate {
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
