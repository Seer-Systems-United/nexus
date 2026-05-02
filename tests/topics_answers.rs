//! # Topics answers tests
//!
//! Tests for answer normalization and classification logic.

use nexus::topics::answers::normalize_answers;

const PRESIDENTIAL_APPROVAL_ID: &str = "presidential-approval";

#[test]
fn rolls_up_approval_components() {
    let answers = normalize_answers(
        PRESIDENTIAL_APPROVAL_ID,
        [
            ("Strongly approve", 15.0),
            ("Somewhat approve", 20.0),
            ("Somewhat disapprove", 10.0),
            ("Strongly disapprove", 50.0),
        ],
    );

    let approve = answers
        .iter()
        .find(|answer| answer.id == "approve")
        .expect("approve rollup should exist");
    let disapprove = answers
        .iter()
        .find(|answer| answer.id == "disapprove")
        .expect("disapprove rollup should exist");

    assert_eq!(approve.value, 35.0);
    assert_eq!(disapprove.value, 60.0);
}

#[test]
fn net_rows_override_component_rollups() {
    let answers = normalize_answers(
        PRESIDENTIAL_APPROVAL_ID,
        [
            ("Strongly approve", 15.0),
            ("Somewhat approve", 20.0),
            ("Approve (Net)", 34.0),
        ],
    );

    let approve = answers
        .iter()
        .find(|answer| answer.id == "approve")
        .expect("approve answer should exist");

    assert_eq!(approve.value, 34.0);
}
