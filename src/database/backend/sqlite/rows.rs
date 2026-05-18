use diesel::{Queryable, prelude::Insertable};

use crate::{
    database::{
        person::DatabasePerson, poll::DatabasePoll, question::DatabaseQuestion,
        response::DatabaseResponse,
    },
    expr::ExpressionError,
};

use super::{
    schema,
    util::{format_datetime, parse_datetime, parse_uuid},
};

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::people)]
pub(super) struct SqlitePerson {
    pub(super) id: String,
    pub(super) given_name: String,
    pub(super) surname: String,
    pub(super) suffix: Option<String>,
    pub(super) prefix: Option<String>,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::polls)]
pub(super) struct SqlitePoll {
    pub(super) id: String,
    pub(super) source_id: String,
    pub(super) published_timestamp: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::responses)]
pub(super) struct SqliteResponse {
    pub(super) id: String,
    pub(super) question_id: String,
    pub(super) demographic_id: String,
    pub(super) unit_id: String,
    pub(super) answer: String,
    pub(super) value: i32,
}

#[derive(Insertable)]
#[diesel(table_name = schema::sources)]
pub(super) struct SqliteSource {
    pub(super) id: String,
    pub(super) name: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::questions)]
pub(super) struct SqliteQuestion {
    pub(super) id: String,
    pub(super) text: String,
    pub(super) keywords: String,
    pub(super) poll_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::demographics)]
pub(super) struct SqliteDemographic {
    pub(super) id: String,
    pub(super) key: String,
    pub(super) demographic_type: String,
    pub(super) label: Option<String>,
    pub(super) lower_bound: Option<i32>,
    pub(super) upper_bound: Option<i32>,
    pub(super) registered: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::response_units)]
pub(super) struct SqliteResponseUnit {
    pub(super) id: String,
    pub(super) name: String,
}

impl TryFrom<SqlitePerson> for DatabasePerson {
    type Error = ExpressionError;

    fn try_from(row: SqlitePerson) -> Result<Self, Self::Error> {
        Ok(Self {
            id: parse_uuid(row.id)?,
            given_name: row.given_name,
            surname: row.surname,
            suffix: row.suffix,
            prefix: row.prefix,
        })
    }
}

impl From<&DatabasePerson> for SqlitePerson {
    fn from(person: &DatabasePerson) -> Self {
        Self {
            id: person.id.to_string(),
            given_name: person.given_name.clone(),
            surname: person.surname.clone(),
            suffix: person.suffix.clone(),
            prefix: person.prefix.clone(),
        }
    }
}

impl TryFrom<SqlitePoll> for DatabasePoll {
    type Error = ExpressionError;

    fn try_from(row: SqlitePoll) -> Result<Self, Self::Error> {
        Ok(Self {
            id: parse_uuid(row.id)?,
            source_id: parse_uuid(row.source_id)?,
            published_timestamp: parse_datetime(&row.published_timestamp)?,
        })
    }
}

impl From<&DatabasePoll> for SqlitePoll {
    fn from(poll: &DatabasePoll) -> Self {
        Self {
            id: poll.id.to_string(),
            source_id: poll.source_id.to_string(),
            published_timestamp: format_datetime(poll.published_timestamp),
        }
    }
}

impl TryFrom<SqliteQuestion> for DatabaseQuestion {
    type Error = ExpressionError;

    fn try_from(row: SqliteQuestion) -> Result<Self, Self::Error> {
        Ok(Self {
            id: parse_uuid(row.id)?,
            poll_id: parse_uuid(row.poll_id)?,
            text: row.text,
            keywords: row.keywords,
        })
    }
}

impl From<&DatabaseQuestion> for SqliteQuestion {
    fn from(question: &DatabaseQuestion) -> Self {
        Self {
            id: question.id.to_string(),
            poll_id: question.poll_id.to_string(),
            text: question.text.clone(),
            keywords: question.keywords.clone(),
        }
    }
}

impl TryFrom<SqliteResponse> for DatabaseResponse {
    type Error = ExpressionError;

    fn try_from(row: SqliteResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            id: parse_uuid(row.id)?,
            question_id: parse_uuid(row.question_id)?,
            demographic_id: parse_uuid(row.demographic_id)?,
            unit_id: parse_uuid(row.unit_id)?,
            answer: row.answer,
            value: row.value,
        })
    }
}

impl From<&DatabaseResponse> for SqliteResponse {
    fn from(response: &DatabaseResponse) -> Self {
        Self {
            id: response.id.to_string(),
            question_id: response.question_id.to_string(),
            demographic_id: response.demographic_id.to_string(),
            unit_id: response.unit_id.to_string(),
            answer: response.answer.clone(),
            value: response.value,
        }
    }
}
