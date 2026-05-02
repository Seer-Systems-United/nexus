//! # YouGov extraction tests
//!
//! Tests for YouGov document header extraction logic.

use nexus::sources::yougov::server::extract::extract_document_header;

#[test]
fn document_header_extraction_finds_title() {
    let lines = vec![
        String::new(),
        "The Economist/YouGov Poll".to_string(),
        "April 1 - 4, 2026".to_string(),
    ];
    let (title, subtitle) = extract_document_header(&lines).unwrap();
    assert_eq!(title, "The Economist/YouGov Poll");
    assert_eq!(subtitle.as_deref(), Some("April 1 - 4, 2026"));
}
