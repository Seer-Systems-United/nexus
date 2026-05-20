use std::sync::Arc;

use chrono::{NaiveDate, TimeZone, Utc};
use nexus::{
    database::{
        BackendTrait, demographic::DatabaseDemographic, poll::DatabasePoll,
        question::DatabaseQuestion, response::DatabaseResponse,
        response_unit::DatabaseResponseUnit, sqlite::SqliteBackend,
    },
    expr::get::get,
    poll::{
        Poll,
        question::Question,
        response::{Response, demographic::Demographic, unit::Unit},
        source::yougov::YouGov,
    },
};

#[test]
pub fn test_sqlite_store_executes_local_responses_expression() {
    let store = SqliteBackend::in_memory().unwrap();

    let source_id = uuid::Uuid::new_v4();
    let poll_id = uuid::Uuid::new_v4();
    let question_id = uuid::Uuid::new_v4();
    let demographic_id = uuid::Uuid::new_v4();
    let unit_id = uuid::Uuid::new_v4();
    let response = DatabaseResponse {
        id: uuid::Uuid::new_v4(),
        question_id,
        demographic_id,
        unit_id,
        answer: "Approve".to_string(),
        value: 52,
    };

    store.insert_source(source_id, "YouGov").unwrap();
    store
        .insert_poll(&DatabasePoll {
            id: poll_id,
            source_id,
            published_timestamp: NaiveDate::from_ymd_opt(2026, 4, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
        })
        .unwrap();
    store
        .insert_question(question_id, poll_id, "Do you approve?")
        .unwrap();
    store
        .insert_demographic(demographic_id, "all", "all")
        .unwrap();
    store.insert_response_unit(unit_id, "percent").unwrap();
    store.insert_response(&response).unwrap();

    let results = get()
        .responses()
        .from_source(YouGov)
        .from_source_id(source_id)
        .from("04-15-2025")
        .to("04-15-2026")
        .from_question("approve")
        .from_question_id(question_id)
        .from_demographic(Demographic::All)
        .execute_with(&store)
        .unwrap();

    assert_eq!(results, vec![response]);

    let demographics = store
        .get_demographics_by_ids(vec![demographic_id])
        .expect("demographic lookup should succeed");

    assert_eq!(
        demographics,
        vec![DatabaseDemographic {
            id: demographic_id,
            key: "all".to_string(),
            demographic_type: "all".to_string(),
            label: None,
            lower_bound: None,
            upper_bound: None,
            registered: None,
        }]
    );

    let units = store
        .get_response_units_by_ids(vec![unit_id])
        .expect("response unit lookup should succeed");

    assert_eq!(
        units,
        vec![DatabaseResponseUnit {
            id: unit_id,
            name: "percent".to_string(),
        }]
    );
}

#[test]
pub fn test_sqlite_store_executes_local_questions_expression() {
    let store = SqliteBackend::in_memory().unwrap();

    let source_id = uuid::Uuid::new_v4();
    let poll_id = uuid::Uuid::new_v4();
    let question_id = uuid::Uuid::new_v4();

    store.insert_source(source_id, "YouGov").unwrap();
    store
        .insert_poll(&DatabasePoll {
            id: poll_id,
            source_id,
            published_timestamp: NaiveDate::from_ymd_opt(2026, 4, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
        })
        .unwrap();
    store
        .insert_question(question_id, poll_id, "Do you approve?")
        .unwrap();

    let results = get()
        .questions()
        .from_source(YouGov)
        .from_source_id(source_id)
        .from("04-15-2025")
        .to("04-15-2026")
        .from_question("approve")
        .execute_with(&store)
        .unwrap();

    assert_eq!(
        results,
        vec![DatabaseQuestion {
            id: question_id,
            poll_id,
            text: "Do you approve?".to_string(),
            keywords: "Do you approve?".to_string(),
        }]
    );

    let stemmed_results = get()
        .questions()
        .from_source_id(source_id)
        .from_question("approval")
        .execute_with(&store)
        .unwrap();

    assert_eq!(stemmed_results, results);
}

#[test]
pub fn test_sqlite_store_saves_poll_graph_idempotently() {
    let store = SqliteBackend::in_memory().unwrap();
    let poll = Poll {
        published_timestamp: Utc.with_ymd_and_hms(2026, 4, 1, 12, 0, 0).unwrap(),
        questions: vec![Question::new(
            "Do you approve?",
            vec![Response {
                demographic: Demographic::All,
                answer: Arc::from("Approve"),
                value: 52,
                unit: Unit::Percent,
            }],
        )],
    };

    store.save_poll("YouGov", &poll).unwrap();
    store.save_poll("YouGov", &poll).unwrap();

    let results = get()
        .responses()
        .from_source(YouGov)
        .from_question("approve")
        .from_demographic(Demographic::All)
        .execute_with(&store)
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].answer, "Approve");
    assert_eq!(results[0].value, 52);
}
