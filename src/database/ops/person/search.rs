use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use tracing::{debug, error};

use crate::{
    database::{get_connection, ops::person::DatabasePerson},
    schema::{self},
};

pub fn search_people_by_surname(
    surname: &str,
) -> Result<Vec<DatabasePerson>, diesel::result::Error> {
    debug!(surname = %surname, "searching people by surname");

    let mut conn = get_connection();

    match schema::people::table
        .filter(schema::people::surname.eq(surname))
        .load::<DatabasePerson>(&mut conn)
    {
        Ok(people) => {
            debug!(count = people.len(), surname = %surname, "found people by surname");
            Ok(people)
        }
        Err(e) => {
            error!(error = %e, surname = %surname, "error searching people by surname");
            Err(e)
        }
    }
}

pub fn search_people_by_given_name(
    given_name: &str,
) -> Result<Vec<DatabasePerson>, diesel::result::Error> {
    debug!(given_name = %given_name, "searching people by given name");

    let mut conn = get_connection();

    match schema::people::table
        .filter(schema::people::given_name.eq(given_name))
        .load::<DatabasePerson>(&mut conn)
    {
        Ok(people) => {
            debug!(count = people.len(), given_name = %given_name, "found people by given name");
            Ok(people)
        }
        Err(e) => {
            error!(error = %e, given_name = %given_name, "error searching people by given name");
            Err(e)
        }
    }
}

pub fn search_people_by_full_name(
    given_name: &str,
    surname: &str,
) -> Result<Vec<DatabasePerson>, diesel::result::Error> {
    debug!(
        given_name = %given_name,
        surname = %surname,
        "searching people by full name"
    );

    let mut conn = get_connection();

    match schema::people::table
        .filter(
            schema::people::given_name
                .eq(given_name)
                .and(schema::people::surname.eq(surname)),
        )
        .load::<DatabasePerson>(&mut conn)
    {
        Ok(people) => {
            debug!(
                count = people.len(),
                given_name = %given_name,
                surname = %surname,
                "found people by full name"
            );
            Ok(people)
        }
        Err(e) => {
            error!(
                error = %e,
                given_name = %given_name,
                surname = %surname,
                "error searching people by full name"
            );
            Err(e)
        }
    }
}
