use crate::sources::Scope;
use crate::sources::date::SimpleDate;
use crate::sources::ipsos::server::download::DynError;
use std::io::Error as IoError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleStub {
    pub title: String,
    pub article_url: String,
    pub published_on: SimpleDate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleDetails {
    pub title: String,
    pub pdf_url: String,
}

#[derive(Default)]
pub(super) struct DownloadOutcome {
    pub(super) pdfs: Vec<super::super::IpsosPollPdf>,
    pub(super) candidates: usize,
    failures: usize,
    failure_samples: Vec<String>,
}

impl DownloadOutcome {
    pub(super) fn record_failure(&mut self, stub: &ArticleStub, error: DynError) {
        self.failures += 1;
        let failure = format!("{} [{}]: {error}", stub.title, stub.article_url);
        tracing::warn!(source = "ipsos", failure = %failure, "skipping Ipsos poll");
        if self.failure_samples.len() < 3 {
            self.failure_samples.push(failure);
        }
    }

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
