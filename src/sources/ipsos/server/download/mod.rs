mod client;
mod models;
mod parse;
mod text;

pub use models::{ArticleDetails, ArticleStub};
pub use parse::{parse_article_details, parse_landing_stubs};
pub use text::parse_text_date;

use crate::sources::Scope;
use models::DownloadOutcome;
use std::collections::HashSet;
use std::error::Error;

const IPSOS_LATEST_POLLS_URL: &str = "https://www.ipsos.com/en-us/latest-us-opinion-polls";

pub type DynError = Box<dyn Error + Send + Sync>;

pub(crate) async fn download_ipsos_polls(
    scope: Scope,
) -> Result<Vec<super::IpsosPollPdf>, DynError> {
    let client = reqwest::Client::builder().build()?;
    let landing_page = client::fetch_html(&client, IPSOS_LATEST_POLLS_URL).await?;
    let stubs = parse_landing_stubs(&landing_page)?;
    let cutoff = scope.cutoff_date()?;
    let entry_limit = scope.entry_limit();
    let mut seen_articles = HashSet::new();
    let mut outcome = DownloadOutcome::default();

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

        outcome.candidates += 1;
        match download_one(&client, &stub).await {
            Ok(pdf) => outcome.pdfs.push(pdf),
            Err(error) => outcome.record_failure(&stub, error),
        }

        if entry_limit
            .map(|limit| outcome.pdfs.len() >= limit)
            .unwrap_or(false)
        {
            break;
        }
    }

    outcome.finish(scope)
}

async fn download_one(
    client: &reqwest::Client,
    stub: &ArticleStub,
) -> Result<super::IpsosPollPdf, DynError> {
    let article_page = client::fetch_html(client, &stub.article_url).await?;
    let details = parse_article_details(&article_page)?;
    let bytes = client::fetch_pdf_bytes(client, &details.pdf_url, &stub.article_url).await?;

    Ok(super::IpsosPollPdf {
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
