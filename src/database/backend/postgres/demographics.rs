use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::{database::demographic::DatabaseDemographic, expr::ExpressionError};

use super::{rows::DemographicRow, schema};

pub(super) fn demographics_by_ids(
    conn: &mut PgConnection,
    demographic_ids: &[uuid::Uuid],
) -> Result<Vec<DatabaseDemographic>, ExpressionError> {
    if demographic_ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(schema::demographics::table
        .filter(schema::demographics::id.eq_any(demographic_ids))
        .load::<DemographicRow>(conn)?
        .into_iter()
        .map(DatabaseDemographic::from)
        .collect())
}
