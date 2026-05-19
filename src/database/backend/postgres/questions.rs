use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, dsl::sql, sql_types::Text,
};
use diesel_full_text_search::{TsVectorExtensions, plainto_tsquery};
use tracing::{debug, trace};

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
    connection::get_connection,
    polls::{poll_ids_for_source_ids, poll_ids_from_date, poll_ids_to_date},
    schema,
    source::source_ids_by_name,
};

pub(super) fn get_questions(filters: &[Filter]) -> Result<Vec<DatabaseQuestion>, ExpressionError> {
    debug!(?filters, "executing Get(Questions) query");

    let mut conn = get_connection();
    let mut query = schema::questions::table.into_boxed();

    for filter in filters {
        match filter {
            Filter::QuestionId { question_id } => {
                trace!(question_id = %question_id, "filtering questions by id");
                query = query.filter(schema::questions::id.eq(question_id));
            }
            Filter::QuestionIds { question_ids } => {
                trace!(count = question_ids.len(), "filtering questions by ids");
                if question_ids.is_empty() {
                    return Ok(Vec::new());
                }
                query = query.filter(schema::questions::id.eq_any(question_ids));
            }
            Filter::QuestionSource { source_name } => {
                trace!(source_name = %source_name, "filtering questions by poll source");
                let source_ids = source_ids_by_name(&mut conn, source_name)?;
                let poll_ids = poll_ids_for_source_ids(&mut conn, source_ids)?;

                if poll_ids.is_empty() {
                    return Ok(Vec::new());
                }

                query = query.filter(schema::questions::poll_id.eq_any(poll_ids));
            }
            Filter::QuestionSourceId { source_id } => {
                trace!(source_id = %source_id, "filtering questions by poll source id");
                let poll_ids = poll_ids_for_source_ids(&mut conn, vec![*source_id])?;

                if poll_ids.is_empty() {
                    return Ok(Vec::new());
                }

                query = query.filter(schema::questions::poll_id.eq_any(poll_ids));
            }
            Filter::QuestionFrom { date } => {
                trace!(date = %date, "filtering questions from date");
                let poll_ids = poll_ids_from_date(&mut conn, parse_date_start(date)?)?;

                if poll_ids.is_empty() {
                    return Ok(Vec::new());
                }

                query = query.filter(schema::questions::poll_id.eq_any(poll_ids));
            }
            Filter::QuestionTo { date } => {
                trace!(date = %date, "filtering questions to date");
                let poll_ids = poll_ids_to_date(&mut conn, parse_date_end(date)?)?;

                if poll_ids.is_empty() {
                    return Ok(Vec::new());
                }

                query = query.filter(schema::questions::poll_id.eq_any(poll_ids));
            }
            Filter::QuestionQuestion { question } => {
                trace!(question = %question, "filtering questions by question");
                query = query.filter(
                    schema::questions::text
                        .eq(question)
                        .or(schema::questions::keywords.matches(plainto_tsquery(question))),
                );
            }
            filter => return invalid_filter(Table::Questions, filter),
        }
    }

    Ok(query
        .select((
            schema::questions::id,
            schema::questions::poll_id,
            schema::questions::text,
            sql::<Text>("keywords::text"),
        ))
        .load::<(uuid::Uuid, uuid::Uuid, String, String)>(&mut conn)?
        .into_iter()
        .map(|(id, poll_id, text, keywords)| DatabaseQuestion {
            id,
            poll_id,
            text,
            keywords,
        })
        .collect())
}
