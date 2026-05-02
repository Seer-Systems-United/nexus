use nexus::sources::DataStructure;
use nexus::sources::emerson::server::extract::parse_crosstab_sheet;

fn empty() -> calamine::Data {
    calamine::Data::Empty
}

fn text(value: &str) -> calamine::Data {
    calamine::Data::String(value.to_string())
}

fn number(value: f64) -> calamine::Data {
    calamine::Data::Float(value)
}

#[test]
fn crosstab_rows_are_answer_options_with_values_across_columns() {
    let rows = vec![
        vec![empty(), empty(), text("Do you approve? - Selected Choice")],
        vec![
            empty(),
            empty(),
            text("Approve"),
            empty(),
            text("Disapprove"),
            empty(),
        ],
        vec![],
        vec![
            text("Party"),
            text("Democrat"),
            empty(),
            number(0.75),
            empty(),
            number(0.15),
        ],
        vec![
            empty(),
            text("Republican"),
            empty(),
            number(0.20),
            empty(),
            number(0.70),
        ],
        vec![
            text("Race"),
            text("White"),
            empty(),
            number(45.0),
            empty(),
            number(40.0),
        ],
    ];

    let structures = parse_crosstab_sheet(&rows);

    assert_eq!(structures.len(), 1);
    let DataStructure::Crosstab { title, panels, .. } = &structures[0] else {
        panic!("expected crosstab");
    };

    assert_eq!(title, "Do you approve?");
    assert_eq!(panels[0].columns, vec!["Democrat", "Republican", "White"]);
    assert_eq!(panels[0].rows[0].label, "Approve");
    assert_eq!(panels[0].rows[0].values, vec![75.0, 20.0, 45.0]);
    assert_eq!(panels[0].rows[1].label, "Disapprove");
    assert_eq!(panels[0].rows[1].values, vec![15.0, 70.0, 40.0]);
}
