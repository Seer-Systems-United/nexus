//! # Ipsos text parsing utilities
//!
//! Helper functions for parsing dates and normalizing text
//! from Ipsos article pages.

use crate::sources::date::{SimpleDate, parse_month_name};
use scraper::ElementRef;

/// Base URL for Ipsos website.
const IPSOS_BASE_URL: &str = "https://www.ipsos.com";

/// Normalize whitespace in text: collapse runs of whitespace into single spaces.
pub(super) fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Extract text content from an HTML element.
pub(super) fn text_contents(element: ElementRef<'_>) -> String {
    normalize_text(&element.text().collect::<String>())
}

/// Clean a day token by removing non-digit characters.
fn clean_day_token(input: &str) -> Option<u8> {
    input
        .trim_matches(|ch: char| !ch.is_ascii_digit())
        .parse()
        .ok()
}

/// Clean a year token by removing non-digit characters.
fn clean_year_token(input: &str) -> Option<i32> {
    input
        .trim_matches(|ch: char| !ch.is_ascii_digit())
        .parse()
        .ok()
}

/// Parse a date from text with format "Month Day Year".
///
/// # Parameters
/// - `input`: Text containing a date (e.g., "January 15 2024").
///
/// # Returns
/// - `Some(SimpleDate)` if a valid date is found.
/// - `None` otherwise.
pub fn parse_text_date(input: &str) -> Option<SimpleDate> {
    let tokens = input.split_whitespace().collect::<Vec<_>>();

    for window in tokens.windows(3) {
        let Some(month) = parse_month_name(window[0]) else {
            continue;
        };
        let Some(day) = clean_day_token(window[1]) else {
            continue;
        };
        let Some(year) = clean_year_token(window[2]) else {
            continue;
        };

        return Some(SimpleDate::new(year, month, day));
    }

    None
}

/// Convert a relative URL to absolute using Ipsos base URL.
pub(super) fn absolute_url(href: &str) -> Option<String> {
    if href.starts_with("https://") || href.starts_with("http://") {
        Some(href.to_string())
    } else if href.starts_with('/') {
        Some(format!("{IPSOS_BASE_URL}{href}"))
    } else {
        None
    }
}

/// Clean a URL by removing fragment and query string.
pub(super) fn clean_url(href: &str) -> String {
    href.split(['#', '?']).next().unwrap_or(href).to_string()
}

/// Check if a URL is an Ipsos article page (not PDF, topic, etc.).
pub(super) fn is_ipsos_article_url(url: &str) -> bool {
    url.starts_with("https://www.ipsos.com/en-us/")
        && !url.ends_with(".pdf")
        && !url.contains("/latest-us-opinion-polls")
        && !url.contains("/topic/")
        && !url.contains("/insights-hub")
}
