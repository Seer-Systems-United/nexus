use super::utils::*;
use crate::sources::date::{SimpleDate, parse_month_name};
use scraper::{ElementRef, Html, Selector};

pub struct ReleaseStub {
    pub article_url: String,
    pub date: String,
    pub published_on: SimpleDate,
}

pub fn parse_release_stubs(html: &str) -> Result<Vec<ReleaseStub>, super::DynError> {
    let document = Html::parse_document(html);
    let card_selector = Selector::parse(".post-list .item-post").expect("valid post selector");
    let action_selector = Selector::parse(".action a[href]").expect("valid action selector");
    let date_selector = Selector::parse(".meta-info span").expect("valid date selector");
    let mut stubs = Vec::new();

    for card in document.select(&card_selector) {
        let article_url = card
            .select(&action_selector)
            .next()
            .and_then(|element| element.value().attr("href"))
            .map(str::to_owned);
        let date = card.select(&date_selector).last().map(text_contents);
        let published_on = date.as_deref().and_then(parse_emerson_blog_date);

        if let (Some(article_url), Some(date), Some(published_on)) =
            (article_url, date, published_on)
        {
            stubs.push(ReleaseStub {
                article_url,
                date,
                published_on,
            });
        }
    }

    Ok(stubs)
}

fn parse_emerson_blog_date(input: &str) -> Option<SimpleDate> {
    let normalized = normalize_text(input);
    let mut parts = normalized.split_whitespace();
    let day = parts
        .next()?
        .trim_end_matches(|ch: char| ch.is_ascii_alphabetic())
        .parse()
        .ok()?;
    let month = parse_month_name(parts.next()?)?;
    let year = parts.next()?.parse().ok()?;

    Some(SimpleDate::new(year, month, day))
}

pub struct ReleaseDetails {
    pub title: String,
    pub sheet_id: String,
}

pub fn parse_release_details(html: &str) -> Result<ReleaseDetails, super::DynError> {
    let document = Html::parse_document(html);
    let heading_selector = Selector::parse("h1").expect("valid heading selector");
    let link_selector = Selector::parse("a[href]").expect("valid anchor selector");

    let title = document
        .select(&heading_selector)
        .next()
        .map(text_contents)
        .filter(|text| !text.is_empty())
        .ok_or_else(|| missing_resource_error("Emerson release title not found"))?;

    let sheet_id = document
        .select(&link_selector)
        .filter_map(|element| element.value().attr("href"))
        .find_map(extract_google_sheet_id)
        .ok_or_else(|| missing_resource_error("Emerson Google Sheet link not found"))?;

    Ok(ReleaseDetails { title, sheet_id })
}

fn extract_google_sheet_id(url: &str) -> Option<String> {
    let start = url.find(GOOGLE_SHEETS_PATH_FRAGMENT)? + GOOGLE_SHEETS_PATH_FRAGMENT.len();
    let remainder = &url[start..];
    let end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
    let sheet_id = &remainder[..end];

    (!sheet_id.is_empty()).then(|| sheet_id.to_string())
}

fn text_contents(element: ElementRef<'_>) -> String {
    normalize_text(&element.text().collect::<String>())
}
