use diesel::RunQueryDsl;
use tracing::{debug, error, instrument};

use crate::{
    database::{get_connection, ops::person::DatabasePerson},
    schema::{self},
};

fn create_person(
    given_name: String,
    surname: String,
    suffix: Option<String>,
    prefix: Option<String>,
) -> DatabasePerson {
    DatabasePerson {
        id: uuid::Uuid::new_v4(),
        given_name,
        surname,
        suffix,
        prefix,
    }
}

#[instrument(level = "info", skip_all, fields(given_name = %given_name, surname = %surname))]
pub fn create_person_in_db(
    given_name: String,
    surname: String,
    suffix: Option<String>,
    prefix: Option<String>,
) -> Result<DatabasePerson, diesel::result::Error> {
    debug!("creating person");

    let mut conn = get_connection();

    let new_person = create_person(given_name, surname, suffix, prefix);

    match diesel::insert_into(schema::people::table)
        .values(&new_person)
        .execute(&mut conn)
    {
        Ok(rows) => {
            debug!(rows, person_id = %new_person.id, "inserted person");
            Ok(new_person)
        }
        Err(e) => {
            error!(error = %e, "error inserting user");
            Err(e)
        }
    }
}
