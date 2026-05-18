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

pub(super) fn get_questions(
    conn: &mut SqliteConnection,
    filters: &[Filter],
) -> Result<Vec<DatabaseQuestion>, ExpressionError> {
    let mut query = schema::questions::table.into_boxed::<diesel::sqlite::Sqlite>();

    for filter in filters {
        match filter {
            Filter::QuestionSource { source_name } => {
                let source_ids = source_ids_by_name(conn, source_name)?;
                let poll_ids = poll_ids_for_source_ids(conn, source_ids)?;
                if poll_ids.is_empty() {
                    return Ok(Vec::new());
                }
                query = query.filter(schema::questions::poll_id.eq_any(poll_ids));
            }
            Filter::QuestionFrom { date } => {
                let poll_ids = poll_ids_from_date(conn, parse_date_start(date)?)?;
                if poll_ids.is_empty() {
                    return Ok(Vec::new());
                }
                query = query.filter(schema::questions::poll_id.eq_any(poll_ids));
            }
            Filter::QuestionTo { date } => {
                let poll_ids = poll_ids_to_date(conn, parse_date_end(date)?)?;
                if poll_ids.is_empty() {
                    return Ok(Vec::new());
                }
                query = query.filter(schema::questions::poll_id.eq_any(poll_ids));
            }
            Filter::QuestionQuestion { question } => {
                let pattern = format!("%{question}%");
                query = query.filter(
                    schema::questions::text
                        .eq(question)
                        .or(schema::questions::text.like(pattern.clone()))
                        .or(schema::questions::keywords.like(pattern)),
                );
            }
            filter => return invalid_filter(Table::Questions, filter),
        }
    }

    query
        .load::<SqliteQuestion>(conn)?
        .into_iter()
        .map(DatabaseQuestion::try_from)
        .collect()
}
