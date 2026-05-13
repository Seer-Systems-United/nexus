use crate::poll::{
    question::Question,
    source::yougov::api::{
        get::{get_latest_editorial_document, get_latest_editorial_pdf_pages},
        parse::parse_pages,
    },
};
use crate::utils::pdf::extract::extract_pdf_from_url;
use chrono::{DateTime, Utc};

pub mod get;
pub mod models;
pub mod parse;

pub fn latest_survey() -> Vec<Question> {
    parse_pages(&get_latest_editorial_pdf_pages())
}

pub fn latest_survey_with_timestamp() -> (Vec<Question>, DateTime<Utc>) {
    let Some(document) = get_latest_editorial_document() else {
        return (Vec::new(), Utc::now());
    };

    let published_timestamp = DateTime::parse_from_rfc3339(&document.created_at)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    (
        parse_pages(&extract_pdf_from_url(&document.url)),
        published_timestamp,
    )
}
