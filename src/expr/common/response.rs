use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    expr::{ExpressionError, common::query::ResponsesQuery, traits::FilterApplication},
    schema,
};

pub(crate) fn apply_question_ids_filter<'a>(
    query: ResponsesQuery<'a>,
    question_ids: Vec<uuid::Uuid>,
) -> FilterApplication<ResponsesQuery<'a>> {
    if question_ids.is_empty() {
        return FilterApplication::Empty;
    }

    FilterApplication::Applied(query.filter(schema::responses::question_id.eq_any(question_ids)))
}

pub(crate) fn apply_poll_ids_filter<'a>(
    query: ResponsesQuery<'a>,
    poll_ids: Vec<uuid::Uuid>,
    conn: &mut diesel::PgConnection,
) -> Result<FilterApplication<ResponsesQuery<'a>>, ExpressionError> {
    if poll_ids.is_empty() {
        return Ok(FilterApplication::Empty);
    }

    let question_ids = schema::questions::table
        .filter(schema::questions::poll_id.eq_any(poll_ids))
        .select(schema::questions::id)
        .load::<uuid::Uuid>(conn)?;

    Ok(apply_question_ids_filter(query, question_ids))
}
