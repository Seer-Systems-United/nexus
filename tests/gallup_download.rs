use nexus::sources::gallup::server::download::{
    datawrapper_dataset_url, gallup_search_page_url, parse_article_assets, parse_search_stubs,
};

#[test]
fn search_url_includes_page_number() {
    let url = gallup_search_page_url(3);
    assert!(url.contains("p=3"));
}

#[test]
fn parses_poll_tiles_from_gallup_search_results() {
    let html = r#"
        <section class="cmstile tile-feature">
            <div class="meta clearfix"><time datetime="2026-04-22">Apr 22, 2026</time></div>
            <div class="copy">
                <div class="tile-linktext"><h3><a href="/poll/708722/disapproval-congress-ties-record-high.aspx">Disapproval of Congress Ties Record High at 86%</a></h3></div>
            </div>
        </section>
        <section class="cmstile tile-feature">
            <div class="meta clearfix"><time datetime="2026-04-20">Apr 20, 2026</time></div>
            <div class="copy">
                <div class="tile-linktext"><h3><a href="/opinion/gallup/708557/value-silver-tsunami-depends-intangible-assets.aspx">Not a poll</a></h3></div>
            </div>
        </section>
    "#;

    let stubs = parse_search_stubs(html).expect("Gallup stubs should parse");

    assert_eq!(stubs.len(), 1);
    assert_eq!(
        stubs[0].article_url,
        "https://news.gallup.com/poll/708722/disapproval-congress-ties-record-high.aspx"
    );
}

#[test]
fn parses_pdf_and_chart_assets_from_article_html() {
    let html = r#"
        <html>
            <body>
                <a href="https://news.gallup.com/file/poll/708728/example.pdf">(PDF download)</a>
                <iframe data-src="https://datawrapper.dwcdn.net/Ne1q8/8/" title="Congress trend"></iframe>
            </body>
        </html>
    "#;

    let assets = parse_article_assets(html).expect("Gallup assets should parse");

    assert_eq!(
        assets.pdf_url.as_deref(),
        Some("https://news.gallup.com/file/poll/708728/example.pdf")
    );
    assert_eq!(assets.charts.len(), 1);
    assert_eq!(
        datawrapper_dataset_url(&assets.charts[0].chart_url),
        "https://datawrapper.dwcdn.net/Ne1q8/8/dataset.csv"
    );
}
