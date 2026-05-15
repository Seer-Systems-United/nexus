use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, sql_types::Text};
use diesel_full_text_search::to_tsvector;
use tracing::{debug, trace};

use crate::{
    database::common::domain::{demographic_record, unit_name},
    expr::ExpressionError,
    poll::{Poll, response::Response},
};

use super::{connection::get_connection, schema};

pub(super) fn save_poll(source_name: &str, poll: &Poll) -> Result<(), ExpressionError> {
    debug!(
        source_name,
        questions_len = poll.questions.len(),
        published_timestamp = %poll.published_timestamp,
        "saving poll"
    );

    let mut conn = get_connection();
    conn.transaction(|conn| {
        let source_id = source_id(conn, source_name)?;
        let poll_id = poll_id(conn, source_id, poll)?;

        for question in &poll.questions {
            let question_id = question_id(conn, poll_id, &question.text)?;

            for response in &question.responses {
                save_response(conn, question_id, response)?;
            }
        }

        Ok(())
    })
}

fn source_id(conn: &mut PgConnection, source_name: &str) -> Result<uuid::Uuid, ExpressionError> {
    let id = uuid::Uuid::new_v4();

    diesel::insert_into(schema::sources::table)
        .values((
            schema::sources::id.eq(id),
            schema::sources::name.eq(source_name),
        ))
        .on_conflict(schema::sources::name)
        .do_nothing()
        .execute(conn)?;

    schema::sources::table
        .filter(schema::sources::name.eq(source_name))
        .select(schema::sources::id)
        .first::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}

fn poll_id(
    conn: &mut PgConnection,
    source_id: uuid::Uuid,
    poll: &Poll,
) -> Result<uuid::Uuid, ExpressionError> {
    let published_timestamp = poll.published_timestamp.naive_utc();
    let id = uuid::Uuid::new_v4();

    diesel::insert_into(schema::polls::table)
        .values((
            schema::polls::id.eq(id),
            schema::polls::source_id.eq(source_id),
            schema::polls::published_timestamp.eq(published_timestamp),
        ))
        .on_conflict((schema::polls::source_id, schema::polls::published_timestamp))
        .do_nothing()
        .execute(conn)?;

    schema::polls::table
        .filter(schema::polls::source_id.eq(source_id))
        .filter(schema::polls::published_timestamp.eq(published_timestamp))
        .select(schema::polls::id)
        .first::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}

fn question_id(
    conn: &mut PgConnection,
    poll_id: uuid::Uuid,
    text: &str,
) -> Result<uuid::Uuid, ExpressionError> {
    let id = uuid::Uuid::new_v4();

    diesel::insert_into(schema::questions::table)
        .values((
            schema::questions::id.eq(id),
            schema::questions::poll_id.eq(poll_id),
            schema::questions::text.eq(text),
            schema::questions::keywords.eq(to_tsvector::<Text, _>(text)),
        ))
        .on_conflict((schema::questions::poll_id, schema::questions::text))
        .do_nothing()
        .execute(conn)?;

    schema::questions::table
        .filter(schema::questions::poll_id.eq(poll_id))
        .filter(schema::questions::text.eq(text))
        .select(schema::questions::id)
        .first::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}

fn save_response(
    conn: &mut PgConnection,
    question_id: uuid::Uuid,
    response: &Response,
) -> Result<(), ExpressionError> {
    let demographic_id = demographic_id(conn, &response.demographic)?;
    let unit_id = unit_id(conn, &response.unit)?;
    let id = uuid::Uuid::new_v4();
    let answer = response.answer.as_ref();
    let value = i32::from(response.value);

    trace!(question_id = %question_id, answer, value, "saving response");

    diesel::insert_into(schema::responses::table)
        .values((
            schema::responses::id.eq(id),
            schema::responses::question_id.eq(question_id),
            schema::responses::demographic_id.eq(demographic_id),
            schema::responses::unit_id.eq(unit_id),
            schema::responses::answer.eq(answer),
            schema::responses::value.eq(value),
        ))
        .on_conflict((
            schema::responses::question_id,
            schema::responses::demographic_id,
            schema::responses::unit_id,
            schema::responses::answer,
            schema::responses::value,
        ))
        .do_nothing()
        .execute(conn)?;

    Ok(())
}

fn demographic_id(
    conn: &mut PgConnection,
    demographic: &crate::poll::response::demographic::Demographic,
) -> Result<uuid::Uuid, ExpressionError> {
    let record = demographic_record(demographic);
    let id = uuid::Uuid::new_v4();

    diesel::insert_into(schema::demographics::table)
        .values((
            schema::demographics::id.eq(id),
            schema::demographics::key.eq(record.key.as_str()),
            schema::demographics::demographic_type.eq(record.demographic_type),
            schema::demographics::label.eq(record.label.as_deref()),
            schema::demographics::lower_bound.eq(record.lower_bound),
            schema::demographics::upper_bound.eq(record.upper_bound),
            schema::demographics::registered.eq(record.registered),
        ))
        .on_conflict(schema::demographics::key)
        .do_nothing()
        .execute(conn)?;

    schema::demographics::table
        .filter(schema::demographics::key.eq(record.key))
        .select(schema::demographics::id)
        .first::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}

fn unit_id(
    conn: &mut PgConnection,
    unit: &crate::poll::response::unit::Unit,
) -> Result<uuid::Uuid, ExpressionError> {
    let name = unit_name(unit);
    let id = uuid::Uuid::new_v4();

    diesel::insert_into(schema::response_units::table)
        .values((
            schema::response_units::id.eq(id),
            schema::response_units::name.eq(name.as_str()),
        ))
        .on_conflict(schema::response_units::name)
        .do_nothing()
        .execute(conn)?;

    schema::response_units::table
        .filter(schema::response_units::name.eq(name))
        .select(schema::response_units::id)
        .first::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}
