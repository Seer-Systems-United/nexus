use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, error};

use crate::{
    database::{get_connection, response::DatabaseResponse},
    schema,
};

pub fn search_responses_by_question_id(
    question_id: uuid::Uuid,
) -> Result<Vec<DatabaseResponse>, diesel::result::Error> {
    debug!(question_id = %question_id, "searching responses by question id");

    let mut conn = get_connection();

    match schema::responses::table
        .filter(schema::responses::question_id.eq(question_id))
        .select(DatabaseResponse::as_select())
        .load::<DatabaseResponse>(&mut conn)
    {
        Ok(responses) => {
            debug!(count = responses.len(), question_id = %question_id, "found responses by question id");
            Ok(responses)
        }
        Err(error) => {
            error!(%error, question_id = %question_id, "error searching responses by question id");
            Err(error)
        }
    }
}
