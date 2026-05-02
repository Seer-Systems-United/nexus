//! # Ipsos download models
//!
//! Defines data structures for Ipsos article stubs,
//! article details, and download outcomes.

use crate::sources::Scope;
use crate::sources::date::SimpleDate;
use crate::sources::ipsos::server::download::DynError;
use std::io::Error as IoError;

/// Metadata stub for an Ipsos poll article found in listings.
///
/// # Fields
/// - `title`: Poll title.
/// - `article_url`: URL of the article page.
/// - `published_on`: Publication date.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleStub {
    pub title: String,
    pub article_url: String,
    pub published_on: SimpleDate,
}

/// Details extracted from an Ipsos article page.
///
/// # Fields
/// - `title`: Poll title (may be empty).
/// - `pdf_url`: Direct URL to the PDF.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleDetails {
    pub title: String,
    pub pdf_url: String,
}

/// Tracks the outcome of downloading Ipsos polls.
#[derive(Default)]
pub(super) struct DownloadOutcome {
    pub(super) pdfs: Vec<super::super::IpsosPollPdf>,
    pub(super) candidates: usize,
    failures: usize,
    failure_samples: Vec<String>,
}

impl DownloadOutcome {
    /// Record a failed download with logging.
    pub(super) fn record_failure(&mut self, stub: &ArticleStub, error: DynError) {
        self.failures += 1;
        let failure = format!("{} [{}]: {error}", stub.title, stub.article_url);
        tracing::warn!(source = "ipsos", failure = %failure, "skipping Ipsos poll");
        if self.failure_samples.len() < 3 {
            self.failure_samples.push(failure);
        }
    }

    /// Finish the download and return collected PDFs.
    ///
    /// Logs a summary and returns an error if no PDFs were downloaded
    /// but failures occurred.
    pub(super) fn finish(self, scope: Scope) -> Result<Vec<super::super::IpsosPollPdf>, DynError> {
        tracing::info!(
            source = "ipsos",
            scope = %scope,
            candidates = self.candidates,
            failures = self.failures,
            pdfs = self.pdfs.len(),
            "downloaded Ipsos poll PDFs"
        );

        if self.candidates > 0 && self.pdfs.is_empty() && self.failures > 0 {
            let detail = self.failure_samples.join(" | ");
            return Err(IoError::other(format!(
                "failed to download any Ipsos poll PDFs from {} releases: {detail}",
                self.failures
            ))
            .into());
        }

        Ok(self.pdfs)
    }
}
