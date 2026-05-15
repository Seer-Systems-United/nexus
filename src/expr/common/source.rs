use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{expr::ExpressionError, schema};

pub(crate) fn source_ids_by_name(
    source_name: &str,
    conn: &mut diesel::PgConnection,
) -> Result<Vec<uuid::Uuid>, ExpressionError> {
    schema::sources::table
        .filter(schema::sources::name.eq(source_name))
        .select(schema::sources::id)
        .load::<uuid::Uuid>(conn)
        .map_err(ExpressionError::from)
}
