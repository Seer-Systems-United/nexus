use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use tracing::{debug, error, instrument};

use crate::{
    database::{
        demographic::create::create_demographic_in_db, get_connection, response::DatabaseResponse,
        response_unit::create::create_response_unit_in_db,
    },
    poll::response::Response,
    schema,
};

fn create_response(
    question_id: uuid::Uuid,
    demographic_id: uuid::Uuid,
    unit_id: uuid::Uuid,
    answer: String,
    value: i32,
) -> DatabaseResponse {
    DatabaseResponse {
        id: uuid::Uuid::new_v4(),
        question_id,
        demographic_id,
        unit_id,
        answer,
        value,
    }
}

#[instrument(level = "info", skip_all, fields(question_id = %question_id, answer = %response.answer, value = response.value))]
pub fn create_response_in_db(
    question_id: uuid::Uuid,
    response: &Response,
) -> Result<DatabaseResponse, diesel::result::Error> {
    debug!("creating response");

    let demographic = create_demographic_in_db(&response.demographic)?;
    let unit = create_response_unit_in_db(&response.unit)?;
    let answer = response.answer.to_string();
    let value = i32::from(response.value);
    let mut conn = get_connection();

    match schema::responses::table
        .filter(
            schema::responses::question_id
                .eq(question_id)
                .and(schema::responses::demographic_id.eq(demographic.id))
                .and(schema::responses::unit_id.eq(unit.id))
                .and(schema::responses::answer.eq(&answer))
                .and(schema::responses::value.eq(value)),
        )
        .select(DatabaseResponse::as_select())
        .first::<DatabaseResponse>(&mut conn)
        .optional()
    {
        Ok(Some(response)) => {
            debug!(response_id = %response.id, "response already exists");
            return Ok(response);
        }
        Ok(None) => {}
        Err(error) => {
            error!(%error, "error checking for existing response");
            return Err(error);
        }
    }

    let new_response = create_response(question_id, demographic.id, unit.id, answer, value);

    match diesel::insert_into(schema::responses::table)
        .values(&new_response)
        .returning(DatabaseResponse::as_returning())
        .get_result(&mut conn)
    {
        Ok(response) => {
            debug!(response_id = %response.id, "inserted response");
            Ok(response)
        }
        Err(error) => {
            error!(%error, response_id = %new_response.id, "error inserting response");
            Err(error)
        }
    }
}
