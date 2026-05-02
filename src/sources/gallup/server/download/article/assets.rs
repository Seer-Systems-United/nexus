use super::ArticleStub;
use crate::sources::gallup::server::download::{datawrapper_dataset_url, network};

pub(super) async fn fetch_pdf(
    client: &reqwest::Client,
    pdf_url: Option<&str>,
    stub: &ArticleStub,
) -> (Option<Vec<u8>>, bool) {
    let Some(pdf_url) = pdf_url else {
        return (None, false);
    };

    match network::fetch_pdf_bytes(client, pdf_url, &stub.article_url).await {
        Ok(pdf_bytes) => (Some(pdf_bytes), false),
        Err(error) => {
            tracing::warn!(
                source = "gallup",
                article = %stub.article_url,
                pdf_url = %pdf_url,
                error = %error,
                "failed to download Gallup PDF"
            );
            (None, true)
        }
    }
}

pub(super) async fn fetch_charts(
    client: &reqwest::Client,
    chart_stubs: Vec<crate::sources::gallup::server::download::parse::ChartStub>,
    stub: &ArticleStub,
) -> (Vec<crate::sources::gallup::server::GallupChartAsset>, usize) {
    let mut charts = Vec::new();
    let mut failures = 0usize;

    for chart in chart_stubs {
        let dataset_url = datawrapper_dataset_url(&chart.chart_url);
        match network::fetch_bytes(client, &dataset_url).await {
            Ok(csv_bytes) => charts.push(crate::sources::gallup::server::GallupChartAsset {
                title: chart.title,
                csv_bytes,
            }),
            Err(error) => {
                failures += 1;
                tracing::warn!(
                    source = "gallup",
                    article = %stub.article_url,
                    chart_url = %chart.chart_url,
                    dataset_url = %dataset_url,
                    error = %error,
                    "failed to download Gallup chart dataset"
                );
            }
        }
    }

    (charts, failures)
}
