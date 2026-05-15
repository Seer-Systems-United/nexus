use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{expr::ExpressionError, schema};

pub(crate) fn poll_ids_for_source_ids(
    source_ids: Vec<uuid::Uuid>,
    conn: &mut diesel::PgConnection,
) -> Result<Vec<uuid::Uuid>, ExpressionError> {
    if source_ids.is_empty() {
        return Ok(Vec::new());
    }

    schema::polls::table
        .filter(schema::polls::source_id.eq_any(source_ids))
        .select(schema::polls::id)
        .load::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}

pub(crate) fn poll_ids_from_date(
    date: NaiveDateTime,
    conn: &mut diesel::PgConnection,
) -> Result<Vec<uuid::Uuid>, ExpressionError> {
    schema::polls::table
        .filter(schema::polls::published_timestamp.ge(date))
        .select(schema::polls::id)
        .load::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}

pub(crate) fn poll_ids_to_date(
    date: NaiveDateTime,
    conn: &mut diesel::PgConnection,
) -> Result<Vec<uuid::Uuid>, ExpressionError> {
    schema::polls::table
        .filter(schema::polls::published_timestamp.le(date))
        .select(schema::polls::id)
        .load::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}
