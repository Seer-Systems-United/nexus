use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use tracing::instrument;

use crate::expr::ExpressionError;

use super::schema;

#[instrument(skip(conn))]
pub(super) fn source_ids_by_name(
    conn: &mut SqliteConnection,
    source_name: &str,
) -> Result<Vec<String>, ExpressionError> {
    schema::sources::table
        .filter(schema::sources::name.eq(source_name))
        .select(schema::sources::id)
        .load::<String>(conn)
        .map_err(ExpressionError::from)
}

#[instrument(skip(conn, source_ids))]
pub(super) fn source_names_by_ids(
    conn: &mut SqliteConnection,
    source_ids: &[uuid::Uuid],
) -> Result<Vec<String>, ExpressionError> {
    if source_ids.is_empty() {
        return Ok(Vec::new());
    }

    schema::sources::table
        .filter(
            schema::sources::id.eq_any(
                source_ids
                    .iter()
                    .map(uuid::Uuid::to_string)
                    .collect::<Vec<_>>(),
            ),
        )
        .select(schema::sources::name)
        .load::<String>(conn)
        .map_err(ExpressionError::from)
}
