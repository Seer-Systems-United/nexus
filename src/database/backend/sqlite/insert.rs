use diesel::{RunQueryDsl, SqliteConnection};
use tracing::{debug, instrument};

use crate::{
    database::{person::DatabasePerson, poll::DatabasePoll, response::DatabaseResponse},
    expr::ExpressionError,
};

use super::{
    question_search::upsert_question_fts,
    rows::{
        SqliteDemographic, SqlitePerson, SqlitePoll, SqliteQuestion, SqliteResponse,
        SqliteResponseUnit, SqliteSource,
    },
    schema,
};

#[instrument(skip(conn, person))]
pub(super) fn insert_person(
    conn: &mut SqliteConnection,
    person: &DatabasePerson,
) -> Result<(), ExpressionError> {
    debug!(?person, "Inserting person");
    let row = SqlitePerson::from(person);
    diesel::insert_into(schema::people::table)
        .values(&row)
        .execute(conn)?;
    Ok(())
}

#[instrument(skip(conn, poll))]
pub(super) fn insert_poll(
    conn: &mut SqliteConnection,
    poll: &DatabasePoll,
) -> Result<(), ExpressionError> {
    debug!(?poll, "Inserting poll");
    let row = SqlitePoll::from(poll);
    diesel::insert_into(schema::polls::table)
        .values(&row)
        .execute(conn)?;
    Ok(())
}

#[instrument(skip(conn, response))]
pub(super) fn insert_response(
    conn: &mut SqliteConnection,
    response: &DatabaseResponse,
) -> Result<(), ExpressionError> {
    debug!(?response, "Inserting response");
    let row = SqliteResponse::from(response);
    diesel::insert_into(schema::responses::table)
        .values(&row)
        .execute(conn)?;
    Ok(())
}

#[instrument(skip(conn, name))]
pub(super) fn insert_source(
    conn: &mut SqliteConnection,
    id: uuid::Uuid,
    name: impl AsRef<str>,
) -> Result<(), ExpressionError> {
    debug!(%id, name = %name.as_ref(), "Inserting source");
    let row = SqliteSource {
        id: id.to_string(),
        name: name.as_ref().to_string(),
    };
    diesel::insert_into(schema::sources::table)
        .values(&row)
        .execute(conn)?;
    Ok(())
}

#[instrument(skip(conn, text))]
pub(super) fn insert_question(
    conn: &mut SqliteConnection,
    id: uuid::Uuid,
    poll_id: uuid::Uuid,
    text: impl AsRef<str>,
) -> Result<(), ExpressionError> {
    debug!(%id, %poll_id, text = %text.as_ref(), "Inserting question");
    let text = text.as_ref().to_string();
    let row = SqliteQuestion {
        id: id.to_string(),
        text: text.clone(),
        keywords: text,
        poll_id: poll_id.to_string(),
    };
    diesel::insert_into(schema::questions::table)
        .values(&row)
        .execute(conn)?;
    upsert_question_fts(conn, &row.id, &row.text, &row.keywords)?;
    Ok(())
}

#[instrument(skip(conn, key, demographic_type))]
pub(super) fn insert_demographic(
    conn: &mut SqliteConnection,
    id: uuid::Uuid,
    key: impl AsRef<str>,
    demographic_type: impl AsRef<str>,
) -> Result<(), ExpressionError> {
    debug!(%id, key = %key.as_ref(), demographic_type = %demographic_type.as_ref(), "Inserting demographic");
    let row = SqliteDemographic {
        id: id.to_string(),
        key: key.as_ref().to_string(),
        demographic_type: demographic_type.as_ref().to_string(),
        label: None,
        lower_bound: None,
        upper_bound: None,
        registered: None,
    };
    diesel::insert_into(schema::demographics::table)
        .values(&row)
        .execute(conn)?;
    Ok(())
}

#[instrument(skip(conn, name))]
pub(super) fn insert_response_unit(
    conn: &mut SqliteConnection,
    id: uuid::Uuid,
    name: impl AsRef<str>,
) -> Result<(), ExpressionError> {
    debug!(%id, name = %name.as_ref(), "Inserting response unit");
    let row = SqliteResponseUnit {
        id: id.to_string(),
        name: name.as_ref().to_string(),
    };
    diesel::insert_into(schema::response_units::table)
        .values(&row)
        .execute(conn)?;
    Ok(())
}
