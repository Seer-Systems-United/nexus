mod article;
mod network;
mod parse;
mod state;
mod utils;

pub use parse::{parse_article_assets, parse_search_stubs};
pub use utils::{datawrapper_dataset_url, gallup_search_page_url};

use crate::sources::{Scope, date::SimpleDate};
use std::error::Error;

pub type DynError = Box<dyn Error + Send + Sync>;

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
