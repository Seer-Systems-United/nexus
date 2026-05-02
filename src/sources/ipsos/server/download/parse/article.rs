//! # Ipsos article page parser
//!
//! Parses individual Ipsos article pages to extract poll details.

use crate::sources::ipsos::server::download::DynError;
use crate::sources::ipsos::server::download::models::ArticleDetails;
use crate::sources::ipsos::server::download::text::{absolute_url, clean_url, text_contents};
use scraper::{Html, Selector};
use std::io::{Error as IoError, ErrorKind};

pub fn parse_article_details(html: &str) -> Result<ArticleDetails, DynError> {
    let document = Html::parse_document(html);
    let heading_selector = Selector::parse("h1").expect("valid heading selector");
    let download_link_selector =
        Selector::parse(".block-download-center a[href]").expect("valid download selector");
    let fallback_link_selector = Selector::parse("a[href]").expect("valid link selector");

    let title = document
        .select(&heading_selector)
        .next()
        .map(text_contents)
        .filter(|title| !title.is_empty())
        .ok_or_else(|| missing_resource_error("Ipsos article title not found"))?;
    let pdf_url = download_pdf_url(&document, &download_link_selector, &fallback_link_selector)
        .ok_or_else(|| missing_resource_error("Ipsos Topline PDF not found"))?;

    Ok(ArticleDetails { title, pdf_url })
}

fn download_pdf_url(
    document: &Html,
    download_link_selector: &Selector,
    fallback_link_selector: &Selector,
) -> Option<String> {
    pdf_url_from_selector(document, download_link_selector)
        .or_else(|| pdf_url_from_selector(document, fallback_link_selector))
}

fn pdf_url_from_selector(document: &Html, selector: &Selector) -> Option<String> {
    document
        .select(selector)
        .filter_map(|element| element.value().attr("href"))
        .filter(|href| href.to_ascii_lowercase().contains(".pdf"))
        .filter_map(absolute_url)
        .map(|url| clean_url(&url))
        .next()
}

fn missing_resource_error(resource: &'static str) -> DynError {
    IoError::new(ErrorKind::NotFound, resource).into()
}
