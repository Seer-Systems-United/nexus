use super::parse::ReleaseStub;
use super::{DynError, network, parse_release_details, utils};

pub(super) async fn download_release_workbook(
    client: &reqwest::Client,
    release: &ReleaseStub,
) -> Result<crate::sources::emerson::server::EmersonWorkbook, DynError> {
    let article_page = network::fetch_page(client, &release.article_url).await?;
    let details = parse_release_details(&article_page)?;
    let workbook_bytes =
        network::fetch_bytes(client, &utils::workbook_download_url(&details.sheet_id)).await?;

    Ok(crate::sources::emerson::server::EmersonWorkbook {
        title: details.title,
        date: release.date.clone(),
        bytes: workbook_bytes,
    })
}
