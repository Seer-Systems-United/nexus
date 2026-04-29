use crate::sources::{
    Scope,
    date::{SimpleDate, parse_month_name},
};
use scraper::{ElementRef, Html, Selector};
use std::{
    collections::HashSet,
    error::Error,
    io::{Error as IoError, ErrorKind},
};

const EMERSON_BLOG_URL: &str = "https://emersoncollegepolling.com/blog/";
const GOOGLE_SHEETS_PATH_FRAGMENT: &str = "/spreadsheets/d/";
const MAX_BLOG_PAGES: usize = 36;

type DynError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReleaseStub {
    article_url: String,
    date: String,
    published_on: SimpleDate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReleaseDetails {
    title: String,
    sheet_id: String,
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

fn parse_release_stubs(html: &str) -> Result<Vec<ReleaseStub>, DynError> {
    let document = Html::parse_document(html);
    let card_selector = Selector::parse(".post-list .item-post").expect("valid post selector");
    let action_selector = Selector::parse(".action a[href]").expect("valid action selector");
    let date_selector = Selector::parse(".meta-info span").expect("valid date selector");

    let mut releases = Vec::new();

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
            releases.push(ReleaseStub {
                article_url,
                date,
                published_on,
            });
        }
    }

    Ok(releases)
}

fn extract_google_sheet_id(url: &str) -> Option<String> {
    let start = url.find(GOOGLE_SHEETS_PATH_FRAGMENT)? + GOOGLE_SHEETS_PATH_FRAGMENT.len();
    let remainder = &url[start..];
    let end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
    let sheet_id = &remainder[..end];

    (!sheet_id.is_empty()).then(|| sheet_id.to_string())
}

fn parse_release_details(html: &str) -> Result<ReleaseDetails, DynError> {
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

fn workbook_download_url(sheet_id: &str) -> String {
    format!("https://docs.google.com/spreadsheets/d/{sheet_id}/export?format=xlsx")
}

fn emerson_blog_page_url(page_number: usize) -> String {
    if page_number <= 1 {
        EMERSON_BLOG_URL.to_string()
    } else {
        format!("{EMERSON_BLOG_URL}page/{page_number}/")
    }
}

async fn fetch_html(url: &str) -> Result<String, DynError> {
    Ok(reqwest::get(url).await?.error_for_status()?.text().await?)
}

async fn fetch_bytes(url: &str) -> Result<Vec<u8>, DynError> {
    Ok(reqwest::get(url)
        .await?
        .error_for_status()?
        .bytes()
        .await?
        .to_vec())
}

async fn download_release_workbook(
    release: &ReleaseStub,
) -> Result<super::EmersonWorkbook, DynError> {
    let article_page = fetch_html(&release.article_url).await?;
    let details = parse_release_details(&article_page)?;
    let workbook_bytes = fetch_bytes(&workbook_download_url(&details.sheet_id)).await?;

    Ok(super::EmersonWorkbook {
        title: details.title,
        date: release.date.clone(),
        bytes: workbook_bytes,
    })
}

pub(crate) async fn download_emerson_workbooks(
    scope: Scope,
) -> Result<Vec<super::EmersonWorkbook>, DynError> {
    let cutoff = scope.cutoff_date()?;
    let entry_limit = scope.entry_limit();
    tracing::info!(
        source = "emerson",
        scope = %scope,
        cutoff = cutoff.map(|date| date.format_iso()),
        entry_limit,
        "downloading Emerson releases"
    );
    let mut seen_articles = HashSet::new();
    let mut workbooks = Vec::new();
    let mut stale_pages = 0usize;
    let mut candidate_count = 0usize;
    let mut failed_count = 0usize;
    let mut failure_samples = Vec::new();

    for page_number in 1..=MAX_BLOG_PAGES {
        let landing_page = fetch_html(&emerson_blog_page_url(page_number)).await?;
        let releases = parse_release_stubs(&landing_page)?;

        if releases.is_empty() {
            break;
        }

        let mut page_has_scoped_release = false;

        for release in releases.into_iter().filter(|release| {
            cutoff
                .map(|cutoff| release.published_on >= cutoff)
                .unwrap_or(true)
        }) {
            page_has_scoped_release = true;

            if !seen_articles.insert(release.article_url.clone()) {
                continue;
            }

            candidate_count += 1;

            match download_release_workbook(&release).await {
                Ok(workbook) => workbooks.push(workbook),
                Err(error) => {
                    failed_count += 1;

                    let failure = format!("{} [{}]: {error}", release.date, release.article_url);
                    tracing::warn!(
                        source = "emerson",
                        failure = %failure,
                        "skipping Emerson release"
                    );

                    if failure_samples.len() < 3 {
                        failure_samples.push(failure);
                    }
                }
            }

            if entry_limit
                .map(|limit| workbooks.len() >= limit)
                .unwrap_or(false)
            {
                break;
            }
        }

        if entry_limit
            .map(|limit| workbooks.len() >= limit)
            .unwrap_or(false)
        {
            break;
        }

        if page_has_scoped_release {
            stale_pages = 0;
        } else {
            stale_pages += 1;
            if stale_pages >= 2 {
                break;
            }
        }
    }

    tracing::info!(
        source = "emerson",
        scope = %scope,
        candidates = candidate_count,
        failures = failed_count,
        workbooks = workbooks.len(),
        "downloaded Emerson workbooks"
    );

    if candidate_count > 0 && workbooks.is_empty() && failed_count > 0 {
        let detail = failure_samples.join(" | ");
        return Err(IoError::new(
            ErrorKind::Other,
            format!(
                "failed to download any Emerson workbooks from {failed_count} recent releases: {detail}"
            ),
        )
        .into());
    }

    Ok(workbooks)
}

#[cfg(test)]
mod tests {
    use crate::sources::date::SimpleDate;

    use super::{
        extract_google_sheet_id, parse_emerson_blog_date, parse_release_details,
        parse_release_stubs,
    };

    #[test]
    fn parses_release_stubs_from_blog_listing() {
        let html = r#"
            <div class="post-list">
                <div class="item-post">
                    <div class="action"><a href="https://example.com/ca/">Full Release &amp; Results</a></div>
                    <div class="meta-info"><span>Date: </span><span>16th Apr 2026</span></div>
                </div>
                <div class="item-post">
                    <div class="action"><a href="https://example.com/mi/">Full Release &amp; Results</a></div>
                    <div class="meta-info"><span>Date: </span><span>16th Apr 2026</span></div>
                </div>
                <div class="item-post">
                    <div class="action"><a href="https://example.com/nyc/">Full Release &amp; Results</a></div>
                    <div class="meta-info"><span>Date: </span><span>09th Apr 2026</span></div>
                </div>
            </div>
        "#;

        let releases = parse_release_stubs(html).expect("releases should parse");

        assert_eq!(releases.len(), 3);
        assert_eq!(releases[0].article_url, "https://example.com/ca/");
        assert_eq!(releases[1].published_on, SimpleDate::new(2026, 4, 16));
        assert_eq!(releases[2].published_on, SimpleDate::new(2026, 4, 9));
    }

    #[test]
    fn parses_ordinal_emerson_blog_dates() {
        let published_on =
            parse_emerson_blog_date("16th Apr 2026").expect("blog date should parse");

        assert_eq!(published_on, SimpleDate::new(2026, 4, 16));
    }

    #[test]
    fn extracts_release_title_and_google_sheet_id() {
        let html = r#"
            <html>
                <body>
                    <h1>California 2026 Poll: Sample Title</h1>
                    <a href="https://docs.google.com/spreadsheets/d/abc123/edit?gid=1982529522#gid=1982529522">FULL RESULTS</a>
                </body>
            </html>
        "#;

        let release = parse_release_details(html).expect("release details should parse");

        assert_eq!(release.title, "California 2026 Poll: Sample Title");
        assert_eq!(release.sheet_id, "abc123");
    }

    #[test]
    fn extracts_google_sheet_ids_from_edit_urls() {
        let sheet_id = extract_google_sheet_id(
            "https://docs.google.com/spreadsheets/d/1BNMMXfepjtJli40FxOUGMFij70wgoHSs/edit?gid=1982529522#gid=1982529522",
        )
        .expect("sheet id should parse");

        assert_eq!(sheet_id, "1BNMMXfepjtJli40FxOUGMFij70wgoHSs");
    }
}
