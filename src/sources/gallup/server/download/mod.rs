use crate::sources::Scope;
use std::{collections::HashSet, error::Error};

mod fetch;
mod parse;
mod utils;

type DynError = Box<dyn Error + Send + Sync>;

pub(crate) async fn download_gallup_articles(
    scope: Scope,
) -> Result<Vec<crate::sources::gallup::server::GallupArticleAsset>, DynError> {
    let client = reqwest::Client::builder().build()?;
    let cutoff = scope.cutoff_date()?;
    let entry_limit = scope.entry_limit();
    tracing::info!(
        source = "gallup",
        scope = %scope,
        cutoff = cutoff.map(|date| date.format_iso()),
        entry_limit,
        "downloading Gallup articles"
    );

    let mut seen_articles = HashSet::new();
    let mut articles = Vec::new();
    let mut stale_pages = 0usize;
    let mut pdf_failures = 0usize;
    let mut chart_failures = 0usize;
    let mut skipped_articles = 0usize;

    for page_number in 1..=utils::MAX_SEARCH_PAGES {
        let search_html =
            fetch::fetch_page(&client, &utils::gallup_search_page_url(page_number)).await?;
        let stubs = parse::parse_search_stubs(&search_html)?;

        if stubs.is_empty() {
            break;
        }

        let mut page_has_scoped_article = false;

        for stub in stubs.into_iter().filter(|stub| {
            cutoff
                .map(|cutoff| stub.published_on >= cutoff)
                .unwrap_or(true)
        }) {
            page_has_scoped_article = true;

            if !seen_articles.insert(stub.article_url.clone()) {
                continue;
            }

            let article_html = fetch::fetch_page(&client, &stub.article_url).await?;
            let assets = parse::parse_article_assets(&article_html)?;
            let pdf_bytes = match assets.pdf_url {
                Some(ref pdf_url) => {
                    match fetch::fetch_pdf_bytes(&client, pdf_url, &stub.article_url).await {
                        Ok(pdf_bytes) => Some(pdf_bytes),
                        Err(error) => {
                            pdf_failures += 1;
                            tracing::warn!(
                                source = "gallup",
                                article = %stub.article_url,
                                pdf_url = %pdf_url,
                                error = %error,
                                "failed to download Gallup PDF"
                            );
                            None
                        }
                    }
                }
                None => None,
            };

            let mut charts = Vec::new();
            for chart in assets.charts {
                let dataset_url = utils::datawrapper_dataset_url(&chart.chart_url);
                match fetch::fetch_bytes(&client, &dataset_url).await {
                    Ok(csv_bytes) => {
                        charts.push(crate::sources::gallup::server::GallupChartAsset {
                            title: chart.title,
                            csv_bytes,
                        });
                    }
                    Err(error) => {
                        chart_failures += 1;
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

            if pdf_bytes.is_none() && charts.is_empty() {
                skipped_articles += 1;
                tracing::debug!(
                    source = "gallup",
                    article = %stub.article_url,
                    "skipping Gallup article with no downloadable assets"
                );
                continue;
            }

            articles.push(crate::sources::gallup::server::GallupArticleAsset {
                title: stub.title,
                published_on: stub.published_on.format_iso(),
                pdf_bytes,
                charts,
            });

            if entry_limit
                .map(|limit| articles.len() >= limit)
                .unwrap_or(false)
            {
                break;
            }
        }

        if entry_limit
            .map(|limit| articles.len() >= limit)
            .unwrap_or(false)
        {
            break;
        }

        if page_has_scoped_article {
            stale_pages = 0;
        } else {
            stale_pages += 1;
            if stale_pages >= 2 {
                break;
            }
        }
    }

    articles.sort_by(|left, right| right.published_on.cmp(&left.published_on));

    tracing::info!(
        source = "gallup",
        scope = %scope,
        articles = articles.len(),
        pdf_failures,
        chart_failures,
        skipped_articles,
        "downloaded Gallup source assets"
    );

    Ok(articles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_url_includes_page_number() {
        let url = utils::gallup_search_page_url(3);
        assert!(url.contains("p=3"));
    }

    #[test]
    fn parses_poll_tiles_from_gallup_search_results() {
        let html = r#"
            <section class="cmstile tile-feature">
                <div class="meta clearfix"><time datetime="2026-04-22">Apr 22, 2026</time></div>
                <div class="copy">
                    <div class="tile-linktext"><h3><a href="/poll/708722/disapproval-congress-ties-record-high.aspx">Disapproval of Congress Ties Record High at 86%</a></h3></div>
                </div>
            </section>
            <section class="cmstile tile-feature">
                <div class="meta clearfix"><time datetime="2026-04-20">Apr 20, 2026</time></div>
                <div class="copy">
                    <div class="tile-linktext"><h3><a href="/opinion/gallup/708557/value-silver-tsunami-depends-intangible-assets.aspx">Not a poll</a></h3></div>
                </div>
            </section>
        "#;

        let stubs = parse::parse_search_stubs(html).expect("Gallup stubs should parse");

        assert_eq!(stubs.len(), 1);
        assert_eq!(
            stubs[0].article_url,
            "https://news.gallup.com/poll/708722/disapproval-congress-ties-record-high.aspx"
        );
    }

    #[test]
    fn parses_pdf_and_chart_assets_from_article_html() {
        let html = r#"
            <html>
                <body>
                    <a href="https://news.gallup.com/file/poll/708728/example.pdf">(PDF download)</a>
                    <iframe data-src="https://datawrapper.dwcdn.net/Ne1q8/8/" title="Congress trend"></iframe>
                </body>
            </html>
        "#;

        let assets = parse::parse_article_assets(html).expect("Gallup assets should parse");

        assert_eq!(
            assets.pdf_url.as_deref(),
            Some("https://news.gallup.com/file/poll/708728/example.pdf")
        );
        assert_eq!(assets.charts.len(), 1);
        assert_eq!(
            utils::datawrapper_dataset_url(&assets.charts[0].chart_url),
            "https://datawrapper.dwcdn.net/Ne1q8/8/dataset.csv"
        );
    }
}
