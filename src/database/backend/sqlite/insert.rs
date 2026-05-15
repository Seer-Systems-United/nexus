use diesel::{RunQueryDsl, SqliteConnection};

use crate::{
    database::{person::DatabasePerson, poll::DatabasePoll, response::DatabaseResponse},
    expr::ExpressionError,
};

use super::{
    rows::{
        SqliteDemographic, SqlitePerson, SqlitePoll, SqliteQuestion, SqliteResponse,
        SqliteResponseUnit, SqliteSource,
    },
    schema,
};

pub(super) fn insert_person(
    conn: &mut SqliteConnection,
    person: &DatabasePerson,
) -> Result<(), ExpressionError> {
    let row = SqlitePerson::from(person);
    diesel::insert_into(schema::people::table)
        .values(&row)
        .execute(conn)?;
    Ok(())
}

pub(super) fn insert_poll(
    conn: &mut SqliteConnection,
    poll: &DatabasePoll,
) -> Result<(), ExpressionError> {
    let row = SqlitePoll::from(poll);
    diesel::insert_into(schema::polls::table)
        .values(&row)
        .execute(conn)?;
    Ok(())
}

pub(super) fn insert_response(
    conn: &mut SqliteConnection,
    response: &DatabaseResponse,
) -> Result<(), ExpressionError> {
    let row = SqliteResponse::from(response);
    diesel::insert_into(schema::responses::table)
        .values(&row)
        .execute(conn)?;
    Ok(())
}

pub(super) fn insert_source(
    conn: &mut SqliteConnection,
    id: uuid::Uuid,
    name: impl AsRef<str>,
) -> Result<(), ExpressionError> {
    let row = SqliteSource {
        id: id.to_string(),
        name: name.as_ref().to_string(),
    };
    diesel::insert_into(schema::sources::table)
        .values(&row)
        .execute(conn)?;
    Ok(())
}

pub(super) fn insert_question(
    conn: &mut SqliteConnection,
    id: uuid::Uuid,
    poll_id: uuid::Uuid,
    text: impl AsRef<str>,
) -> Result<(), ExpressionError> {
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
    Ok(())
}

pub(super) fn insert_demographic(
    conn: &mut SqliteConnection,
    id: uuid::Uuid,
    key: impl AsRef<str>,
    demographic_type: impl AsRef<str>,
) -> Result<(), ExpressionError> {
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

pub(super) fn insert_response_unit(
    conn: &mut SqliteConnection,
    id: uuid::Uuid,
    name: impl AsRef<str>,
) -> Result<(), ExpressionError> {
    let row = SqliteResponseUnit {
        id: id.to_string(),
        name: name.as_ref().to_string(),
    };
    diesel::insert_into(schema::response_units::table)
        .values(&row)
        .execute(conn)?;
    Ok(())
}
