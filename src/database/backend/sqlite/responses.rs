use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection,
    TextExpressionMethods,
};
use tracing::{debug, instrument, trace};

use crate::{
    database::{
        common::{
            date::{parse_date_end, parse_date_start},
            filter::invalid_filter,
        },
        response::DatabaseResponse,
    },
    expr::{
        ExpressionError,
        ops::{Filter, Table},
    },
};

use super::{
    polls::{poll_ids_for_source_ids, poll_ids_from_date, poll_ids_to_date},
    rows::SqliteResponse,
    schema,
    source::source_ids_by_name,
};

#[instrument(skip(conn, filters))]
pub(super) fn get_responses(
    conn: &mut SqliteConnection,
    filters: &[Filter],
) -> Result<Vec<DatabaseResponse>, ExpressionError> {
    let mut query = schema::responses::table.into_boxed::<diesel::sqlite::Sqlite>();

    for filter in filters {
        debug!(?filter, "Applying filter");
        match filter {
            Filter::ResponseSource { source_name } => {
                trace!(?source_name, "Filtering by source name");
                let source_ids = source_ids_by_name(conn, source_name)?;
                let poll_ids = poll_ids_for_source_ids(conn, source_ids)?;
                let question_ids = question_ids_for_poll_ids(conn, poll_ids)?;
                if question_ids.is_empty() {
                    debug!("No question IDs found for source name");
                    return Ok(Vec::new());
                }
                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseSourceId { source_id } => {
                trace!(?source_id, "Filtering by source ID");
                let poll_ids = poll_ids_for_source_ids(conn, vec![source_id.to_string()])?;
                let question_ids = question_ids_for_poll_ids(conn, poll_ids)?;
                if question_ids.is_empty() {
                    debug!("No question IDs found for source ID");
                    return Ok(Vec::new());
                }
                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseFrom { date } => {
                trace!(?date, "Filtering from date");
                let poll_ids = poll_ids_from_date(conn, parse_date_start(date)?)?;
                let question_ids = question_ids_for_poll_ids(conn, poll_ids)?;
                if question_ids.is_empty() {
                    debug!("No question IDs found for from date");
                    return Ok(Vec::new());
                }
                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseTo { date } => {
                trace!(?date, "Filtering to date");
                let poll_ids = poll_ids_to_date(conn, parse_date_end(date)?)?;
                let question_ids = question_ids_for_poll_ids(conn, poll_ids)?;
                if question_ids.is_empty() {
                    debug!("No question IDs found for to date");
                    return Ok(Vec::new());
                }
                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseQuestion { question } => {
                trace!(?question, "Filtering by question text");
                let question_ids = question_ids_for_search(conn, question)?;
                if question_ids.is_empty() {
                    debug!("No question IDs found for question text");
                    return Ok(Vec::new());
                }
                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseQuestionId { question_id } => {
                trace!(?question_id, "Filtering by question ID");
                query = query.filter(schema::responses::question_id.eq(question_id.to_string()));
            }
            Filter::ResponseDemographic { demographic_key } => {
                trace!(?demographic_key, "Filtering by demographic key");
                let demographic_ids = demographic_ids_by_key(conn, demographic_key)?;
                if demographic_ids.is_empty() {
                    debug!("No demographic IDs found for key");
                    return Ok(Vec::new());
                }
                query = query.filter(schema::responses::demographic_id.eq_any(demographic_ids));
            }
            filter => return invalid_filter(Table::Responses, filter),
        }
    }

    let responses = query.load::<SqliteResponse>(conn)?;
    debug!(count = responses.len(), "Loaded responses from database");
    responses
        .into_iter()
        .map(DatabaseResponse::try_from)
        .collect()
}

#[instrument(skip(conn, poll_ids))]
fn question_ids_for_poll_ids(
    conn: &mut SqliteConnection,
    poll_ids: Vec<String>,
) -> Result<Vec<String>, ExpressionError> {
    if poll_ids.is_empty() {
        debug!("No poll IDs provided");
        return Ok(Vec::new());
    }

    let ids = schema::questions::table
        .filter(schema::questions::poll_id.eq_any(poll_ids))
        .select(schema::questions::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from)?;
    debug!(count = ids.len(), "Loaded question IDs for poll IDs");
    Ok(ids)
}

#[instrument(skip(conn, question))]
fn question_ids_for_search(
    conn: &mut SqliteConnection,
    question: &str,
) -> Result<Vec<String>, ExpressionError> {
    let pattern = format!("%{question}%");

    let ids = schema::questions::table
        .filter(
            schema::questions::text
                .eq(question)
                .or(schema::questions::text.like(pattern.clone()))
                .or(schema::questions::keywords.like(pattern)),
        )
        .select(schema::questions::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from)?;
    debug!(count = ids.len(), "Loaded question IDs for search");
    Ok(ids)
}

#[instrument(skip(conn, demographic_key))]
fn demographic_ids_by_key(
    conn: &mut SqliteConnection,
    demographic_key: &str,
) -> Result<Vec<String>, ExpressionError> {
    let ids = schema::demographics::table
        .filter(schema::demographics::key.eq(demographic_key))
        .select(schema::demographics::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from)?;
    debug!(count = ids.len(), "Loaded demographic IDs for key");
    Ok(ids)
}
