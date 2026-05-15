use chrono::NaiveDate;
use nexus::{
    database::{poll::DatabasePoll, response::DatabaseResponse, sqlite::SqliteBackend},
    expr::get::get,
    poll::{response::demographic::Demographic, source::yougov::YouGov},
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
        .from("04-15-2025")
        .to("04-15-2026")
        .from_question("approve")
        .from_demographic(Demographic::All)
        .execute_with(&store)
        .unwrap();

    assert_eq!(results, vec![response]);
}
