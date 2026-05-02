//! # Ipsos extraction summary
//!
//! Creates subtitle strings for Ipsos data collections
//! showing date range and poll count.

use crate::sources::Scope;

/// Generate a subtitle for an Ipsos data collection.
///
/// # Parameters
/// - `scope`: The scope used for data loading.
/// - `pdfs`: The collected poll PDFs.
///
/// # Returns
/// - `Some(String)` with formatted subtitle.
/// - `None` if PDFs list is empty.
pub(super) fn collection_subtitle(
    scope: Scope,
    pdfs: &[crate::sources::ipsos::server::IpsosPollPdf],
) -> Option<String> {
    let first = pdfs.first()?;
    let last = pdfs.last().unwrap_or(first);
    let poll_label = if pdfs.len() == 1 { "poll" } else { "polls" };

    Some(format!(
        "{} collection: {} to {} ({} {poll_label})",
        scope.collection_label(),
        last.published_on,
        first.published_on,
        pdfs.len()
    ))
}
