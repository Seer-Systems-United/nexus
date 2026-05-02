use super::{DynError, client};
use scraper::{Html, Selector};
use std::io::{Error as IoError, ErrorKind};

const ECONOMIST_LANDING_PAGE_URL: &str = "https://yougov.com/en-us/content/the-economist";
const ECONOMIST_ARTICLE_PREFIX: &str = "https://yougov.com/en-us/articles/";
const ECONOMIST_CROSSTABS_PDF_FRAGMENT: &str = "/documents/econTabReport_";

pub(super) async fn poll_pdf_urls(scope: crate::sources::Scope) -> Result<Vec<String>, DynError> {
    let limit = scope.entry_limit().unwrap_or(10).max(1);
    let article_urls = economist_article_urls().await?;
    let mut pdf_urls = Vec::new();

    for article_url in article_urls.into_iter().take(limit) {
        if let Ok(article_page) = client::fetch_html(&article_url).await {
            let article_doc = Html::parse_document(&article_page);
            if let Some(pdf_url) = first_matching_href(&article_doc, is_economist_crosstabs_pdf_url)
            {
                pdf_urls.push(clean_pdf_url(&pdf_url));
            }
        }
    }

    if pdf_urls.is_empty() {
        return Err(missing_resource_error(
            "no Economist crosstabs PDF links found",
        ));
    }

    Ok(pdf_urls)
}

async fn economist_article_urls() -> Result<Vec<String>, DynError> {
    let landing_page = client::fetch_html(ECONOMIST_LANDING_PAGE_URL).await?;
    let document = Html::parse_document(&landing_page);
    let selector = Selector::parse("a[href]").expect("valid anchor selector");

    Ok(document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .filter(|href| href.starts_with(ECONOMIST_ARTICLE_PREFIX))
        .map(str::to_owned)
        .collect())
}

fn first_matching_href(document: &Html, predicate: impl Fn(&str) -> bool) -> Option<String> {
    let selector = Selector::parse("a[href]").expect("valid anchor selector");

    document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .find(|href| predicate(href))
        .map(str::to_owned)
}

fn missing_resource_error(resource: &'static str) -> DynError {
    IoError::new(ErrorKind::NotFound, resource).into()
}

pub fn is_economist_crosstabs_pdf_url(href: &str) -> bool {
    let normalized = href.to_ascii_lowercase();
    normalized.contains(&ECONOMIST_CROSSTABS_PDF_FRAGMENT.to_ascii_lowercase())
        && normalized.contains(".pdf")
}

pub fn clean_pdf_url(href: &str) -> String {
    href.split(['#', '?']).next().unwrap_or(href).to_string()
}
