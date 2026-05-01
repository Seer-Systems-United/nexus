use super::utils::*;
use crate::sources::date::SimpleDate;
use scraper::{Html, Selector};

pub(crate) struct ArticleStub {
    pub(crate) title: String,
    pub(crate) article_url: String,
    pub(crate) published_on: SimpleDate,
}

pub(crate) fn parse_search_stubs(html: &str) -> Result<Vec<ArticleStub>, super::DynError> {
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

pub(crate) struct ChartStub {
    pub(crate) title: String,
    pub(crate) chart_url: String,
}

pub(crate) struct ArticleAssets {
    pub(crate) pdf_url: Option<String>,
    pub(crate) charts: Vec<ChartStub>,
}

pub(crate) fn parse_article_assets(html: &str) -> Result<ArticleAssets, super::DynError> {
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
