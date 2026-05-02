//! # Gallup HTML parsing module
//!
//! Parses Gallup search pages and article pages to extract
//! article stubs, chart stubs, and PDF URLs.

use super::utils::*;
use crate::sources::date::SimpleDate;
use scraper::{Html, Selector};

/// Metadata stub for a Gallup article found in search results.
///
/// # Fields
/// - `title`: Article title.
/// - `article_url`: URL of the article page.
/// - `published_on`: Publication date.
pub struct ArticleStub {
    pub title: String,
    pub article_url: String,
    pub published_on: SimpleDate,
}

/// Parse article stubs from a Gallup search results page.
///
/// # Parameters
/// - `html`: The HTML content of the search page.
///
/// # Returns
/// - `Ok(Vec<ArticleStub>)`: List of found article stubs.
pub fn parse_search_stubs(html: &str) -> Result<Vec<ArticleStub>, super::DynError> {
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

/// Chart stub for a Gallup datawrapper chart.
///
/// # Fields
/// - `title`: Chart title.
/// - `chart_url`: URL of the chart (datawrapper).
pub struct ChartStub {
    pub title: String,
    pub chart_url: String,
}

/// Assets found in a Gallup article page.
///
/// # Fields
/// - `pdf_url`: Optional PDF URL.
/// - `charts`: List of chart stubs.
pub struct ArticleAssets {
    pub pdf_url: Option<String>,
    pub charts: Vec<ChartStub>,
}

/// Parse an article page for PDF and chart assets.
///
/// # Parameters
/// - `html`: The HTML content of the article page.
///
/// # Returns
/// - `Ok(ArticleAssets)`: Found assets.
pub fn parse_article_assets(html: &str) -> Result<ArticleAssets, super::DynError> {
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
