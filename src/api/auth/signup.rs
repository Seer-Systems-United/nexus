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
        (status = 409, description = "User already exists", body = crate::api::error::ApiErrorBody),
        (status = 503, description = "Database unavailable", body = crate::api::error::ApiErrorBody),
    )
)]
pub async fn post_signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    validate_signup_request(&request)?;

    let name = request.name;
    let email = request.email;
    let supplied_password = request.password;

    let user = db::run(state.clone(), move |conn| {
        conn.transaction::<User, ApiError, _>(|conn| {
            match user::get_user_by_email(conn, &email) {
                Ok(_) => return Err(ApiError::conflict("user already exists")),
                Err(diesel::result::Error::NotFound) => {}
                Err(error) => return Err(ApiError::database(error)),
            }

            let created_user =
                user::create_user(conn, &name, &email).map_err(ApiError::database)?;

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

    if request.email.trim().is_empty() {
        return Err(ApiError::bad_request("email is required"));
    }

    if request.password.len() < 8 {
        return Err(ApiError::bad_request(
            "password must be at least 8 characters long",
        ));
    }

    Ok(())
}
