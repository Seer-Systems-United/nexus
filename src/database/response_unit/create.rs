use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error, instrument};

use crate::{
    database::{get_connection, response_unit::DatabaseResponseUnit},
    poll::response::unit::Unit,
    schema,
};

fn create_response_unit(name: String) -> DatabaseResponseUnit {
    DatabaseResponseUnit {
        id: uuid::Uuid::new_v4(),
        name,
    }
}

pub fn response_unit_name(unit: &Unit) -> String {
    match unit {
        Unit::Percent => "percent".to_string(),
        Unit::Count => "count".to_string(),
        Unit::Other(unit) => format!("other:{unit}"),
    }
}

#[instrument(level = "info", skip_all, fields(response_unit_name = %name))]
pub fn create_response_unit_by_name_in_db(
    name: &str,
) -> Result<DatabaseResponseUnit, diesel::result::Error> {
    debug!("creating response_unit");

    let mut conn = get_connection();

    match schema::response_units::table
        .filter(schema::response_units::name.eq(name))
        .select(DatabaseResponseUnit::as_select())
        .first::<DatabaseResponseUnit>(&mut conn)
        .optional()
    {
        Ok(Some(response_unit)) => {
            debug!(response_unit_id = %response_unit.id, "response_unit already exists");
            return Ok(response_unit);
        }
        Ok(None) => {}
        Err(error) => {
            error!(%error, "error checking for existing response_unit");
            return Err(error);
        }
    }

    let new_response_unit = create_response_unit(name.to_string());

    match diesel::insert_into(schema::response_units::table)
        .values(&new_response_unit)
        .returning(DatabaseResponseUnit::as_returning())
        .get_result(&mut conn)
    {
        Ok(response_unit) => {
            debug!(response_unit_id = %response_unit.id, "inserted response_unit");
            Ok(response_unit)
        }
        Err(error) => {
            error!(%error, response_unit_id = %new_response_unit.id, "error inserting response_unit");
            Err(error)
        }
    }
}

pub fn create_response_unit_in_db(
    unit: &Unit,
) -> Result<DatabaseResponseUnit, diesel::result::Error> {
    create_response_unit_by_name_in_db(&response_unit_name(unit))
}
