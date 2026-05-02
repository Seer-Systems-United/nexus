//! # User database operations module
//!
//! Handles user creation, lookup by ID/email/account number,
//! and account number generation for polling federation users.

mod account;

pub use account::{
    AccountNumberError, generate_account_number, is_valid_account_number, normalize_account_number,
};

use crate::database::DbConnection;
use crate::database::schema::user::users::dsl::{
    account_number as user_account_number, email as user_email, users,
};
use crate::database::schema::user::{NewUser, User};
use diesel::prelude::*;

/// Create a new user from Google OpenID Connect with email.
///
/// # Parameters
/// - `conn`: Database connection.
/// - `name`: Display name from Google profile.
/// - `email`: Email address from Google profile.
///
/// # Returns
/// - `Ok(User)`: The newly created user record.
pub fn create_oidc_user(conn: &mut DbConnection, name: &str, email: &str) -> QueryResult<User> {
    let new_user = NewUser {
        id: uuid::Uuid::new_v4(),
        name: name.to_owned(),
        email: Some(email.to_owned()),
        account_number: None,
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
}

/// Create a new user with an allocated account number (password-based auth).
///
/// # Parameters
/// - `conn`: Database connection.
/// - `name`: Display name for the user.
/// - `account_number`: Pre-allocated unique account number.
///
/// # Returns
/// - `Ok(User)`: The newly created user record.
pub fn create_account_number_user(
    conn: &mut DbConnection,
    name: &str,
    account_number: &str,
) -> QueryResult<User> {
    let new_user = NewUser {
        id: uuid::Uuid::new_v4(),
        name: name.to_owned(),
        email: None,
        account_number: Some(account_number.to_owned()),
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
}

/// Look up a user by their UUID.
///
/// # Parameters
/// - `conn`: Database connection.
/// - `id`: UUID of the user to find.
///
/// # Returns
/// - `Ok(User)`: The user record if found.
pub fn get_user_by_id(conn: &mut DbConnection, id: &uuid::Uuid) -> QueryResult<User> {
    users.find(*id).first(conn)
}

/// Look up a user by their email address.
///
/// # Parameters
/// - `conn`: Database connection.
/// - `email`: Email address to search for.
///
/// # Returns
/// - `Ok(User)`: The user record if found.
pub fn get_user_by_email(conn: &mut DbConnection, email: &str) -> QueryResult<User> {
    users.filter(user_email.eq(email)).first(conn)
}

/// Look up a user by their account number.
///
/// # Parameters
/// - `conn`: Database connection.
/// - `account_number`: Account number to search for.
///
/// # Returns
/// - `Ok(User)`: The user record if found.
pub fn get_user_by_account_number(
    conn: &mut DbConnection,
    account_number: &str,
) -> QueryResult<User> {
    users
        .filter(user_account_number.eq(account_number))
        .first(conn)
}

/// Get an existing user by email, or create a new OIDC user if not found.
///
/// # Parameters
/// - `conn`: Database connection.
/// - `name`: Display name for new user creation.
/// - `email`: Email address to search or create with.
///
/// # Returns
/// - `Ok(User)`: Existing or newly created user record.
pub fn get_or_create_user_by_email(
    conn: &mut DbConnection,
    name: &str,
    email: &str,
) -> QueryResult<User> {
    match get_user_by_email(conn, email) {
        Ok(user) => Ok(user),
        Err(diesel::result::Error::NotFound) => create_oidc_user(conn, name, email),
        Err(error) => Err(error),
    }
}
