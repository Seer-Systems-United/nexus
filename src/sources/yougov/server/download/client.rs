use super::DynError;

pub(super) async fn fetch_html(url: &str) -> Result<String, DynError> {
    Ok(reqwest::get(url).await?.error_for_status()?.text().await?)
}

pub(super) async fn fetch_bytes(url: &str) -> Result<Vec<u8>, DynError> {
    Ok(reqwest::get(url)
        .await?
        .error_for_status()?
        .bytes()
        .await?
        .to_vec())
}
