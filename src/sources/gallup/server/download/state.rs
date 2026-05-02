//! # Gallup download state tracking
//!
//! Tracks Gallup article downloads: seen URLs, collected articles,
//! failure counts, and pagination decisions.

use super::article::ArticleDownload;
use crate::sources::Scope;
use std::collections::HashSet;

/// Tracks the state of a Gallup download operation.
pub(super) struct DownloadState {
    scope: Scope,
    seen_articles: HashSet<String>,
    articles: Vec<crate::sources::gallup::server::GallupArticleAsset>,
    stale_pages: usize,
    pdf_failures: usize,
    chart_failures: usize,
    skipped_articles: usize,
}

impl DownloadState {
    /// Create a new `DownloadState` for the given scope.
    pub(super) fn new(scope: Scope) -> Self {
        Self {
            scope,
            seen_articles: HashSet::new(),
            articles: Vec::new(),
            stale_pages: 0,
            pdf_failures: 0,
            chart_failures: 0,
            skipped_articles: 0,
        }
    }

    /// Mark an article URL as seen; returns true if it was new.
    pub(super) fn mark_seen(&mut self, article_url: &str) -> bool {
        self.seen_articles.insert(article_url.to_string())
    }

    /// Record a downloaded article (or skipped result).
    pub(super) fn record_download(&mut self, downloaded: ArticleDownload) {
        self.pdf_failures += usize::from(downloaded.pdf_failed);
        self.chart_failures += downloaded.chart_failures;
        self.skipped_articles += usize::from(downloaded.skipped);
        if let Some(article) = downloaded.article {
            self.articles.push(article);
        }
    }

    /// Check if the entry limit has been reached.
    pub(super) fn limit_reached(&self, entry_limit: Option<usize>) -> bool {
        entry_limit
            .map(|limit| self.articles.len() >= limit)
            .unwrap_or(false)
    }

    /// Decide whether to stop paginating based on stale pages.
    pub(super) fn should_stop_after_page(&mut self, page_has_scoped_article: bool) -> bool {
        if page_has_scoped_article {
            self.stale_pages = 0;
            return false;
        }

        self.stale_pages += 1;
        self.stale_pages >= 2
    }

    /// Finish the download and return collected articles.
    ///
    /// Sorts articles by publication date (newest first) and logs a summary.
    pub(super) fn finish(mut self) -> Vec<crate::sources::gallup::server::GallupArticleAsset> {
        self.articles
            .sort_by(|left, right| right.published_on.cmp(&left.published_on));
        tracing::info!(
            source = "gallup",
            scope = %self.scope,
            articles = self.articles.len(),
            pdf_failures = self.pdf_failures,
            chart_failures = self.chart_failures,
            skipped_articles = self.skipped_articles,
            "downloaded Gallup source assets"
        );
        self.articles
    }
}
