use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::expr::ExpressionError;

use super::schema;

pub(super) fn source_ids_by_name(
    conn: &mut PgConnection,
    source_name: &str,
) -> Result<Vec<uuid::Uuid>, ExpressionError> {
    schema::sources::table
        .filter(schema::sources::name.eq(source_name))
        .select(schema::sources::id)
        .load::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}

pub(super) fn source_names_by_ids(
    conn: &mut PgConnection,
    source_ids: &[uuid::Uuid],
) -> Result<Vec<String>, ExpressionError> {
    if source_ids.is_empty() {
        return Ok(Vec::new());
    }

    schema::sources::table
        .filter(schema::sources::id.eq_any(source_ids))
        .select(schema::sources::name)
        .load::<String>(conn)
        .map_err(ExpressionError::from)
}
