use scraper::{Html, Selector};
use std::{
    error::Error,
    io::{Error as IoError, ErrorKind},
};

const ECONOMIST_LANDING_PAGE_URL: &str = "https://yougov.com/en-us/content/the-economist";
const ECONOMIST_ARTICLE_PREFIX: &str = "https://yougov.com/en-us/articles/";
const ECONOMIST_CROSSTABS_PDF_FRAGMENT: &str = "/documents/econTabReport_";

fn first_matching_href(document: &Html, predicate: impl Fn(&str) -> bool) -> Option<String> {
    let selector = Selector::parse("a[href]").expect("valid anchor selector");

    document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .find(|href| predicate(href))
        .map(str::to_owned)
}

fn missing_resource_error(resource: &'static str) -> Box<dyn Error + Send + Sync> {
    IoError::new(ErrorKind::NotFound, resource).into()
}

fn is_economist_crosstabs_pdf_url(href: &str) -> bool {
    let normalized = href.to_ascii_lowercase();
    normalized.contains(&ECONOMIST_CROSSTABS_PDF_FRAGMENT.to_ascii_lowercase())
        && normalized.contains(".pdf")
}

fn clean_pdf_url(href: &str) -> String {
    href.split(['#', '?']).next().unwrap_or(href).to_string()
}

async fn fetch_html(url: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    Ok(reqwest::get(url).await?.error_for_status()?.text().await?)
}

async fn latest_economist_article_url() -> Result<String, Box<dyn Error + Send + Sync>> {
    let landing_page = fetch_html(ECONOMIST_LANDING_PAGE_URL).await?;
    let document = Html::parse_document(&landing_page);

    first_matching_href(&document, |href| href.starts_with(ECONOMIST_ARTICLE_PREFIX))
        .ok_or_else(|| missing_resource_error("latest Economist article link not found"))
}

async fn poll_pdf_urls(
    scope: crate::sources::Scope,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let limit = scope.entry_limit().unwrap_or(10).max(1);

    let article_urls: Vec<String> = {
        let landing_page = fetch_html(ECONOMIST_LANDING_PAGE_URL).await?;
        let document = Html::parse_document(&landing_page);
        let selector = Selector::parse("a[href]").expect("valid anchor selector");
        document
            .select(&selector)
            .filter_map(|element| element.value().attr("href"))
            .filter(|href| href.starts_with(ECONOMIST_ARTICLE_PREFIX))
            .map(str::to_owned)
            .collect()
    };

    let mut pdf_urls = Vec::new();

    for article_url in article_urls.into_iter().take(limit) {
        if let Ok(article_page) = fetch_html(&article_url).await {
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

pub async fn download_yougov_data(
    scope: crate::sources::Scope,
) -> Result<Vec<Vec<u8>>, Box<dyn Error + Send + Sync>> {
    tracing::info!(source = "yougov", "downloading YouGov poll PDFs");
    let pdf_urls = poll_pdf_urls(scope).await?;
    let mut all_pdf_bytes = Vec::new();

    for pdf_url in pdf_urls {
        match reqwest::get(&pdf_url).await?.error_for_status() {
            Ok(response) => {
                let pdf_bytes = response.bytes().await?;
                tracing::info!(
                    source = "yougov",
                    pdf_url = %pdf_url,
                    bytes = pdf_bytes.len(),
                    "downloaded YouGov poll PDF"
                );
                all_pdf_bytes.push(pdf_bytes.to_vec());
            }
            Err(err) => {
                tracing::warn!(source = "yougov", error = %err, url = %pdf_url, "failed to download PDF");
            }
        }
    }

    Ok(all_pdf_bytes)
}

#[cfg(test)]
mod tests {
    use super::{clean_pdf_url, is_economist_crosstabs_pdf_url};

    #[test]
    fn matches_yougov_crosstab_pdf_links_with_page_fragments() {
        let href =
            "https://d3nkl3psvxxpe9.cloudfront.net/documents/econTabReport_Ty7ikPd.pdf#page=44";

        assert!(is_economist_crosstabs_pdf_url(href));
        assert_eq!(
            clean_pdf_url(href),
            "https://d3nkl3psvxxpe9.cloudfront.net/documents/econTabReport_Ty7ikPd.pdf"
        );
    }
}
