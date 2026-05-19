use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use tracing::{debug, instrument, trace};

use crate::{
    database::{
        common::{
            date::{parse_date_end, parse_date_start},
            filter::invalid_filter,
        },
        question::DatabaseQuestion,
    },
    expr::{
        ExpressionError,
        ops::{Filter, Table},
    },
};

use super::{
    polls::{poll_ids_for_source_ids, poll_ids_from_date, poll_ids_to_date},
    question_search::question_ids_for_search,
    rows::SqliteQuestion,
    schema,
    source::source_ids_by_name,
};

#[instrument(level = "trace", skip(conn))]
pub(super) fn get_questions(
    conn: &mut SqliteConnection,
    filters: &[Filter],
) -> Result<Vec<DatabaseQuestion>, ExpressionError> {
    trace!(?filters, "Starting get_questions with filters");
    let mut query = schema::questions::table.into_boxed::<diesel::sqlite::Sqlite>();

    for filter in filters {
        match filter {
            Filter::QuestionId { question_id } => {
                debug!(?question_id, "Filtering by QuestionId");
                query = query.filter(schema::questions::id.eq(question_id.to_string()));
            }
            Filter::QuestionIds { question_ids } => {
                debug!(count = question_ids.len(), "Filtering by QuestionIds");
                if question_ids.is_empty() {
                    debug!("No question IDs provided, returning empty vector");
                    return Ok(Vec::new());
                }
                query = query.filter(
                    schema::questions::id.eq_any(
                        question_ids
                            .iter()
                            .map(uuid::Uuid::to_string)
                            .collect::<Vec<_>>(),
                    ),
                );
            }
            Filter::QuestionPollId { poll_id } => {
                debug!(?poll_id, "Filtering by QuestionPollId");
                query = query.filter(schema::questions::poll_id.eq(poll_id.to_string()));
            }
            Filter::QuestionPollIds { poll_ids } => {
                debug!(count = poll_ids.len(), "Filtering by QuestionPollIds");
                if poll_ids.is_empty() {
                    debug!("No poll IDs provided, returning empty vector");
                    return Ok(Vec::new());
                }
                query = query.filter(
                    schema::questions::poll_id.eq_any(
                        poll_ids
                            .iter()
                            .map(uuid::Uuid::to_string)
                            .collect::<Vec<_>>(),
                    ),
                );
            }
            Filter::QuestionSource { source_name } => {
                debug!(?source_name, "Filtering by QuestionSource");
                let source_ids = source_ids_by_name(conn, source_name)?;
                trace!(?source_ids, "Source IDs found");
                let poll_ids = poll_ids_for_source_ids(conn, source_ids)?;
                trace!(?poll_ids, "Poll IDs for source");
                if poll_ids.is_empty() {
                    debug!("No poll IDs found for source, returning empty vector");
                    return Ok(Vec::new());
                }
                query = query.filter(schema::questions::poll_id.eq_any(poll_ids));
            }
            Filter::QuestionSourceId { source_id } => {
                debug!(?source_id, "Filtering by QuestionSourceId");
                let poll_ids = poll_ids_for_source_ids(conn, vec![source_id.to_string()])?;
                trace!(?poll_ids, "Poll IDs for source_id");
                if poll_ids.is_empty() {
                    debug!("No poll IDs found for source_id, returning empty vector");
                    return Ok(Vec::new());
                }
                query = query.filter(schema::questions::poll_id.eq_any(poll_ids));
            }
            Filter::QuestionFrom { date } => {
                debug!(?date, "Filtering by QuestionFrom");
                let parsed_date = parse_date_start(date)?;
                trace!(?parsed_date, "Parsed start date");
                let poll_ids = poll_ids_from_date(conn, parsed_date)?;
                trace!(?poll_ids, "Poll IDs from date");
                if poll_ids.is_empty() {
                    debug!("No poll IDs found from date, returning empty vector");
                    return Ok(Vec::new());
                }
                query = query.filter(schema::questions::poll_id.eq_any(poll_ids));
            }
            Filter::QuestionTo { date } => {
                debug!(?date, "Filtering by QuestionTo");
                let parsed_date = parse_date_end(date)?;
                trace!(?parsed_date, "Parsed end date");
                let poll_ids = poll_ids_to_date(conn, parsed_date)?;
                trace!(?poll_ids, "Poll IDs to date");
                if poll_ids.is_empty() {
                    debug!("No poll IDs found to date, returning empty vector");
                    return Ok(Vec::new());
                }
                query = query.filter(schema::questions::poll_id.eq_any(poll_ids));
            }
            Filter::QuestionQuestion { question } => {
                debug!(?question, "Filtering by QuestionQuestion");
                let question_ids = question_ids_for_search(conn, question)?;
                if question_ids.is_empty() {
                    debug!("No question IDs found for full-text search");
                    return Ok(Vec::new());
                }
                query = query.filter(schema::questions::id.eq_any(question_ids));
            }
            filter => {
                debug!(?filter, "Invalid filter for Questions");
                return invalid_filter(Table::Questions, filter);
            }
        }
    }

    let results = query
        .load::<SqliteQuestion>(conn)?
        .into_iter()
        .map(DatabaseQuestion::try_from)
        .collect();
    trace!("Loaded questions from database");
    results
}
