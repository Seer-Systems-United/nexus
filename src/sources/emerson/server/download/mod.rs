use crate::sources::Scope;
use std::{collections::HashSet, error::Error};

mod fetch;
mod parse;
mod utils;

type DynError = Box<dyn Error + Send + Sync>;

pub(crate) async fn download_emerson_data(
    scope: Scope,
) -> Result<Vec<crate::sources::emerson::server::EmersonWorkbook>, DynError> {
    let client = reqwest::Client::new();
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

    for page_number in 1..=utils::MAX_BLOG_PAGES {
        let landing_page =
            fetch::fetch_page(&client, &utils::emerson_blog_page_url(page_number)).await?;
        let releases = parse::parse_release_stubs(&landing_page)?;

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

            match download_release_workbook(&client, &release).await {
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
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "failed to download any Emerson workbooks from {failed_count} recent releases: {detail}"
            ),
        )
        .into());
    }

    Ok(workbooks)
}

async fn download_release_workbook(
    client: &reqwest::Client,
    release: &parse::ReleaseStub,
) -> Result<crate::sources::emerson::server::EmersonWorkbook, DynError> {
    let article_page = fetch::fetch_page(client, &release.article_url).await?;
    let details = parse::parse_release_details(&article_page)?;
    let workbook_bytes =
        fetch::fetch_bytes(client, &utils::workbook_download_url(&details.sheet_id)).await?;

    Ok(crate::sources::emerson::server::EmersonWorkbook {
        title: details.title,
        date: release.date.clone(),
        bytes: workbook_bytes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sources::date::SimpleDate;

    #[test]
    fn blog_url_includes_page_number() {
        let url = utils::emerson_blog_page_url(2);
        assert!(url.contains("page/2"));
    }

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

        let releases = parse::parse_release_stubs(html).expect("releases should parse");

        assert_eq!(releases.len(), 3);
        assert_eq!(releases[0].article_url, "https://example.com/ca/");
        assert_eq!(releases[1].published_on, SimpleDate::new(2026, 4, 16));
        assert_eq!(releases[2].published_on, SimpleDate::new(2026, 4, 9));
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

        let release = parse::parse_release_details(html).expect("release details should parse");

        assert_eq!(release.title, "California 2026 Poll: Sample Title");
        assert_eq!(release.sheet_id, "abc123");
    }
}
