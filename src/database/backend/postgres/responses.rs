use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use diesel_full_text_search::{TsVectorExtensions, plainto_tsquery};
use tracing::{debug, trace};

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
    connection::get_connection,
    polls::{poll_ids_for_source_ids, poll_ids_from_date, poll_ids_to_date},
    rows::ResponseRow,
    schema,
    source::source_ids_by_name,
};

pub(super) fn get_responses(filters: &[Filter]) -> Result<Vec<DatabaseResponse>, ExpressionError> {
    debug!(?filters, "executing Get(Responses) query");

    let mut conn = get_connection();
    let mut query = schema::responses::table.into_boxed();

    for filter in filters {
        match filter {
            Filter::ResponseSource { source_name } => {
                trace!(source_name = %source_name, "filtering responses by poll source");
                let source_ids = source_ids_by_name(&mut conn, source_name)?;
                let poll_ids = poll_ids_for_source_ids(&mut conn, source_ids)?;
                let question_ids = question_ids_for_poll_ids(&mut conn, poll_ids)?;

                if question_ids.is_empty() {
                    return Ok(Vec::new());
                }

                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseSourceId { source_id } => {
                trace!(source_id = %source_id, "filtering responses by poll source id");
                let poll_ids = poll_ids_for_source_ids(&mut conn, vec![*source_id])?;
                let question_ids = question_ids_for_poll_ids(&mut conn, poll_ids)?;

                if question_ids.is_empty() {
                    return Ok(Vec::new());
                }

                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseFrom { date } => {
                trace!(date = %date, "filtering responses from date");
                let poll_ids = poll_ids_from_date(&mut conn, parse_date_start(date)?)?;
                let question_ids = question_ids_for_poll_ids(&mut conn, poll_ids)?;

                if question_ids.is_empty() {
                    return Ok(Vec::new());
                }

                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseTo { date } => {
                trace!(date = %date, "filtering responses to date");
                let poll_ids = poll_ids_to_date(&mut conn, parse_date_end(date)?)?;
                let question_ids = question_ids_for_poll_ids(&mut conn, poll_ids)?;

                if question_ids.is_empty() {
                    return Ok(Vec::new());
                }

                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseQuestion { question } => {
                trace!(question = %question, "filtering responses by question");
                let question_ids = question_ids_for_search(&mut conn, question)?;

                if question_ids.is_empty() {
                    return Ok(Vec::new());
                }

                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseQuestionId { question_id } => {
                trace!(question_id = %question_id, "filtering responses by question id");
                query = query.filter(schema::responses::question_id.eq(question_id));
            }
            Filter::ResponseDemographic { demographic_key } => {
                trace!(demographic_key = %demographic_key, "filtering responses by demographic");
                let demographic_ids = demographic_ids_by_key(&mut conn, demographic_key)?;

                if demographic_ids.is_empty() {
                    return Ok(Vec::new());
                }

                query = query.filter(schema::responses::demographic_id.eq_any(demographic_ids));
            }
            filter => return invalid_filter(Table::Responses, filter),
        }
    }

    Ok(query
        .load::<ResponseRow>(&mut conn)?
        .into_iter()
        .map(DatabaseResponse::from)
        .collect())
}

fn question_ids_for_poll_ids(
    conn: &mut PgConnection,
    poll_ids: Vec<uuid::Uuid>,
) -> Result<Vec<uuid::Uuid>, ExpressionError> {
    if poll_ids.is_empty() {
        return Ok(Vec::new());
    }

    schema::questions::table
        .filter(schema::questions::poll_id.eq_any(poll_ids))
        .select(schema::questions::id)
        .load::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}

fn question_ids_for_search(
    conn: &mut PgConnection,
    question: &str,
) -> Result<Vec<uuid::Uuid>, ExpressionError> {
    schema::questions::table
        .filter(
            schema::questions::text
                .eq(question)
                .or(schema::questions::keywords.matches(plainto_tsquery(question))),
        )
        .select(schema::questions::id)
        .load::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}

fn demographic_ids_by_key(
    conn: &mut PgConnection,
    demographic_key: &str,
) -> Result<Vec<uuid::Uuid>, ExpressionError> {
    schema::demographics::table
        .filter(schema::demographics::key.eq(demographic_key))
        .select(schema::demographics::id)
        .load::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}
