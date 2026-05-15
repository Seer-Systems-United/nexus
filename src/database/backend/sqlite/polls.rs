use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::{
    database::{
        common::{
            date::{parse_date_end, parse_date_start},
            filter::invalid_filter,
        },
        poll::DatabasePoll,
    },
    expr::{
        ExpressionError,
        ops::{Filter, Table},
    },
};

use super::{rows::SqlitePoll, schema, source::source_ids_by_name, util::format_datetime};

pub(super) fn get_polls(
    conn: &mut SqliteConnection,
    filters: &[Filter],
) -> Result<Vec<DatabasePoll>, ExpressionError> {
    let mut query = schema::polls::table.into_boxed::<diesel::sqlite::Sqlite>();

    for filter in filters {
        match filter {
            Filter::PollSource { source_name } => {
                let source_ids = source_ids_by_name(conn, source_name)?;
                if source_ids.is_empty() {
                    return Ok(Vec::new());
                }
                query = query.filter(schema::polls::source_id.eq_any(source_ids));
            }
            Filter::PollFrom { date } => {
                query = query.filter(
                    schema::polls::published_timestamp.ge(format_datetime(parse_date_start(date)?)),
                );
            }
            Filter::PollTo { date } => {
                query = query.filter(
                    schema::polls::published_timestamp.le(format_datetime(parse_date_end(date)?)),
                );
            }
            filter => return invalid_filter(Table::Polls, filter),
        }
    }

    query
        .load::<SqlitePoll>(conn)?
        .into_iter()
        .map(DatabasePoll::try_from)
        .collect()
}

pub(super) fn poll_ids_for_source_ids(
    conn: &mut SqliteConnection,
    source_ids: Vec<String>,
) -> Result<Vec<String>, ExpressionError> {
    if source_ids.is_empty() {
        return Ok(Vec::new());
    }

    schema::polls::table
        .filter(schema::polls::source_id.eq_any(source_ids))
        .select(schema::polls::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from)
}

pub(super) fn poll_ids_from_date(
    conn: &mut SqliteConnection,
    date: NaiveDateTime,
) -> Result<Vec<String>, ExpressionError> {
    schema::polls::table
        .filter(schema::polls::published_timestamp.ge(format_datetime(date)))
        .select(schema::polls::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from)
}

pub(super) fn poll_ids_to_date(
    conn: &mut SqliteConnection,
    date: NaiveDateTime,
) -> Result<Vec<String>, ExpressionError> {
    schema::polls::table
        .filter(schema::polls::published_timestamp.le(format_datetime(date)))
        .select(schema::polls::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from)
}
