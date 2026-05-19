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
    let person_id = uuid::Uuid::new_v4();
    let person_ids = [uuid::Uuid::new_v4(), uuid::Uuid::new_v4()];
    let expr = get()
        .names()
        .by_id(person_id)
        .by_ids(person_ids)
        .where_as(NameField::FirstName, "John");

    assert_eq!(expr.table(), Some(Table::People));
    assert_eq!(
        expr.filters(),
        &[
            Filter::PersonId { person_id },
            Filter::PersonIds {
                person_ids: person_ids.into(),
            },
            Filter::Name {
                field: NameField::FirstName,
                value: "John".to_string()
            }
        ]
    );
}

#[test]
pub fn test_get_polls_expression() {
    let poll_id = uuid::Uuid::new_v4();
    let poll_ids = [uuid::Uuid::new_v4(), uuid::Uuid::new_v4()];
    let expr = get()
        .polls()
        .by_id(poll_id)
        .by_ids(poll_ids)
        .from_source(YouGov)
        .from("04-15-2025")
        .to("04-15-2026");

    assert_eq!(expr.table(), Some(Table::Polls));
    assert_eq!(
        expr.filters(),
        &[
            Filter::PollId { poll_id },
            Filter::PollIds {
                poll_ids: poll_ids.into(),
            },
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
    let question_id = uuid::Uuid::new_v4();
    let question_ids = [uuid::Uuid::new_v4(), uuid::Uuid::new_v4()];
    let source_id = uuid::Uuid::new_v4();
    let expr = get()
        .questions()
        .by_id(question_id)
        .by_ids(question_ids)
        .from_source(YouGov)
        .from_source_id(source_id)
        .from("04-15-2025")
        .to("04-15-2026")
        .from_question("Do you approve?");

    assert_eq!(expr.table(), Some(Table::Questions));
    assert_eq!(
        expr.filters(),
        &[
            Filter::QuestionId { question_id },
            Filter::QuestionIds {
                question_ids: question_ids.into(),
            },
            Filter::QuestionSource {
                source_name: "YouGov"
            },
            Filter::QuestionSourceId { source_id },
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
    let response_id = uuid::Uuid::new_v4();
    let response_ids = [uuid::Uuid::new_v4(), uuid::Uuid::new_v4()];
    let source_id = uuid::Uuid::new_v4();
    let question_id = uuid::Uuid::new_v4();
    let expr = get()
        .responses()
        .by_id(response_id)
        .by_ids(response_ids)
        .from_source(YouGov)
        .from_source_id(source_id)
        .from("04-15-2025")
        .to("04-15-2026")
        .from_question("Do you approve?")
        .from_question_id(question_id)
        .from_demographic(Demographic::All);

    assert_eq!(expr.table(), Some(Table::Responses));
    assert_eq!(
        expr.filters(),
        &[
            Filter::ResponseId { response_id },
            Filter::ResponseIds {
                response_ids: response_ids.into(),
            },
            Filter::ResponseSource {
                source_name: "YouGov"
            },
            Filter::ResponseSourceId { source_id },
            Filter::ResponseFrom {
                date: "04-15-2025".to_string()
            },
            Filter::ResponseTo {
                date: "04-15-2026".to_string()
            },
            Filter::ResponseQuestion {
                question: "Do you approve?".to_string()
            },
            Filter::ResponseQuestionId { question_id },
            Filter::ResponseDemographic {
                demographic_key: "all".to_string()
            },
        ]
    );
}
