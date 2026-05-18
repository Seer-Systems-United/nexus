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
        question::DatabaseQuestion,
    },
    expr::{
        ExpressionError,
        ops::{Filter, Table},
    },
};

use super::{
    polls::{poll_ids_for_source_ids, poll_ids_from_date, poll_ids_to_date},
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
                let pattern = format!("%{question}%");
                trace!(?pattern, "LIKE pattern for question");
                query = query.filter(
                    schema::questions::text
                        .eq(question)
                        .or(schema::questions::text.like(pattern.clone()))
                        .or(schema::questions::keywords.like(pattern)),
                );
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
