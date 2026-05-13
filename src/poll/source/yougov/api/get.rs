use crate::utils::pdf::extract::extract_pdf_from_url;

use super::models::{CmsDocumentEditorial, SurveyApiResponse, SurveyResponseItem};

const LOOKUP_API_URL: &str = "https://api-test.yougov.com/public-data/v5/us/search/entity/bb747389-a904-11e1-9412-005056900141/surveys/";

pub fn get_latest_editorial_pdf_pages() -> Vec<String> {
    get_latest_editorial_document()
        .map(|document| extract_pdf_from_url(&document.url))
        .unwrap_or_default()
}

pub fn get_latest_editorial_url() -> String {
    get_latest_editorial_document()
        .map(|document| document.url)
        .unwrap_or_default()
}

pub fn get_latest_editorial_document() -> Option<CmsDocumentEditorial> {
    let response = get_latest_surveys();

    for item in response.data {
        if let SurveyResponseItem::CmsDocumentEditorial(document) = item {
            return Some(document);
        }
    }

    None
}

pub fn get_latest_surveys() -> SurveyApiResponse {
    let response = reqwest::blocking::get(LOOKUP_API_URL)
        .unwrap()
        .json::<SurveyApiResponse>()
        .unwrap();

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poll::source::yougov::api::latest_survey;

    #[test]
    fn test_get_latest_surveys() {
        let response = get_latest_surveys();
        assert!(response.hits > 0);
        assert!(!response.data.is_empty());
    }

    #[test]
    fn test_parse_latest_survey() {
        // This test will just check that we can call the function without panicking.
        latest_survey();
    }
}
