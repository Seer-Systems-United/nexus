//! # Emerson network fetcher
//!
//! Fetches web pages for Emerson release listings.

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
