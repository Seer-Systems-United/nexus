use nexus::sources::date::SimpleDate;
use nexus::sources::ipsos::server::download::{
    parse_article_details, parse_landing_stubs, parse_text_date,
};

#[test]
fn parses_landing_poll_headlines() {
    let html = r#"
        <section class="block-publications-content">
            <div class="block-wysiwyg">
                <h3>April 2026:</h3>
                <h2>Americans increasingly feel the economy is on the wrong track</h2>
                <p><strong>Washington, D.C., April 28, 2026 - </strong>This
                   <a href="/en-us/americans-increasingly-feel-economy-wrong-track">latest Reuters/Ipsos poll</a>
                   finds the economy is on the wrong track.</p>
                <h2>External-only item</h2>
                <p><strong>Washington, D.C., April 27, 2026 - </strong>Read
                   <a href="https://example.com/report">the report</a>.</p>
            </div>
        </section>
    "#;

    let stubs = parse_landing_stubs(html).expect("stubs should parse");

    assert_eq!(stubs.len(), 1);
    assert_eq!(
        stubs[0].article_url,
        "https://www.ipsos.com/en-us/americans-increasingly-feel-economy-wrong-track"
    );
    assert_eq!(stubs[0].published_on, SimpleDate::new(2026, 4, 28));
}

#[test]
fn parses_download_center_pdf_link() {
    let html = r#"
        <h1>Americans increasingly feel the economy is on the wrong track</h1>
        <section class="block-download-center">
            <a href="https://www.ipsos.com/sites/default/files/topline.pdf?download=1">
                Download pdf
            </a>
        </section>
    "#;

    let details = parse_article_details(html).expect("details should parse");

    assert_eq!(
        details.title,
        "Americans increasingly feel the economy is on the wrong track"
    );
    assert_eq!(
        details.pdf_url,
        "https://www.ipsos.com/sites/default/files/topline.pdf"
    );
}

#[test]
fn parses_ipsos_text_dates() {
    let date = parse_text_date("Washington, DC, September 27, 2025 - New polling")
        .expect("date should parse");

    assert_eq!(date, SimpleDate::new(2025, 9, 27));
}
