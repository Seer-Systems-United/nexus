use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error};

use crate::{
    database::{demographic::DatabaseDemographic, get_connection},
    schema,
};

pub fn get_demographic_by_id(id: uuid::Uuid) -> Result<DatabaseDemographic, diesel::result::Error> {
    debug!(demographic_id = %id, "getting demographic by id");

    let mut conn = get_connection();

    match schema::demographics::table
        .find(id)
        .select(DatabaseDemographic::as_select())
        .first::<DatabaseDemographic>(&mut conn)
    {
        Ok(demographic) => {
            debug!(demographic_id = %id, "found demographic");
            Ok(demographic)
        }
        Err(error) => {
            error!(%error, demographic_id = %id, "error finding demographic by id");
            Err(error)
        }
    }
}
