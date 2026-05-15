use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection,
    TextExpressionMethods,
};

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

pub(super) fn get_responses(
    conn: &mut SqliteConnection,
    filters: &[Filter],
) -> Result<Vec<DatabaseResponse>, ExpressionError> {
    let mut query = schema::responses::table.into_boxed::<diesel::sqlite::Sqlite>();

    for filter in filters {
        match filter {
            Filter::ResponseSource { source_name } => {
                let source_ids = source_ids_by_name(conn, source_name)?;
                let poll_ids = poll_ids_for_source_ids(conn, source_ids)?;
                let question_ids = question_ids_for_poll_ids(conn, poll_ids)?;
                if question_ids.is_empty() {
                    return Ok(Vec::new());
                }
                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseFrom { date } => {
                let poll_ids = poll_ids_from_date(conn, parse_date_start(date)?)?;
                let question_ids = question_ids_for_poll_ids(conn, poll_ids)?;
                if question_ids.is_empty() {
                    return Ok(Vec::new());
                }
                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseTo { date } => {
                let poll_ids = poll_ids_to_date(conn, parse_date_end(date)?)?;
                let question_ids = question_ids_for_poll_ids(conn, poll_ids)?;
                if question_ids.is_empty() {
                    return Ok(Vec::new());
                }
                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseQuestion { question } => {
                let question_ids = question_ids_for_search(conn, question)?;
                if question_ids.is_empty() {
                    return Ok(Vec::new());
                }
                query = query.filter(schema::responses::question_id.eq_any(question_ids));
            }
            Filter::ResponseDemographic { demographic_key } => {
                let demographic_ids = demographic_ids_by_key(conn, demographic_key)?;
                if demographic_ids.is_empty() {
                    return Ok(Vec::new());
                }
                query = query.filter(schema::responses::demographic_id.eq_any(demographic_ids));
            }
            filter => return invalid_filter(Table::Responses, filter),
        }
    }

    query
        .load::<SqliteResponse>(conn)?
        .into_iter()
        .map(DatabaseResponse::try_from)
        .collect()
}

fn question_ids_for_poll_ids(
    conn: &mut SqliteConnection,
    poll_ids: Vec<String>,
) -> Result<Vec<String>, ExpressionError> {
    if poll_ids.is_empty() {
        return Ok(Vec::new());
    }

    schema::questions::table
        .filter(schema::questions::poll_id.eq_any(poll_ids))
        .select(schema::questions::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from)
}

fn question_ids_for_search(
    conn: &mut SqliteConnection,
    question: &str,
) -> Result<Vec<String>, ExpressionError> {
    let pattern = format!("%{question}%");

    schema::questions::table
        .filter(
            schema::questions::text
                .eq(question)
                .or(schema::questions::text.like(pattern.clone()))
                .or(schema::questions::keywords.like(pattern)),
        )
        .select(schema::questions::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from)
}

fn demographic_ids_by_key(
    conn: &mut SqliteConnection,
    demographic_key: &str,
) -> Result<Vec<String>, ExpressionError> {
    schema::demographics::table
        .filter(schema::demographics::key.eq(demographic_key))
        .select(schema::demographics::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from)
}
