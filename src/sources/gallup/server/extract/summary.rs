//! # Gallup collection subtitle generator
//!
//! Creates subtitle strings for Gallup data collections
//! showing date range and article count.

use crate::sources::Scope;

/// Generate a subtitle for a Gallup data collection.
///
/// # Parameters
/// - `scope`: The scope used for data loading.
/// - `articles`: The collected articles.
///
/// # Returns
/// - `Some(String)` with formatted subtitle.
/// - `None` if articles list is empty.
pub(super) fn collection_subtitle(
    scope: Scope,
    articles: &[crate::sources::gallup::server::GallupArticleAsset],
) -> Option<String> {
    let first = articles.first()?;
    let last = articles.last().unwrap_or(first);
    let article_label = if articles.len() == 1 {
        "article"
    } else {
        "articles"
    };

    Some(format!(
        "{} collection: {} to {} ({} {article_label})",
        scope.collection_label(),
        last.published_on,
        first.published_on,
        articles.len()
    ))
}
