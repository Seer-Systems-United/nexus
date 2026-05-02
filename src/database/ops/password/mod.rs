//! # Password operations module
//!
//! Handles password hashing, verification, storage, and retrieval
//! using PBKDF2-SHA256 with configurable iterations.

mod format;
mod hash;

pub use hash::{get_password_hash, hash_password, verify_password};

use crate::database::DbConnection;
use crate::database::schema::login::login::dsl::{
    created_at as login_created_at, login, user_id as login_user_id,
};
use crate::database::schema::login::{Login, NewLogin};
use diesel::prelude::*;

/// Errors that can occur during password operations.
///
/// # Variants
/// - `Crypto`: Cryptographic operation failed (e.g., random salt generation).
/// - `InvalidFormat`: Password hash string has invalid format.
/// - `InvalidBase64`: Base64 decoding of hash components failed.
/// - `InvalidIterations`: Iteration count in stored hash is invalid.
/// - `Database`: Diesel database error.
#[derive(Debug)]
pub enum PasswordError {
    Crypto,
    InvalidFormat,
    InvalidBase64(base64::DecodeError),
    InvalidIterations,
    Database(diesel::result::Error),
}

impl From<diesel::result::Error> for PasswordError {
    fn from(error: diesel::result::Error) -> Self {
        Self::Database(error)
    }
}

/// Create a new login record with a hashed password for a user.
///
/// # Parameters
/// - `conn`: Database connection.
/// - `user_id`: UUID of the user to create login for.
/// - `password`: Plaintext password to hash and store.
///
/// # Returns
/// - `Ok(Login)`: The newly created login record.
///
/// # Errors
/// - `PasswordError::Crypto`: Password hashing failed.
/// - `PasswordError::Database`: Database insert failed.
pub fn create_login(
    conn: &mut DbConnection,
    user_id: uuid::Uuid,
    password: &str,
) -> Result<Login, PasswordError> {
    let password_hash = hash_password(password)?;
    let new_login = NewLogin {
        id: uuid::Uuid::new_v4(),
        user_id,
        password_hash,
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(login)
        .values(&new_login)
        .get_result(conn)
        .map_err(PasswordError::from)
}

/// Get the most recent login record for a user.
///
/// # Parameters
/// - `conn`: Database connection.
/// - `user_id`: UUID of the user.
///
/// # Returns
/// - `Ok(Login)`: The most recently created login record.
pub fn get_latest_login_for_user(
    conn: &mut DbConnection,
    user_id: uuid::Uuid,
) -> QueryResult<Login> {
    login
        .filter(login_user_id.eq(user_id))
        .order(login_created_at.desc())
        .first(conn)
}

/// Verify a user's password against their latest login record.
///
/// # Parameters
/// - `conn`: Database connection.
/// - `user_id`: UUID of the user.
/// - `password`: Plaintext password to verify.
///
/// # Returns
/// - `Ok(true)`: Password matches.
/// - `Ok(false)`: Password does not match.
///
/// # Errors
/// - `PasswordError::Database`: Failed to fetch login record.
/// - `PasswordError::InvalidFormat`: Stored hash has invalid format.
pub fn verify_login_password(
    conn: &mut DbConnection,
    user_id: uuid::Uuid,
    password: &str,
) -> Result<bool, PasswordError> {
    let login_row = get_latest_login_for_user(conn, user_id)?;

    verify_password(password, &login_row.password_hash)
}
