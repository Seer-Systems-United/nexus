use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::database::schema::user::User;

#[derive(Debug, Deserialize, ToSchema)]
pub struct SignupRequest {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub account_number: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: UserResponse,
}

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
