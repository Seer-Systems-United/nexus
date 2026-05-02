//! # Ipsos HTTP client helpers
//!
//! Fetches HTML pages and PDF bytes from Ipsos servers.
//! Handles referrals for PDF downloads.

use super::DynError;
use reqwest::header::REFERER;

/// Fetch an HTML page as a string.
///
/// # Parameters
/// - `client`: HTTP client to use.
/// - `url`: The URL to fetch.
///
/// # Returns
/// - `Ok(String)`: The response body as text.
pub(super) async fn fetch_html(client: &reqwest::Client, url: &str) -> Result<String, DynError> {
    Ok(client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}

/// Fetch PDF bytes with a referer header.
///
/// Ipsos requires a referer header for PDF downloads.
///
/// # Parameters
/// - `client`: HTTP client to use.
/// - `pdf_url`: The PDF URL to fetch.
/// - `article_url`: The article URL to use as referer.
pub(super) async fn fetch_pdf_bytes(
    client: &reqwest::Client,
    pdf_url: &str,
    article_url: &str,
) -> Result<Vec<u8>, DynError> {
    Ok(client
        .get(pdf_url)
        .header(REFERER, article_url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?
        .to_vec())
}
