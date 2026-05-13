use diesel::{QueryDsl, RunQueryDsl};
use tracing::{debug, error};

use crate::{
    database::{get_connection, person::DatabasePerson},
    schema::{self},
};

pub fn get_person_by_id(id: uuid::Uuid) -> Result<DatabasePerson, diesel::result::Error> {
    debug!(person_id = %id, "getting person by id");

    let mut conn = get_connection();

    match schema::people::table
        .find(id)
        .first::<DatabasePerson>(&mut conn)
    {
        Ok(person) => {
            debug!(person_id = %id, "found person");
            Ok(person)
        }
        Err(e) => {
            error!(error = %e, person_id = %id, "error finding person by id");
            Err(e)
        }
    }
}
