mod network;
mod parse;
mod release;
mod state;
mod utils;

pub use parse::{parse_release_details, parse_release_stubs};
pub use utils::emerson_blog_page_url;

use crate::sources::{Scope, date::SimpleDate};
use std::error::Error;

pub type DynError = Box<dyn Error + Send + Sync>;

pub(crate) async fn download_emerson_data(
    scope: Scope,
) -> Result<Vec<crate::sources::emerson::server::EmersonWorkbook>, DynError> {
    let client = reqwest::Client::new();
    let cutoff = scope.cutoff_date()?;
    let entry_limit = scope.entry_limit();
    let mut state = state::DownloadState::new(scope);

    tracing::info!(
        source = "emerson",
        scope = %scope,
        cutoff = cutoff.map(|date| date.format_iso()),
        entry_limit,
        "downloading Emerson releases"
    );

    for page_number in 1..=utils::MAX_BLOG_PAGES {
        let page_has_scoped_release =
            process_page(&client, page_number, cutoff, entry_limit, &mut state).await?;

        if state.limit_reached(entry_limit) || state.should_stop_after_page(page_has_scoped_release)
        {
            break;
        }
    }

    state.finish()
}

async fn process_page(
    client: &reqwest::Client,
    page_number: usize,
    cutoff: Option<SimpleDate>,
    entry_limit: Option<usize>,
    state: &mut state::DownloadState,
) -> Result<bool, DynError> {
    let landing_page = network::fetch_page(client, &emerson_blog_page_url(page_number)).await?;
    let releases = parse_release_stubs(&landing_page)?;
    let mut page_has_scoped_release = false;

    if releases.is_empty() {
        return Ok(false);
    }

    for release in releases.into_iter().filter(|release| {
        cutoff
            .map(|cutoff| release.published_on >= cutoff)
            .unwrap_or(true)
    }) {
        page_has_scoped_release = true;
        if !state.mark_seen(&release.article_url) {
            continue;
        }

        state.record_candidate();
        match release::download_release_workbook(client, &release).await {
            Ok(workbook) => state.push_workbook(workbook),
            Err(error) => state.record_failure(&release, error),
        }

        if state.limit_reached(entry_limit) {
            break;
        }
    }

    Ok(page_has_scoped_release)
}
