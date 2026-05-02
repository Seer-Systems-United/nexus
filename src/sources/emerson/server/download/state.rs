//! # Emerson download state tracking
//!
//! Tracks downloads in progress: seen articles, collected workbooks,
//! failure counts, and pagination decisions.

use super::DynError;
use crate::sources::Scope;
use std::collections::HashSet;

/// Tracks the state of an Emerson download operation.
pub(super) struct DownloadState {
    scope: Scope,
    seen_articles: HashSet<String>,
    workbooks: Vec<crate::sources::emerson::server::EmersonWorkbook>,
    stale_pages: usize,
    candidate_count: usize,
    failed_count: usize,
    failure_samples: Vec<String>,
}

impl DownloadState {
    /// Create a new `DownloadState` for the given scope.
    pub(super) fn new(scope: Scope) -> Self {
        Self {
            scope,
            seen_articles: HashSet::new(),
            workbooks: Vec::new(),
            stale_pages: 0,
            candidate_count: 0,
            failed_count: 0,
            failure_samples: Vec::new(),
        }
    }

    /// Mark an article URL as seen; returns true if it was new.
    pub(super) fn mark_seen(&mut self, article_url: &str) -> bool {
        self.seen_articles.insert(article_url.to_string())
    }

    /// Record that a candidate release was found.
    pub(super) fn record_candidate(&mut self) {
        self.candidate_count += 1;
    }

    /// Add a successfully downloaded workbook.
    pub(super) fn push_workbook(
        &mut self,
        workbook: crate::sources::emerson::server::EmersonWorkbook,
    ) {
        self.workbooks.push(workbook);
    }

    /// Record a failed download and log a warning.
    pub(super) fn record_failure(&mut self, release: &super::parse::ReleaseStub, error: DynError) {
        self.failed_count += 1;
        let failure = format!("{} [{}]: {error}", release.date, release.article_url);
        tracing::warn!(source = "emerson", failure = %failure, "skipping Emerson release");
        if self.failure_samples.len() < 3 {
            self.failure_samples.push(failure);
        }
    }

    /// Check if the entry limit has been reached.
    pub(super) fn limit_reached(&self, entry_limit: Option<usize>) -> bool {
        entry_limit
            .map(|limit| self.workbooks.len() >= limit)
            .unwrap_or(false)
    }

    /// Decide whether to stop paginating based on stale pages.
    ///
    /// If the current page has scoped releases, reset stale count.
    /// Otherwise, increment and stop after 2 consecutive stale pages.
    pub(super) fn should_stop_after_page(&mut self, page_has_scoped_release: bool) -> bool {
        if page_has_scoped_release {
            self.stale_pages = 0;
            return false;
        }

        self.stale_pages += 1;
        self.stale_pages >= 2
    }

    /// Finish the download and return collected workbooks.
    ///
    /// Logs a summary and returns an error if no workbooks were downloaded
    /// but failures occurred.
    pub(super) fn finish(
        self,
    ) -> Result<Vec<crate::sources::emerson::server::EmersonWorkbook>, DynError> {
        tracing::info!(
            source = "emerson",
            scope = %self.scope,
            candidates = self.candidate_count,
            failures = self.failed_count,
            workbooks = self.workbooks.len(),
            "downloaded Emerson workbooks"
        );

        if self.candidate_count >0 && self.workbooks.is_empty() && self.failed_count >0 {
            let detail = self.failure_samples.join(" | ");
            return Err(std::io::Error::other(format!(
                "failed to download any Emerson workbooks from {} recent releases: {detail}",
                self.failed_count
            ))
            .into());
        }

        Ok(self.workbooks)
    }
}
