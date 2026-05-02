use nexus::sources::DataStructure;
use nexus::sources::ipsos::server::extract::{
    is_question_title, normalize_line, parse_questions, parse_row,
};

#[test]
fn parses_ipsos_question_titles() {
    assert!(is_question_title(
        "CP1. In your opinion, what is the most important problem facing the U.S. today?"
    ));
    assert!(is_question_title(
        "Approval5_1. Overall, do you approve or disapprove?"
    ));
    assert!(!is_question_title("www.ipsos.com"));
}

#[test]
fn parses_rows_with_asterisk_and_dash_values() {
    let row = parse_row("Skipped 1% 1% - *", 4).expect("row should parse");

    assert_eq!(row.label, "Skipped");
    assert_eq!(row.values, vec![1.0, 1.0, 0.0, 0.0]);
}

#[test]
fn parses_ipsos_crosstab_from_pdf_lines() {
    let text = r#"
        CP2. Generally speaking, would you say things in this country are heading in the right direction, or are
        they off on the wrong track?
        Total
        (N=1,269)
        Republican
        (N=435)
        Democrat
        (N=351)
        Independent/Something
        else
        (N=451)
        Right direction 19% 46% 4% 11%
        Wrong track 64% 33% 90% 72%
        Don’t know 16% 21% 6% 17%
        Skipped 1% 1% - *
    "#;
    let structures = parse_text(text);

    assert_eq!(structures.len(), 1);
    let DataStructure::Crosstab { title, panels, .. } = &structures[0] else {
        panic!("expected crosstab");
    };

    assert!(title.starts_with("CP2."));
    assert_eq!(
        panels[0].columns,
        vec![
            "Total",
            "Republican",
            "Democrat",
            "Independent/Something else"
        ]
    );
    assert_eq!(panels[0].rows[1].label, "Wrong track");
    assert_eq!(panels[0].rows[1].values, vec![64.0, 33.0, 90.0, 72.0]);
}

#[test]
fn parses_inline_total_sample_size_header() {
    let structures = parse_text(
        r#"
        Q1. Do you approve?
        Total (N=1,000)
        Republican (N=400)
        Democrat (N=350)
        Approve 40% 70% 10%
        Disapprove 55% 25% 85%
    "#,
    );

    assert_eq!(structures.len(), 1);
    let DataStructure::Crosstab { panels, .. } = &structures[0] else {
        panic!("expected crosstab");
    };

    assert_eq!(panels[0].columns, vec!["Total", "Republican", "Democrat"]);
    assert_eq!(panels[0].rows[0].values, vec![40.0, 70.0, 10.0]);
}

fn parse_text(text: &str) -> Vec<DataStructure> {
    let lines = text.lines().map(normalize_line).collect::<Vec<_>>();
    parse_questions(&lines)
}
