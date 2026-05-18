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
