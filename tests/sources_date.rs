//! # Sources date tests
//!
//! Tests for date parsing and SimpleDate utilities.

use nexus::sources::date::{SimpleDate, parse_month_name};
use std::time::{Duration, UNIX_EPOCH};

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
    assert_eq!(
        SimpleDate::from_system_time(UNIX_EPOCH).unwrap(),
        SimpleDate::new(1970, 1, 1)
    );
    let time = UNIX_EPOCH + Duration::from_secs(20_200 * 86_400);
    assert_eq!(
        SimpleDate::from_system_time(time).unwrap(),
        SimpleDate::new(2025, 4, 22)
    );
}

#[test]
fn parses_text_month_names() {
    assert_eq!(parse_month_name("Apr"), Some(4));
    assert_eq!(parse_month_name("September"), Some(9));
    assert_eq!(parse_month_name("Oct."), Some(10));
}
