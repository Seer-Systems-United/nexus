//! # Gallup HTTP network helpers
//!
//! Fetches HTML pages and binary data from Gallup servers.
//! Handles referrals for PDF downloads.

use reqwest::header::REFERER;

/// Fetch an HTML page as a string.
///
/// # Parameters
/// - `client`: HTTP client to use.
/// - `url`: The URL to fetch.
///
/// # Returns
/// - `Ok(String)`: The response body as text.
///
/// # Errors
/// - Returns an error if the request fails or status is not success.
pub(crate) async fn fetch_page(
    client: &reqwest::Client,
    url: &str,
) -> Result<String, super::DynError> {
    Ok(client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}

/// Fetch binary data (e.g., CSV, PDF) as bytes.
///
/// # Parameters
/// - `client`: HTTP client to use.
/// - `url`: The URL to fetch.
///
/// # Returns
/// - `Ok(Vec<u8>)`: The response body as bytes.
pub(crate) async fn fetch_bytes(
    client: &reqwest::Client,
    url: &str,
) -> Result<Vec<u8>, super::DynError> {
    Ok(client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?
        .to_vec())
}

/// Fetch PDF bytes with a referer header pointing to the article URL.
///
/// Gallup requires a referer header for PDF downloads.
///
/// # Parameters
/// - `client`: HTTP client to use.
/// - `pdf_url`: The PDF URL to fetch.
/// - `article_url`: The article URL to use as referer.
pub(crate) async fn fetch_pdf_bytes(
    client: &reqwest::Client,
    pdf_url: &str,
    article_url: &str,
) -> Result<Vec<u8>, super::DynError> {
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
