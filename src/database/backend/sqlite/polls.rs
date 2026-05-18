use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
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

use super::{rows::SqlitePoll, schema, source::source_ids_by_name, util::format_datetime};

#[instrument(level = "trace", skip(conn, filters))]
pub(super) fn get_polls(
    conn: &mut SqliteConnection,
    filters: &[Filter],
) -> Result<Vec<DatabasePoll>, ExpressionError> {
    trace!(?filters, "Starting get_polls with filters");
    let mut query = schema::polls::table.into_boxed::<diesel::sqlite::Sqlite>();

    for filter in filters {
        match filter {
            Filter::PollSource { source_name } => {
                debug!(?source_name, "Filtering polls by source name");
                let source_ids = source_ids_by_name(conn, source_name)?;
                if source_ids.is_empty() {
                    debug!(?source_name, "No source ids found for source name");
                    return Ok(Vec::new());
                }
                query = query.filter(schema::polls::source_id.eq_any(source_ids));
            }
            Filter::PollFrom { date } => {
                let dt = parse_date_start(date)?;
                debug!(?date, ?dt, "Filtering polls from date");
                query = query.filter(schema::polls::published_timestamp.ge(format_datetime(dt)));
            }
            Filter::PollTo { date } => {
                let dt = parse_date_end(date)?;
                debug!(?date, ?dt, "Filtering polls to date");
                query = query.filter(schema::polls::published_timestamp.le(format_datetime(dt)));
            }
            filter => {
                debug!(?filter, "Invalid filter for polls");
                return invalid_filter(Table::Polls, filter);
            }
        }
    }

    let result = query
        .load::<SqlitePoll>(conn)?
        .into_iter()
        .map(DatabasePoll::try_from)
        .collect();
    trace!("Finished get_polls");
    result
}

#[instrument(level = "trace", skip(conn, source_ids))]
pub(super) fn poll_ids_for_source_ids(
    conn: &mut SqliteConnection,
    source_ids: Vec<String>,
) -> Result<Vec<String>, ExpressionError> {
    trace!(?source_ids, "Getting poll ids for source ids");
    if source_ids.is_empty() {
        debug!("No source ids provided");
        return Ok(Vec::new());
    }

    let result = schema::polls::table
        .filter(schema::polls::source_id.eq_any(source_ids))
        .select(schema::polls::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from);
    trace!("Finished poll_ids_for_source_ids");
    result
}

#[instrument(level = "trace", skip(conn))]
pub(super) fn poll_ids_from_date(
    conn: &mut SqliteConnection,
    date: NaiveDateTime,
) -> Result<Vec<String>, ExpressionError> {
    debug!(?date, "Getting poll ids from date");
    let result = schema::polls::table
        .filter(schema::polls::published_timestamp.ge(format_datetime(date)))
        .select(schema::polls::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from);
    trace!("Finished poll_ids_from_date");
    result
}

#[instrument(level = "trace", skip(conn))]
pub(super) fn poll_ids_to_date(
    conn: &mut SqliteConnection,
    date: NaiveDateTime,
) -> Result<Vec<String>, ExpressionError> {
    debug!(?date, "Getting poll ids to date");
    let result = schema::polls::table
        .filter(schema::polls::published_timestamp.le(format_datetime(date)))
        .select(schema::polls::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from);
    trace!("Finished poll_ids_to_date");
    result
}
