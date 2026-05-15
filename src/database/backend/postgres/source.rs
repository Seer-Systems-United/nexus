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
