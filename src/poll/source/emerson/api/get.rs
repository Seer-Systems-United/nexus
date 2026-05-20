use crate::poll::source::emerson::api::models::{self, Post};
use tracing::{debug, error, info, instrument};

#[instrument]
pub fn api_get(url: &str) -> Result<Vec<models::Post>, reqwest::Error> {
    info!(%url, "Fetching polls from Emerson API");
    let response = reqwest::blocking::get(url).map_err(|e| {
        error!(error = ?e, %url, "Failed to execute HTTP GET request");
        e
    })?;

    debug!(status = ?response.status(), "Received response from API");

    let posts = response.json::<Vec<Post>>().map_err(|e| {
        error!(error = ?e, "Failed to parse JSON response into Vec<Post>");
        e
    })?;

    info!(count = posts.len(), "Successfully retrieved Emerson posts");
    Ok(posts)
}
