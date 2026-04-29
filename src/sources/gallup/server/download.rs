use crate::sources::{Scope, date::SimpleDate};
use reqwest::header::REFERER;
use scraper::{Html, Selector};
use std::{collections::HashSet, error::Error};

const GALLUP_BASE_URL: &str = "https://news.gallup.com";
const GALLUP_SEARCH_URL: &str =
    "https://news.gallup.com/Search/raw.aspx?s=date&topic=1&cn=ALL_GALLUP_HEADLINES";
const MAX_SEARCH_PAGES: usize = 36;

type DynError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArticleStub {
    title: String,
    article_url: String,
    published_on: SimpleDate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChartStub {
    title: String,
    chart_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArticleAssets {
    pdf_url: Option<String>,
    charts: Vec<ChartStub>,
}

fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn absolute_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("{GALLUP_BASE_URL}{url}")
    }
}

fn gallup_search_page_url(page_number: usize) -> String {
    format!("{GALLUP_SEARCH_URL}&p={page_number}")
}

fn datawrapper_dataset_url(chart_url: &str) -> String {
    let normalized = chart_url.trim_end_matches('/');
    format!("{normalized}/dataset.csv")
}

fn parse_search_stubs(html: &str) -> Result<Vec<ArticleStub>, DynError> {
    let document = Html::parse_fragment(html);
    let tile_selector =
        Selector::parse("section.cmstile.tile-feature").expect("valid tile selector");
    let link_selector = Selector::parse(".tile-linktext a[href]").expect("valid link selector");
    let time_selector = Selector::parse("time[datetime]").expect("valid time selector");

    let mut stubs = Vec::new();

    for tile in document.select(&tile_selector) {
        let link = tile.select(&link_selector).next();
        let time = tile.select(&time_selector).next();

        let title = link
            .as_ref()
            .map(|element| normalize_text(&element.text().collect::<String>()));
        let article_url = link
            .and_then(|element| element.value().attr("href"))
            .filter(|href| href.contains("/poll/"))
            .map(absolute_url);
        let published_on = time
            .and_then(|element| element.value().attr("datetime"))
            .and_then(SimpleDate::parse_iso);

        if let (Some(title), Some(article_url), Some(published_on)) =
            (title, article_url, published_on)
        {
            stubs.push(ArticleStub {
                title,
                article_url,
                published_on,
            });
        }
    }

    Ok(stubs)
}

fn parse_article_assets(html: &str) -> Result<ArticleAssets, DynError> {
    let document = Html::parse_document(html);
    let link_selector = Selector::parse("a[href]").expect("valid link selector");
    let iframe_selector =
        Selector::parse("iframe[data-src], iframe[src]").expect("valid iframe selector");

    let pdf_url = document
        .select(&link_selector)
        .filter_map(|element| element.value().attr("href"))
        .find(|href| href.contains("/file/poll/") && href.ends_with(".pdf"))
        .map(absolute_url);

    let charts = document
        .select(&iframe_selector)
        .filter_map(|iframe| {
            let chart_url = iframe
                .value()
                .attr("data-src")
                .or_else(|| iframe.value().attr("src"))?;

            if !chart_url.contains("datawrapper.dwcdn.net") {
                return None;
            }

            Some(ChartStub {
                title: iframe
                    .value()
                    .attr("title")
                    .map(normalize_text)
                    .filter(|title| !title.is_empty())
                    .unwrap_or_else(|| "Gallup chart".to_string()),
                chart_url: chart_url.to_string(),
            })
        })
        .collect::<Vec<_>>();

    Ok(ArticleAssets { pdf_url, charts })
}

async fn fetch_html(client: &reqwest::Client, url: &str) -> Result<String, DynError> {
    Ok(client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}

async fn fetch_bytes(client: &reqwest::Client, url: &str) -> Result<Vec<u8>, DynError> {
    Ok(client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?
        .to_vec())
}

async fn fetch_pdf_bytes(
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

pub(crate) async fn download_gallup_articles(
    scope: Scope,
) -> Result<Vec<super::GallupArticleAsset>, DynError> {
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

    for page_number in 1..=MAX_SEARCH_PAGES {
        let search_html = fetch_html(&client, &gallup_search_page_url(page_number)).await?;
        let stubs = parse_search_stubs(&search_html)?;

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

            let article_html = fetch_html(&client, &stub.article_url).await?;
            let assets = parse_article_assets(&article_html)?;
            let pdf_bytes = match assets.pdf_url {
                Some(ref pdf_url) => {
                    match fetch_pdf_bytes(&client, pdf_url, &stub.article_url).await {
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
                let dataset_url = datawrapper_dataset_url(&chart.chart_url);
                match fetch_bytes(&client, &dataset_url).await {
                    Ok(csv_bytes) => {
                        charts.push(super::GallupChartAsset {
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

            articles.push(super::GallupArticleAsset {
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
    use super::{datawrapper_dataset_url, parse_article_assets, parse_search_stubs};

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

        let stubs = parse_search_stubs(html).expect("Gallup stubs should parse");

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

        let assets = parse_article_assets(html).expect("Gallup assets should parse");

        assert_eq!(
            assets.pdf_url.as_deref(),
            Some("https://news.gallup.com/file/poll/708728/example.pdf")
        );
        assert_eq!(assets.charts.len(), 1);
        assert_eq!(
            datawrapper_dataset_url(&assets.charts[0].chart_url),
            "https://datawrapper.dwcdn.net/Ne1q8/8/dataset.csv"
        );
    }
}
