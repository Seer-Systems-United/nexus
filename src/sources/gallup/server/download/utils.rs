pub(crate) const GALLUP_BASE_URL: &str = "https://news.gallup.com";
pub(crate) const GALLUP_SEARCH_URL: &str =
    "https://news.gallup.com/Search/raw.aspx?s=date&topic=1&cn=ALL_GALLUP_HEADLINES";
pub(crate) const MAX_SEARCH_PAGES: usize = 36;

pub(crate) fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn absolute_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("{GALLUP_BASE_URL}{url}")
    }
}

pub(crate) fn gallup_search_page_url(page_number: usize) -> String {
    format!("{GALLUP_SEARCH_URL}&p={page_number}")
}

pub(crate) fn datawrapper_dataset_url(chart_url: &str) -> String {
    let normalized = chart_url.trim_end_matches('/');
    format!("{normalized}/dataset.csv")
}
