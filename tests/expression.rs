use nexus::{
    expr::{
        get::get,
        ops::{Filter, NameField, Table},
    },
    poll::{response::demographic::Demographic, source::yougov::YouGov},
};

#[test]
pub fn test_get_basic_expression() {
    get().names();
}

#[test]
pub fn test_get_names_expression() {
    let expr = get().names().where_as(NameField::FirstName, "John");

    assert_eq!(expr.table(), Some(Table::People));
    assert_eq!(
        expr.filters(),
        &[Filter::Name {
            field: NameField::FirstName,
            value: "John".to_string()
        }]
    );
}

#[test]
pub fn test_get_polls_expression() {
    let expr = get()
        .polls()
        .from_source(YouGov)
        .from("04-15-2025")
        .to("04-15-2026");

    assert_eq!(expr.table(), Some(Table::Polls));
    assert_eq!(
        expr.filters(),
        &[
            Filter::PollSource {
                source_name: "YouGov"
            },
            Filter::PollFrom {
                date: "04-15-2025".to_string()
            },
            Filter::PollTo {
                date: "04-15-2026".to_string()
            },
        ]
    );
}

#[test]
pub fn test_get_questions_expression() {
    let expr = get()
        .questions()
        .from_source(YouGov)
        .from("04-15-2025")
        .to("04-15-2026")
        .from_question("Do you approve?");

    assert_eq!(expr.table(), Some(Table::Questions));
    assert_eq!(
        expr.filters(),
        &[
            Filter::QuestionSource {
                source_name: "YouGov"
            },
            Filter::QuestionFrom {
                date: "04-15-2025".to_string()
            },
            Filter::QuestionTo {
                date: "04-15-2026".to_string()
            },
            Filter::QuestionQuestion {
                question: "Do you approve?".to_string()
            },
        ]
    );
}

#[test]
pub fn test_get_responses_expression() {
    let expr = get()
        .responses()
        .from_source(YouGov)
        .from("04-15-2025")
        .to("04-15-2026")
        .from_question("Do you approve?")
        .from_demographic(Demographic::All);

    assert_eq!(expr.table(), Some(Table::Responses));
    assert_eq!(
        expr.filters(),
        &[
            Filter::ResponseSource {
                source_name: "YouGov"
            },
            Filter::ResponseFrom {
                date: "04-15-2025".to_string()
            },
            Filter::ResponseTo {
                date: "04-15-2026".to_string()
            },
            Filter::ResponseQuestion {
                question: "Do you approve?".to_string()
            },
            Filter::ResponseDemographic {
                demographic_key: "all".to_string()
            },
        ]
    );
}
