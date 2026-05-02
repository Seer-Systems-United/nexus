//! # API sources endpoint tests
//!
//! Tests for source query parsing and API endpoint logic.

use nexus::api::sources::{SourceQuery, parse_scope};
use nexus::sources::Scope;

#[test]
fn defaults_to_latest_scope() {
    let scope = parse_scope(SourceQuery {
        scope: None,
        count: None,
        n: None,
        question: None,
    })
    .unwrap();

    assert_eq!(scope, Scope::Latest);
}

#[test]
fn parses_counted_scopes() {
    let scope = parse_scope(SourceQuery {
        scope: Some("last_days".to_string()),
        count: Some(30),
        n: None,
        question: None,
    })
    .unwrap();

    assert_eq!(scope, Scope::LastDays(30));
}

#[test]
fn rejects_missing_scope_count() {
    let error = parse_scope(SourceQuery {
        scope: Some("last_entries".to_string()),
        count: None,
        n: None,
        question: None,
    });

    assert!(error.is_err());
}
