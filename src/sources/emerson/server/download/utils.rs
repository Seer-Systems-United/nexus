//! # Emerson download utilities
//!
//! Constants and helper functions for downloading Emerson poll data
//! from blog pages and Google Sheets.

/// Base URL for Emerson College Polling blog.
pub(crate) const EMERSON_BLOG_URL: &str = "https://emersoncollegepolling.com/blog/";

/// Fragment in Google Sheets URLs to identify spreadsheet links.
pub(crate) const GOOGLE_SHEETS_PATH_FRAGMENT: &str = "/spreadsheets/d/";

/// Maximum number of blog pages to scan for releases.
pub(crate) const MAX_BLOG_PAGES: usize = 36;

/// Create a "resource not found" error for a given resource name.
pub(crate) fn missing_resource_error(resource: &'static str) -> super::DynError {
    std::io::Error::new(std::io::ErrorKind::NotFound, resource).into()
}

/// Normalize whitespace in text: collapse multiple spaces into one.
pub(crate) fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Build the download URL for a Google Sheets workbook.
///
/// # Parameters
/// - `sheet_id`: The Google Sheets ID.
///
/// # Returns
/// - Direct export URL for xlsx format.
pub(crate) fn workbook_download_url(sheet_id: &str) -> String {
    format!("https://docs.google.com/spreadsheets/d/{sheet_id}/export?format=xlsx")
}

/// Build the Emerson blog page URL for a given page number.
///
/// Page 1 is the root blog URL; subsequent pages use `/page/N/` format.
pub fn emerson_blog_page_url(page_number: usize) -> String {
    if page_number <= 1 {
        EMERSON_BLOG_URL.to_string()
    } else {
        format!("{EMERSON_BLOG_URL}page/{page_number}/")
    }
}
