use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::{database::response_unit::DatabaseResponseUnit, expr::ExpressionError};

use super::{rows::ResponseUnitRow, schema};

pub(super) fn response_units_by_ids(
    conn: &mut PgConnection,
    unit_ids: &[uuid::Uuid],
) -> Result<Vec<DatabaseResponseUnit>, ExpressionError> {
    if unit_ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(schema::response_units::table
        .filter(schema::response_units::id.eq_any(unit_ids))
        .load::<ResponseUnitRow>(conn)?
        .into_iter()
        .map(DatabaseResponseUnit::from)
        .collect())
}
