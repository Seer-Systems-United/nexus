mod assets;

use super::parse::ArticleStub;
use super::{DynError, network, parse_article_assets};
use assets::{fetch_charts, fetch_pdf};

pub(super) struct ArticleDownload {
    pub(super) article: Option<crate::sources::gallup::server::GallupArticleAsset>,
    pub(super) pdf_failed: bool,
    pub(super) chart_failures: usize,
    pub(super) skipped: bool,
}

pub(super) async fn download_article(
    client: &reqwest::Client,
    stub: ArticleStub,
) -> Result<ArticleDownload, DynError> {
    let article_html = network::fetch_page(client, &stub.article_url).await?;
    let assets = parse_article_assets(&article_html)?;
    let (pdf_bytes, pdf_failed) = fetch_pdf(client, assets.pdf_url.as_deref(), &stub).await;
    let (charts, chart_failures) = fetch_charts(client, assets.charts, &stub).await;

    if pdf_bytes.is_none() && charts.is_empty() {
        tracing::debug!(
            source = "gallup",
            article = %stub.article_url,
            "skipping Gallup article with no downloadable assets"
        );
        return Ok(ArticleDownload {
            article: None,
            pdf_failed,
            chart_failures,
            skipped: true,
        });
    }

    Ok(ArticleDownload {
        article: Some(crate::sources::gallup::server::GallupArticleAsset {
            title: stub.title,
            published_on: stub.published_on.format_iso(),
            pdf_bytes,
            charts,
        }),
        pdf_failed,
        chart_failures,
        skipped: false,
    })
}
