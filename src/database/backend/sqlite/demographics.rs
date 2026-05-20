use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use tracing::instrument;

use crate::{database::demographic::DatabaseDemographic, expr::ExpressionError};

use super::{rows::SqliteDemographic, schema};

#[instrument(skip(conn, demographic_ids))]
pub(super) fn demographics_by_ids(
    conn: &mut SqliteConnection,
    demographic_ids: &[uuid::Uuid],
) -> Result<Vec<DatabaseDemographic>, ExpressionError> {
    if demographic_ids.is_empty() {
        return Ok(Vec::new());
    }

    schema::demographics::table
        .filter(
            schema::demographics::id.eq_any(
                demographic_ids
                    .iter()
                    .map(uuid::Uuid::to_string)
                    .collect::<Vec<_>>(),
            ),
        )
        .load::<SqliteDemographic>(conn)?
        .into_iter()
        .map(DatabaseDemographic::try_from)
        .collect()
}
