use tracing::{debug, info, instrument};

#[instrument]
pub fn get_latest_n_polls_url(last_n: usize) -> String {
    // Emerson uses WordPress REST API so we can fetch the latest poll uusing this url
    let url = format!(
        "https://emersoncollegepolling.com/wp-json/wp/v2/posts?per_page={}&orderby=date&order=desc&_fields=id,date,link,title.rendered,content.rendered",
        last_n
    );
    info!(%url, "Constructed Emerson API URL");
    url
}

#[instrument]
pub fn get_latest_poll_url() -> String {
    debug!("Fetching URL for single latest poll");
    get_latest_n_polls_url(1)
}
