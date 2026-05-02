//! # Emerson download tests
//!
//! Tests for Emerson release parsing and download URL logic.

use nexus::sources::date::SimpleDate;
use nexus::sources::emerson::server::download::{
    emerson_blog_page_url, parse_release_details, parse_release_stubs,
};

#[test]
fn blog_url_includes_page_number() {
    let url = emerson_blog_page_url(2);
    assert!(url.contains("page/2"));
}

#[test]
fn parses_release_stubs_from_blog_listing() {
    let html = r#"
        <div class="post-list">
            <div class="item-post">
                <div class="action"><a href="https://example.com/ca/">Full Release &amp; Results</a></div>
                <div class="meta-info"><span>Date: </span><span>16th Apr 2026</span></div>
            </div>
            <div class="item-post">
                <div class="action"><a href="https://example.com/mi/">Full Release &amp; Results</a></div>
                <div class="meta-info"><span>Date: </span><span>16th Apr 2026</span></div>
            </div>
            <div class="item-post">
                <div class="action"><a href="https://example.com/nyc/">Full Release &amp; Results</a></div>
                <div class="meta-info"><span>Date: </span><span>09th Apr 2026</span></div>
            </div>
        </div>
    "#;

    let releases = parse_release_stubs(html).expect("releases should parse");

    assert_eq!(releases.len(), 3);
    assert_eq!(releases[0].article_url, "https://example.com/ca/");
    assert_eq!(releases[1].published_on, SimpleDate::new(2026, 4, 16));
    assert_eq!(releases[2].published_on, SimpleDate::new(2026, 4, 9));
}

#[test]
fn extracts_release_title_and_google_sheet_id() {
    let html = r#"
        <html>
            <body>
                <h1>California 2026 Poll: Sample Title</h1>
                <a href="https://docs.google.com/spreadsheets/d/abc123/edit?gid=1982529522#gid=1982529522">FULL RESULTS</a>
            </body>
        </html>
    "#;

    let release = parse_release_details(html).expect("release details should parse");

    assert_eq!(release.title, "California 2026 Poll: Sample Title");
    assert_eq!(release.sheet_id, "abc123");
}
