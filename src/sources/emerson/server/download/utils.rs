pub(crate) const EMERSON_BLOG_URL: &str = "https://emersoncollegepolling.com/blog/";
pub(crate) const GOOGLE_SHEETS_PATH_FRAGMENT: &str = "/spreadsheets/d/";
pub(crate) const MAX_BLOG_PAGES: usize = 36;

pub(crate) fn missing_resource_error(resource: &'static str) -> super::DynError {
    std::io::Error::new(std::io::ErrorKind::NotFound, resource).into()
}

pub(crate) fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn workbook_download_url(sheet_id: &str) -> String {
    format!("https://docs.google.com/spreadsheets/d/{sheet_id}/export?format=xlsx")
}

pub(crate) fn emerson_blog_page_url(page_number: usize) -> String {
    if page_number <= 1 {
        EMERSON_BLOG_URL.to_string()
    } else {
        format!("{EMERSON_BLOG_URL}page/{page_number}/")
    }
}
