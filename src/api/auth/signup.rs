use axum::Json;
use axum::extract::State;
use diesel::Connection;

use crate::AppState;
use crate::api::auth::auth_response;
use crate::api::auth::types::{AuthResponse, SignupRequest};
use crate::api::db;
use crate::api::error::ApiError;
use crate::database::ops::{password, user};
use crate::database::schema::user::User;

#[utoipa::path(
    post,
    path = "/signup",
    tag = "Auth",
    request_body = SignupRequest,
    responses(
        (status = 200, description = "Signup successful", body = AuthResponse),
        (status = 400, description = "Invalid signup payload", body = crate::api::error::ApiErrorBody),
        (status = 503, description = "Database unavailable", body = crate::api::error::ApiErrorBody),
    )
)]
pub async fn post_signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    validate_signup_request(&request)?;

    let name = request.name;
    let supplied_password = request.password;

    let user = db::run(state.clone(), move |conn| {
        conn.transaction::<User, ApiError, _>(|conn| {
            let account_number = allocate_account_number(conn)?;

            let created_user = user::create_account_number_user(conn, &name, &account_number)
                .map_err(ApiError::database)?;

            password::create_login(conn, created_user.id, &supplied_password)
                .map_err(ApiError::password)?;

            Ok(created_user)
        })
    })
    .await?;

    Ok(Json(auth_response(&state.jwt, user)?))
}

fn validate_signup_request(request: &SignupRequest) -> Result<(), ApiError> {
    if request.name.trim().is_empty() {
        return Err(ApiError::bad_request("name is required"));
    }

    if request.password.len() < 8 {
        return Err(ApiError::bad_request(
            "password must be at least 8 characters long",
        ));
    }

    Ok(())
}

fn allocate_account_number(conn: &mut crate::database::DbConnection) -> Result<String, ApiError> {
    for _ in 0..10 {
        let account_number = user::generate_account_number()
            .map_err(|_| ApiError::internal("failed to generate account number"))?;

        match user::get_user_by_account_number(conn, &account_number) {
            Ok(_) => {}
            Err(diesel::result::Error::NotFound) => return Ok(account_number),
            Err(error) => return Err(ApiError::database(error)),
        }
    }

    Err(ApiError::internal("failed to allocate account number"))
}
