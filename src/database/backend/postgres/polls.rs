use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use tracing::{debug, instrument, trace};

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

use super::{connection::get_connection, rows::PollRow, schema, source::source_ids_by_name};

#[instrument(skip(filters))]
pub(super) fn get_polls(filters: &[Filter]) -> Result<Vec<DatabasePoll>, ExpressionError> {
    debug!(?filters, "executing Get(Polls) query");

    let mut conn = get_connection();
    let mut query = schema::polls::table.into_boxed();

    for filter in filters {
        match filter {
            Filter::PollSource { source_name } => {
                trace!(source_name = %source_name, "filtering by poll source");
                let source_ids = source_ids_by_name(&mut conn, source_name)?;

                if source_ids.is_empty() {
                    return Ok(Vec::new());
                }

                query = query.filter(schema::polls::source_id.eq_any(source_ids));
            }
            Filter::PollFrom { date } => {
                trace!(date = %date, "filtering polls from date");
                query =
                    query.filter(schema::polls::published_timestamp.ge(parse_date_start(date)?));
            }
            Filter::PollTo { date } => {
                trace!(date = %date, "filtering polls to date");
                query = query.filter(schema::polls::published_timestamp.le(parse_date_end(date)?));
            }
            filter => return invalid_filter(Table::Polls, filter),
        }
    }

    Ok(query
        .load::<PollRow>(&mut conn)?
        .into_iter()
        .map(DatabasePoll::from)
        .collect())
}

#[instrument]
pub(super) fn poll_exists_by_timestamp(timestamp: DateTime<Utc>) -> Result<bool, ExpressionError> {
    trace!(%timestamp, "checking if poll exists by timestamp");
    let mut conn = get_connection();
    let count = schema::polls::table
        .filter(schema::polls::published_timestamp.eq(timestamp.naive_utc()))
        .count()
        .get_result::<i64>(&mut conn)?;

    Ok(count > 0)
}

#[instrument(skip(conn))]
pub(super) fn poll_ids_for_source_ids(
    conn: &mut PgConnection,
    source_ids: Vec<uuid::Uuid>,
) -> Result<Vec<uuid::Uuid>, ExpressionError> {
    trace!(?source_ids, "fetching poll ids for source ids");
    if source_ids.is_empty() {
        return Ok(Vec::new());
    }

    schema::polls::table
        .filter(schema::polls::source_id.eq_any(source_ids))
        .select(schema::polls::id)
        .load::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}

#[instrument(skip(conn))]
pub(super) fn poll_ids_from_date(
    conn: &mut PgConnection,
    date: NaiveDateTime,
) -> Result<Vec<uuid::Uuid>, ExpressionError> {
    trace!(%date, "fetching poll ids from date");
    schema::polls::table
        .filter(schema::polls::published_timestamp.ge(date))
        .select(schema::polls::id)
        .load::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}

#[instrument(skip(conn))]
pub(super) fn poll_ids_to_date(
    conn: &mut PgConnection,
    date: NaiveDateTime,
) -> Result<Vec<uuid::Uuid>, ExpressionError> {
    trace!(%date, "fetching poll ids to date");
    schema::polls::table
        .filter(schema::polls::published_timestamp.le(date))
        .select(schema::polls::id)
        .load::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}
