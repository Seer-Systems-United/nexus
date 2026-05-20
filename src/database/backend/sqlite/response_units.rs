use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use tracing::instrument;

use crate::{database::response_unit::DatabaseResponseUnit, expr::ExpressionError};

use super::{rows::SqliteResponseUnit, schema};

#[instrument(skip(conn, unit_ids))]
pub(super) fn response_units_by_ids(
    conn: &mut SqliteConnection,
    unit_ids: &[uuid::Uuid],
) -> Result<Vec<DatabaseResponseUnit>, ExpressionError> {
    if unit_ids.is_empty() {
        return Ok(Vec::new());
    }

    schema::response_units::table
        .filter(
            schema::response_units::id.eq_any(
                unit_ids
                    .iter()
                    .map(uuid::Uuid::to_string)
                    .collect::<Vec<_>>(),
            ),
        )
        .load::<SqliteResponseUnit>(conn)?
        .into_iter()
        .map(DatabaseResponseUnit::try_from)
        .collect()
}
