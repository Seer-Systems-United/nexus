use crate::sources::{
    Scope,
    date::{SimpleDate, parse_month_name},
};
use reqwest::header::REFERER;
use scraper::{ElementRef, Html, Selector};
use std::{
    collections::HashSet,
    error::Error,
    io::{Error as IoError, ErrorKind},
};

const IPSOS_BASE_URL: &str = "https://www.ipsos.com";
const IPSOS_LATEST_POLLS_URL: &str = "https://www.ipsos.com/en-us/latest-us-opinion-polls";

type DynError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArticleStub {
    title: String,
    article_url: String,
    published_on: SimpleDate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArticleDetails {
    title: String,
    pdf_url: String,
}

fn missing_resource_error(resource: &'static str) -> DynError {
    IoError::new(ErrorKind::NotFound, resource).into()
}

fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn text_contents(element: ElementRef<'_>) -> String {
    normalize_text(&element.text().collect::<String>())
}

fn clean_day_token(input: &str) -> Option<u8> {
    input
        .trim_matches(|ch: char| !ch.is_ascii_digit())
        .parse()
        .ok()
}

fn clean_year_token(input: &str) -> Option<i32> {
    input
        .trim_matches(|ch: char| !ch.is_ascii_digit())
        .parse()
        .ok()
}

fn parse_text_date(input: &str) -> Option<SimpleDate> {
    let tokens = input.split_whitespace().collect::<Vec<_>>();

    for window in tokens.windows(3) {
        let Some(month) = parse_month_name(window[0]) else {
            continue;
        };
        let Some(day) = clean_day_token(window[1]) else {
            continue;
        };
        let Some(year) = clean_year_token(window[2]) else {
            continue;
        };

        return Some(SimpleDate::new(year, month, day));
    }

    None
}

fn absolute_url(href: &str) -> Option<String> {
    if href.starts_with("https://") || href.starts_with("http://") {
        Some(href.to_string())
    } else if href.starts_with('/') {
        Some(format!("{IPSOS_BASE_URL}{href}"))
    } else {
        None
    }
}

fn clean_url(href: &str) -> String {
    href.split(['#', '?']).next().unwrap_or(href).to_string()
}

fn is_ipsos_article_url(url: &str) -> bool {
    url.starts_with("https://www.ipsos.com/en-us/")
        && !url.ends_with(".pdf")
        && !url.contains("/latest-us-opinion-polls")
        && !url.contains("/topic/")
        && !url.contains("/insights-hub")
}

fn first_article_href(element: ElementRef<'_>, link_selector: &Selector) -> Option<String> {
    element
        .select(link_selector)
        .filter_map(|link| link.value().attr("href"))
        .filter_map(absolute_url)
        .map(|url| clean_url(&url))
        .find(|url| is_ipsos_article_url(url))
}

fn parse_landing_stubs(html: &str) -> Result<Vec<ArticleStub>, DynError> {
    let document = Html::parse_document(html);
    let scoped_heading_selector =
        Selector::parse(".block-wysiwyg h2").expect("valid Ipsos heading selector");
    let fallback_heading_selector = Selector::parse("h2").expect("valid heading selector");
    let link_selector = Selector::parse("a[href]").expect("valid link selector");
    let headings = document
        .select(&scoped_heading_selector)
        .collect::<Vec<_>>();
    let headings = if headings.is_empty() {
        document
            .select(&fallback_heading_selector)
            .collect::<Vec<_>>()
    } else {
        headings
    };
    let mut stubs = Vec::new();

    for heading in headings {
        let title = text_contents(heading);
        if title.is_empty() {
            continue;
        }

        let mut body_text = String::new();
        let mut article_url = None;

        for sibling in heading.next_siblings() {
            let Some(element) = ElementRef::wrap(sibling) else {
                continue;
            };
            let tag = element.value().name();
            if matches!(tag, "h2" | "h3") {
                break;
            }

            if article_url.is_none() {
                article_url = first_article_href(element, &link_selector);
            }

            let sibling_text = text_contents(element);
            if !sibling_text.is_empty() {
                body_text.push(' ');
                body_text.push_str(&sibling_text);
            }
        }

        let Some(article_url) = article_url else {
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

fn parse_article_details(html: &str) -> Result<ArticleDetails, DynError> {
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

    let mut pdf_url = document
        .select(&download_link_selector)
        .filter_map(|element| element.value().attr("href"))
        .filter(|href| href.to_ascii_lowercase().contains(".pdf"))
        .filter_map(absolute_url)
        .map(|url| clean_url(&url))
        .next();

    if pdf_url.is_none() {
        pdf_url = document
            .select(&fallback_link_selector)
            .filter_map(|element| element.value().attr("href"))
            .filter(|href| href.to_ascii_lowercase().contains(".pdf"))
            .filter_map(absolute_url)
            .map(|url| clean_url(&url))
            .next();
    }

    let pdf_url = pdf_url.ok_or_else(|| missing_resource_error("Ipsos Topline PDF not found"))?;

    Ok(ArticleDetails { title, pdf_url })
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

pub(crate) async fn download_ipsos_polls(
    scope: Scope,
) -> Result<Vec<super::IpsosPollPdf>, DynError> {
    let client = reqwest::Client::builder().build()?;
    let landing_page = fetch_html(&client, IPSOS_LATEST_POLLS_URL).await?;
    let stubs = parse_landing_stubs(&landing_page)?;
    let cutoff = scope.cutoff_date()?;
    let entry_limit = scope.entry_limit();
    let mut seen_articles = HashSet::new();
    let mut pdfs = Vec::new();
    let mut candidates = 0usize;
    let mut failures = 0usize;
    let mut failure_samples = Vec::new();

    tracing::info!(
        source = "ipsos",
        scope = %scope,
        cutoff = cutoff.map(|date| date.format_iso()),
        entry_limit,
        "downloading Ipsos polls"
    );

    for stub in stubs.into_iter().filter(|stub| {
        cutoff
            .map(|cutoff| stub.published_on >= cutoff)
            .unwrap_or(true)
    }) {
        if !seen_articles.insert(stub.article_url.clone()) {
            continue;
        }

        candidates += 1;

        let result = async {
            let article_page = fetch_html(&client, &stub.article_url).await?;
            let details = parse_article_details(&article_page)?;
            let bytes = fetch_pdf_bytes(&client, &details.pdf_url, &stub.article_url).await?;

            Ok::<_, DynError>(super::IpsosPollPdf {
                title: if details.title.is_empty() {
                    stub.title.clone()
                } else {
                    details.title
                },
                published_on: stub.published_on.format_iso(),
                article_url: stub.article_url.clone(),
                pdf_url: details.pdf_url,
                bytes,
            })
        }
        .await;

        match result {
            Ok(pdf) => pdfs.push(pdf),
            Err(error) => {
                failures += 1;
                let failure = format!("{} [{}]: {error}", stub.title, stub.article_url);
                tracing::warn!(
                    source = "ipsos",
                    failure = %failure,
                    "skipping Ipsos poll"
                );

                if failure_samples.len() < 3 {
                    failure_samples.push(failure);
                }
            }
        }

        if entry_limit
            .map(|limit| pdfs.len() >= limit)
            .unwrap_or(false)
        {
            break;
        }
    }

    tracing::info!(
        source = "ipsos",
        scope = %scope,
        candidates,
        failures,
        pdfs = pdfs.len(),
        "downloaded Ipsos poll PDFs"
    );

    if candidates > 0 && pdfs.is_empty() && failures > 0 {
        let detail = failure_samples.join(" | ");
        return Err(IoError::other(format!(
            "failed to download any Ipsos poll PDFs from {failures} releases: {detail}"
        ))
        .into());
    }

    Ok(pdfs)
}

#[cfg(test)]
mod tests {
    use crate::sources::date::SimpleDate;

    use super::{parse_article_details, parse_landing_stubs, parse_text_date};

    #[test]
    fn parses_landing_poll_headlines() {
        let html = r#"
            <section class="block-publications-content">
                <div class="block-wysiwyg">
                    <h3>April 2026:</h3>
                    <h2>Americans increasingly feel the economy is on the wrong track</h2>
                    <p><strong>Washington, D.C., April 28, 2026 - </strong>This
                       <a href="/en-us/americans-increasingly-feel-economy-wrong-track">latest Reuters/Ipsos poll</a>
                       finds the economy is on the wrong track.</p>
                    <h2>External-only item</h2>
                    <p><strong>Washington, D.C., April 27, 2026 - </strong>Read
                       <a href="https://example.com/report">the report</a>.</p>
                </div>
            </section>
        "#;

        let stubs = parse_landing_stubs(html).expect("stubs should parse");

        assert_eq!(stubs.len(), 1);
        assert_eq!(
            stubs[0].article_url,
            "https://www.ipsos.com/en-us/americans-increasingly-feel-economy-wrong-track"
        );
        assert_eq!(stubs[0].published_on, SimpleDate::new(2026, 4, 28));
    }

    #[test]
    fn parses_download_center_pdf_link() {
        let html = r#"
            <h1>Americans increasingly feel the economy is on the wrong track</h1>
            <section class="block-download-center">
                <a href="https://www.ipsos.com/sites/default/files/topline.pdf?download=1">
                    Download pdf
                </a>
            </section>
        "#;

        let details = parse_article_details(html).expect("details should parse");

        assert_eq!(
            details.title,
            "Americans increasingly feel the economy is on the wrong track"
        );
        assert_eq!(
            details.pdf_url,
            "https://www.ipsos.com/sites/default/files/topline.pdf"
        );
    }

    #[test]
    fn parses_ipsos_text_dates() {
        let date = parse_text_date("Washington, DC, September 27, 2025 - New polling")
            .expect("date should parse");

        assert_eq!(date, SimpleDate::new(2025, 9, 27));
    }
}
