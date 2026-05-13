use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error};

use crate::{
    database::{get_connection, poll::DatabasePoll},
    schema,
};

pub fn get_poll_by_id(id: uuid::Uuid) -> Result<DatabasePoll, diesel::result::Error> {
    debug!(poll_id = %id, "getting poll by id");

    let mut conn = get_connection();

    match schema::polls::table
        .find(id)
        .select(DatabasePoll::as_select())
        .first::<DatabasePoll>(&mut conn)
    {
        Ok(poll) => {
            debug!(poll_id = %id, "found poll");
            Ok(poll)
        }
        Err(error) => {
            error!(%error, poll_id = %id, "error finding poll by id");
            Err(error)
        }
    }
}
