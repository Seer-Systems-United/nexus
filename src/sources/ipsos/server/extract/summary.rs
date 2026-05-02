use crate::sources::Scope;

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
