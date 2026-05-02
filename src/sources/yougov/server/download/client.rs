//! # YouGov HTTP client helpers
//!
//! Simple fetchers for HTML pages and binary data
//! from YouGov/The Economist website.

use super::DynError;

/// Fetch an HTML page as a string.
///
/// # Parameters
/// - `url`: The URL to fetch.
///
/// # Returns
/// - `Ok(String)`: The response body as text.
pub(super) async fn fetch_html(url: &str) -> Result<String, DynError> {
    Ok(reqwest::get(url).await?.error_for_status()?.text().await?)
}

/// Fetch binary data (PDF) as bytes.
///
/// # Parameters
/// - `url`: The URL to fetch.
///
/// # Returns
/// - `Ok(Vec<u8>)`: The response body as bytes.
pub(super) async fn fetch_bytes(url: &str) -> Result<Vec<u8>, DynError> {
    Ok(reqwest::get(url)
        .await?
        .error_for_status()?
        .bytes()
        .await?
        .to_vec())
}
