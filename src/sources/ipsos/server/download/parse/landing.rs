use crate::sources::ipsos::server::download::DynError;
use crate::sources::ipsos::server::download::models::ArticleStub;
use crate::sources::ipsos::server::download::text::{
    absolute_url, clean_url, is_ipsos_article_url, parse_text_date, text_contents,
};
use scraper::{ElementRef, Html, Selector};

pub fn parse_landing_stubs(html: &str) -> Result<Vec<ArticleStub>, DynError> {
    let document = Html::parse_document(html);
    let headings = landing_headings(&document);
    let link_selector = Selector::parse("a[href]").expect("valid link selector");
    let mut stubs = Vec::new();

    for heading in headings {
        let title = text_contents(heading);
        if title.is_empty() {
            continue;
        }

        let Some((article_url, body_text)) = article_link_and_body(heading, &link_selector) else {
            continue;
        };
        let Some(published_on) = parse_text_date(&body_text) else {
            continue;
        };

        stubs.push(ArticleStub {
            title,
            article_url,
            published_on,
        });
    }

    Ok(stubs)
}

fn landing_headings(document: &Html) -> Vec<ElementRef<'_>> {
    let scoped = Selector::parse(".block-wysiwyg h2").expect("valid Ipsos heading selector");
    let fallback = Selector::parse("h2").expect("valid heading selector");
    let headings = document.select(&scoped).collect::<Vec<_>>();

    if headings.is_empty() {
        document.select(&fallback).collect::<Vec<_>>()
    } else {
        headings
    }
}

fn article_link_and_body(
    heading: ElementRef<'_>,
    link_selector: &Selector,
) -> Option<(String, String)> {
    let mut body_text = String::new();
    let mut article_url = None;

    for sibling in heading.next_siblings() {
        let Some(element) = ElementRef::wrap(sibling) else {
            continue;
        };
        if matches!(element.value().name(), "h2" | "h3") {
            break;
        }
        article_url = article_url.or_else(|| first_article_href(element, link_selector));
        let sibling_text = text_contents(element);
        if !sibling_text.is_empty() {
            body_text.push(' ');
            body_text.push_str(&sibling_text);
        }
    }

    article_url.map(|url| (url, body_text))
}

fn first_article_href(element: ElementRef<'_>, link_selector: &Selector) -> Option<String> {
    element
        .select(link_selector)
        .filter_map(|link| link.value().attr("href"))
        .filter_map(absolute_url)
        .map(|url| clean_url(&url))
        .find(|url| is_ipsos_article_url(url))
}
