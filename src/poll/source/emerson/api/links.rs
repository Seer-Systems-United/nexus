use crate::{poll::source::emerson::api::models::Post, utils::google::sheets::GoogleSheetLink};
use regex::Regex;
use tracing::{debug, error, info, instrument, warn};

#[instrument(skip(posts))]
pub fn find_spreadsheet_links(posts: &[Post]) -> Vec<GoogleSheetLink> {
    let re =
        Regex::new(r#"https://docs\.google\.com/spreadsheets/d/[^"'<\s]+"#).unwrap_or_else(|e| {
            error!(error = ?e, "Failed to compile Google Sheets regex");
            panic!(
                "Critical failure: Spreadsheet regex compilation failed: {}",
                e
            );
        });

    let links: Vec<GoogleSheetLink> = posts
        .iter()
        .flat_map(|post| {
            let matches: Vec<_> = re
                .find_iter(&post.content.rendered)
                .map(|m| {
                    let link = m.as_str().to_string();
                    debug!(%link, post_id = post.id, "Found spreadsheet link in post content");
                    GoogleSheetLink::new(link)
                })
                .collect();
            matches
        })
        .collect();

    if links.is_empty() {
        warn!("No spreadsheet links were found in the provided posts");
    } else {
        info!(
            count = links.len(),
            "Successfully extracted spreadsheet links"
        );
    }

    links
}
