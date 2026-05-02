use super::DynError;
use reqwest::header::REFERER;

pub(super) async fn fetch_html(client: &reqwest::Client, url: &str) -> Result<String, DynError> {
    Ok(client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}

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
