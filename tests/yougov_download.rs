use nexus::sources::yougov::server::download::{clean_pdf_url, is_economist_crosstabs_pdf_url};

#[test]
fn matches_yougov_crosstab_pdf_links_with_page_fragments() {
    let href = "https://d3nkl3psvxxpe9.cloudfront.net/documents/econTabReport_Ty7ikPd.pdf#page=44";

    assert!(is_economist_crosstabs_pdf_url(href));
    assert_eq!(
        clean_pdf_url(href),
        "https://d3nkl3psvxxpe9.cloudfront.net/documents/econTabReport_Ty7ikPd.pdf"
    );
}
