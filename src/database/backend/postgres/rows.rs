use diesel::Queryable;
use tracing::trace;

use crate::database::{person::DatabasePerson, poll::DatabasePoll, response::DatabaseResponse};

#[derive(Queryable)]
pub(super) struct PersonRow {
    id: uuid::Uuid,
    given_name: String,
    surname: String,
    suffix: Option<String>,
    prefix: Option<String>,
}

#[derive(Queryable)]
pub(super) struct PollRow {
    id: uuid::Uuid,
    source_id: uuid::Uuid,
    published_timestamp: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub(super) struct ResponseRow {
    id: uuid::Uuid,
    question_id: uuid::Uuid,
    demographic_id: uuid::Uuid,
    unit_id: uuid::Uuid,
    answer: String,
    value: i32,
}

impl From<PersonRow> for DatabasePerson {
    fn from(row: PersonRow) -> Self {
        trace!(person_id = %row.id, "Converting PersonRow to DatabasePerson");
        Self {
            id: row.id,
            given_name: row.given_name,
            surname: row.surname,
            suffix: row.suffix,
            prefix: row.prefix,
        }
    }
}

impl From<PollRow> for DatabasePoll {
    fn from(row: PollRow) -> Self {
        trace!(poll_id = %row.id, "Converting PollRow to DatabasePoll");
        Self {
            id: row.id,
            source_id: row.source_id,
            published_timestamp: row.published_timestamp,
        }
    }
}

impl From<ResponseRow> for DatabaseResponse {
    fn from(row: ResponseRow) -> Self {
        trace!(response_id = %row.id, "Converting ResponseRow to DatabaseResponse");
        Self {
            id: row.id,
            question_id: row.question_id,
            demographic_id: row.demographic_id,
            unit_id: row.unit_id,
            answer: row.answer,
            value: row.value,
        }
    }
}
