//! # Gallup download utilities
//!
//! Constants and helper functions for downloading Gallup poll data
//! from search pages and datawrapper charts.

/// Base URL for Gallup news site.
pub(crate) const GALLUP_BASE_URL: &str = "https://news.gallup.com";

/// URL for Gallup search with poll results.
pub(crate) const GALLUP_SEARCH_URL: &str =
    "https://news.gallup.com/Search/raw.aspx?s=date&topic=1&cn=ALL_GALLUP_HEADLINES";

/// Maximum number of search pages to scan.
pub(crate) const MAX_SEARCH_PAGES: usize = 36;

/// Normalize whitespace in text: collapse runs of whitespace into single spaces.
pub(crate) fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Convert a relative URL to an absolute URL using the Gallup base.
///
/// # Parameters
/// - `url`: The URL to make absolute.
///
/// # Returns
/// - Absolute URL string.
pub(crate) fn absolute_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("{GALLUP_BASE_URL}{url}")
    }
}

/// Build the Gallup search page URL for a given page number.
pub fn gallup_search_page_url(page_number: usize) -> String {
    format!("{GALLUP_SEARCH_URL}&p={page_number}")
}

/// Build the dataset CSV URL from a datawrapper chart URL.
///
/// # Parameters
/// - `chart_url`: The datawrapper chart URL.
///
/// # Returns
/// - URL to download the CSV dataset.
pub fn datawrapper_dataset_url(chart_url: &str) -> String {
    let normalized = chart_url.trim_end_matches('/');
    format!("{normalized}/dataset.csv")
}
