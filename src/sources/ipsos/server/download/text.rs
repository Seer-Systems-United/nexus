use crate::sources::date::{SimpleDate, parse_month_name};
use scraper::ElementRef;

const IPSOS_BASE_URL: &str = "https://www.ipsos.com";

pub(super) fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(super) fn text_contents(element: ElementRef<'_>) -> String {
    normalize_text(&element.text().collect::<String>())
}

fn clean_day_token(input: &str) -> Option<u8> {
    input
        .trim_matches(|ch: char| !ch.is_ascii_digit())
        .parse()
        .ok()
}

fn clean_year_token(input: &str) -> Option<i32> {
    input
        .trim_matches(|ch: char| !ch.is_ascii_digit())
        .parse()
        .ok()
}

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

pub(super) fn absolute_url(href: &str) -> Option<String> {
    if href.starts_with("https://") || href.starts_with("http://") {
        Some(href.to_string())
    } else if href.starts_with('/') {
        Some(format!("{IPSOS_BASE_URL}{href}"))
    } else {
        None
    }
}

pub(super) fn clean_url(href: &str) -> String {
    href.split(['#', '?']).next().unwrap_or(href).to_string()
}

pub(super) fn is_ipsos_article_url(url: &str) -> bool {
    url.starts_with("https://www.ipsos.com/en-us/")
        && !url.ends_with(".pdf")
        && !url.contains("/latest-us-opinion-polls")
        && !url.contains("/topic/")
        && !url.contains("/insights-hub")
}
