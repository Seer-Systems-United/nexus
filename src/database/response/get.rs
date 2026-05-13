use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error};

use crate::{
    database::{get_connection, response::DatabaseResponse},
    schema,
};

pub fn get_response_by_id(id: uuid::Uuid) -> Result<DatabaseResponse, diesel::result::Error> {
    debug!(response_id = %id, "getting response by id");

    let mut conn = get_connection();

    match schema::responses::table
        .find(id)
        .select(DatabaseResponse::as_select())
        .first::<DatabaseResponse>(&mut conn)
    {
        Ok(response) => {
            debug!(response_id = %id, "found response");
            Ok(response)
        }
        Err(error) => {
            error!(%error, response_id = %id, "error finding response by id");
            Err(error)
        }
    }
}
