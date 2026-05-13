use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error};

use crate::{
    database::{get_connection, response_unit::DatabaseResponseUnit},
    schema,
};

pub fn get_response_unit_by_id(
    id: uuid::Uuid,
) -> Result<DatabaseResponseUnit, diesel::result::Error> {
    debug!(response_unit_id = %id, "getting response_unit by id");

    let mut conn = get_connection();

    match schema::response_units::table
        .find(id)
        .select(DatabaseResponseUnit::as_select())
        .first::<DatabaseResponseUnit>(&mut conn)
    {
        Ok(response_unit) => {
            debug!(response_unit_id = %id, "found response_unit");
            Ok(response_unit)
        }
        Err(error) => {
            error!(%error, response_unit_id = %id, "error finding response_unit by id");
            Err(error)
        }
    }
}
