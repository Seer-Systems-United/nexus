//! # Gallup download module
//!
//! Downloads Gallup articles and chart data from search pages.
//! Handles pagination, article parsing, and chart CSV extraction.

mod article;
mod network;
mod parse;
mod state;
mod utils;

pub use parse::{parse_article_assets, parse_search_stubs};
pub use utils::{datawrapper_dataset_url, gallup_search_page_url};

use crate::sources::{Scope, date::SimpleDate};
use std::error::Error;

/// Boxed dynamic error type for Gallup operations.
pub type DynError = Box<dyn Error + Send + Sync>;

/// Download Gallup articles for the given scope.
///
/// Iterates through search pages, filtering by cutoff date,
/// until the entry limit is reached or pages go stale.
///
/// # Parameters
/// - `scope`: The query scope defining date range or entry limit.
///
/// # Returns
/// - `Ok(Vec<GallupArticleAsset>)`: Downloaded articles with charts.
///
/// # Errors
/// - Returns an error if download fails.
pub(crate) async fn download_gallup_articles(
    scope: Scope,
) -> Result<Vec<crate::sources::gallup::server::GallupArticleAsset>, DynError> {
    let client = reqwest::Client::builder().build()?;
    let cutoff = scope.cutoff_date()?;
    let entry_limit = scope.entry_limit();
    let mut state = state::DownloadState::new(scope);

    tracing::info!(
        source = "gallup",
        scope = %scope,
        cutoff = cutoff.map(|date| date.format_iso()),
        entry_limit,
        "downloading Gallup articles"
    );

    for page_number in 1..=utils::MAX_SEARCH_PAGES {
        let page_has_scoped_article =
            process_page(&client, page_number, cutoff, entry_limit, &mut state).await?;

        if state.limit_reached(entry_limit) || state.should_stop_after_page(page_has_scoped_article)
        {
            break;
        }
    }

    Ok(state.finish())
}

/// Process a single search page for articles.
///
/// Fetches the search page HTML, parses article stubs, filters by cutoff date,
/// downloads new articles, and updates the download state.
///
/// # Parameters
/// - `client`: HTTP client for fetching pages.
/// - `page_number`: Current search page number (1-indexed).
/// - `cutoff`: Optional cutoff date; articles before this date are skipped.
/// - `entry_limit`: Optional limit on total articles to download.
/// - `state`: Mutable reference to download state for tracking progress.
///
/// # Returns
/// - `Ok(true)` if the page had at least one scoped (non-filtered) article.
/// - `Ok(false)` if the page had no scoped articles.
///
/// # Errors
/// - Returns an error if page fetch or parsing fails.
async fn process_page(
    client: &reqwest::Client,
    page_number: usize,
    cutoff: Option<SimpleDate>,
    entry_limit: Option<usize>,
    state: &mut state::DownloadState,
) -> Result<bool, DynError> {
    let search_html = network::fetch_page(client, &gallup_search_page_url(page_number)).await?;
    let stubs = parse_search_stubs(&search_html)?;
    let mut page_has_scoped_article = false;

    if stubs.is_empty() {
        return Ok(false);
    }

    for stub in stubs.into_iter().filter(|stub| {
        cutoff
            .map(|cutoff| stub.published_on >= cutoff)
            .unwrap_or(true)
    }) {
        page_has_scoped_article = true;
        if !state.mark_seen(&stub.article_url) {
            continue;
        }

        let downloaded = article::download_article(client, stub).await?;
        state.record_download(downloaded);

        if state.limit_reached(entry_limit) {
            break;
        }
    }

    Ok(page_has_scoped_article)
}
