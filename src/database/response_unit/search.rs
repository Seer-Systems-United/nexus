use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error};

use crate::{
    database::{get_connection, response_unit::DatabaseResponseUnit},
    schema,
};

pub fn search_response_units_by_name(
    name: &str,
) -> Result<Vec<DatabaseResponseUnit>, diesel::result::Error> {
    debug!(response_unit_name = %name, "searching response_units by name");

    let mut conn = get_connection();

    match schema::response_units::table
        .filter(schema::response_units::name.eq(name))
        .select(DatabaseResponseUnit::as_select())
        .load::<DatabaseResponseUnit>(&mut conn)
    {
        Ok(response_units) => {
            debug!(count = response_units.len(), response_unit_name = %name, "found response_units by name");
            Ok(response_units)
        }
        Err(error) => {
            error!(%error, response_unit_name = %name, "error searching response_units by name");
            Err(error)
        }
    }
}
