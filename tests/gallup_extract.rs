//! # Gallup extraction tests
//!
//! Tests for Gallup chart CSV parsing and extraction logic.

use nexus::sources::DataStructure;
use nexus::sources::gallup::server::extract::parse_chart_csv;

#[test]
fn parses_breakout_csv_as_crosstab() {
    let chart = parse_chart_csv(
        "Community involvement",
        b"Breakouts,Have volunteered,No but wanted to,No and not wanted to\nAll adults,39,24,37\nMore free time,41%,20%,39%\n",
    )
    .expect("chart should parse");

    let DataStructure::Crosstab { panels, .. } = chart else {
        panic!("expected crosstab");
    };

    assert_eq!(panels[0].columns[0], "Have volunteered");
    assert_eq!(panels[0].rows[0].label, "All adults");
    assert_eq!(panels[0].rows[0].values, vec![39.0, 24.0, 37.0]);
}

#[test]
fn parses_temporal_csv_as_line_graph() {
    let chart = parse_chart_csv(
        "Approval trend",
        b"Year,Approve,Disapprove\n2024,45,50\n2025,47,49\n",
    )
    .expect("chart should parse");

    let DataStructure::LineGraph { x, series, .. } = chart else {
        panic!("expected line graph");
    };

    assert_eq!(x, vec!["2024", "2025"]);
    assert_eq!(series[0].label, "Approve");
    assert_eq!(series[0].values, vec![45.0, 47.0]);
}
