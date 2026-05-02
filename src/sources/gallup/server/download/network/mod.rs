use reqwest::header::REFERER;

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
