use tracing::{debug, info, instrument, warn};

use crate::{
    database::{BackendTrait, default_backend},
    poll::{
        question::Question,
        source::yougov::api::{
            get::{get_latest_editorial_document, get_latest_editorial_pdf_pages},
            parse::parse_pages,
        },
    },
    utils::pdf::extract::extract_pdf_from_url,
};
use chrono::{DateTime, Utc};

pub mod get;
pub mod models;
pub mod parse;

#[instrument(level = "info", skip_all)]
pub fn latest_survey() -> Vec<Question> {
    info!("fetching latest survey pages");
    let pages = get_latest_editorial_pdf_pages();
    debug!(
        pages_len = pages.len(),
        "fetched latest editorial pdf pages"
    );

    let questions = parse_pages(&pages);
    info!(
        questions_len = questions.len(),
        "parsed latest survey questions"
    );
    questions
}

#[instrument(level = "info", skip_all)]
pub fn latest_survey_with_timestamp() -> (Vec<Question>, DateTime<Utc>) {
    let Some(document) = get_latest_editorial_document() else {
        warn!("no latest editorial document found");
        return (Vec::new(), Utc::now());
    };

    info!(url = %document.url, created_at = %document.created_at, "fetched latest editorial document");

    let published_timestamp = DateTime::parse_from_rfc3339(&document.created_at)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .unwrap_or_else(|err| {
            warn!(error = %err, created_at = %document.created_at, "failed to parse document created_at; using current time");
            Utc::now()
        });

    let pdf_bytes = extract_pdf_from_url(&document.url);
    debug!(pdf_bytes_len = pdf_bytes.len(), "downloaded editorial pdf");

    let questions = parse_pages(&pdf_bytes);
    info!(questions_len = questions.len(), published_timestamp = %published_timestamp, "parsed latest survey questions with timestamp");

    (questions, published_timestamp)
}

#[instrument(level = "info", skip_all)]
pub fn has_new_poll() -> bool {
    let Some(doc) = get_latest_editorial_document() else {
        warn!("no latest editorial document found");
        return false;
    };

    // Parse time. Example layout "2026-05-12T16:35:47.365735Z"
    let latest = match DateTime::parse_from_rfc3339(&doc.created_at) {
        Ok(ts) => ts.with_timezone(&Utc),
        Err(err) => {
            warn!(error = %err, created_at = %doc.created_at, "failed to parse created_at");
            return false;
        }
    };

    debug!(created_at = %doc.created_at, parsed_latest = %latest, "checking for new poll by timestamp");

    match default_backend().and_then(|backend| backend.poll_exists_by_timestamp(latest)) {
        Ok(exists) => {
            info!(exists, "poll existence check complete");
            exists
        }
        Err(error) => {
            warn!(%error, "failed to check poll existence");
            false
        }
    }
}
