//! # Topics demographics tests
//!
//! Tests for demographic mapping from polling data panels.

use nexus::sources::{DataGroup, DataPanel};
use nexus::topics::demographics::demographic_for_panel_column;

#[test]
fn maps_party_id_demographics_from_panel_groups() {
    let panel = DataPanel {
        columns: vec![
            "Total".to_string(),
            "Dem".to_string(),
            "Ind".to_string(),
            "Rep".to_string(),
        ],
        groups: vec![DataGroup {
            title: "Party ID".to_string(),
            labels: vec!["Dem".to_string(), "Ind".to_string(), "Rep".to_string()],
        }],
        rows: Vec::new(),
    };

    assert_eq!(demographic_for_panel_column(&panel, 0).id, "total");
    assert_eq!(demographic_for_panel_column(&panel, 1).id, "party-democrat");
    assert_eq!(
        demographic_for_panel_column(&panel, 2).id,
        "party-independent"
    );
    assert_eq!(
        demographic_for_panel_column(&panel, 3).id,
        "party-republican"
    );
}
