//! # Authentication request/response types
//!
//! Defines serializable request and response structs for authentication
//! endpoints (signup, login) with OpenAPI schema support.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::database::schema::user::User;

/// Request payload for user signup.
///
/// # Fields
/// - `name`: Display name of the new user.
/// - `password`: Plaintext password for the new user (hashed before storage).
#[derive(Debug, Deserialize, ToSchema)]
pub struct SignupRequest {
    pub name: String,
    pub password: String,
}

/// Request payload for user login.
///
/// # Fields
/// - `account_number`: Unique account number assigned to the user.
/// - `password`: Plaintext password for the user.
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub account_number: String,
    pub password: String,
}

/// Response returned after successful authentication.
///
/// # Fields
/// - `token`: JWT bearer token for authenticated requests.
/// - `token_type`: Type of token (always "Bearer").
/// - `expires_in`: Token validity duration in seconds.
/// - `user`: Authenticated user's public details.
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: UserResponse,
}

/// Public user details returned in authentication responses.
///
/// # Fields
/// - `id`: Unique user ID (UUID string).
/// - `name`: Display name of the user.
/// - `email`: Optional email address of the user.
/// - `account_number`: Optional unique account number.
/// - `created_at`: ISO 8601 timestamp of user creation.
#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub account_number: Option<String>,
    pub created_at: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
            account_number: user.account_number,
            created_at: user.created_at.to_string(),
        }
    }
}
