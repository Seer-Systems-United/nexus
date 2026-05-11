use diesel::{QueryDsl, RunQueryDsl};
use tracing::{debug, error};

use crate::{
    database::{get_connection, ops::person::Person},
    schema::{self},
};

pub fn get_person_by_id(id: uuid::Uuid) -> Result<Person, diesel::result::Error> {
    debug!(person_id = %id, "getting person by id");

    let mut conn = get_connection();

    match schema::people::table.find(id).first::<Person>(&mut conn) {
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
