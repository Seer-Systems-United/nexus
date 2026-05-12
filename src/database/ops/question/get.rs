use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error};

use crate::{
    database::{get_connection, ops::question::DatabaseQuestion},
    schema::{self},
};

pub fn get_question_by_id(id: uuid::Uuid) -> Result<DatabaseQuestion, diesel::result::Error> {
    debug!(question_id = %id, "getting question by id");

    let mut conn = get_connection();

    match schema::questions::table
        .find(id)
        .select(DatabaseQuestion::as_select())
        .first::<DatabaseQuestion>(&mut conn)
    {
        Ok(question) => {
            debug!(question_id = %id, "found question");
            Ok(question)
        }
        Err(e) => {
            error!(error = %e, question_id = %id, "error finding question by id");
            Err(e)
        }
    }
}
