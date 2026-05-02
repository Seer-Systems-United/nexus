//! # Database module
//!
//! This module handles PostgreSQL database connections, pooling, and
//! high-level database operations for the Nexus system using Diesel
//! and r2d2 connection pooling.
//!
//! ## Module structure
//!
//! - `ops`: Database CRUD operations for users, passwords, and polling data.
//! - `schema`: Diesel schema definitions (auto-generated or handwritten).
//!
//! ## Configuration
//!
//! Requires `DATABASE_URL` environment variable to be set.
//!
//! ## Connection pooling
//!
//! Uses r2d2 with `test_on_check_out` enabled to validate connections
//! before returning them from the pool.

pub mod ops;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;

/// Alias for Diesel PostgreSQL connection type.
pub type DbConnection = PgConnection;

/// Alias for r2d2 connection pool using PostgreSQL connections.
pub type DbPool = Pool<ConnectionManager<DbConnection>>;

/// Reads the PostgreSQL connection URL from the `DATABASE_URL` environment variable.
///
/// # Panics
///
/// Panics if `DATABASE_URL` is not set in the environment.
///
/// # Returns
///
/// The PostgreSQL connection URL as a string.
pub fn database_url_for_env() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

/// Creates a new r2d2 PostgreSQL connection pool.
///
/// # Configuration
///
/// Uses the URL from `database_url_for_env()`.
///
/// # Pool settings
///
/// - `test_on_check_out`: `true` (validates connections before use).
///
/// # Panics
///
/// Panics if the pool cannot be built (e.g., invalid URL, database unreachable).
///
/// # Returns
///
/// A configured `DbPool` ready for use.
pub fn get_connection_pool() -> DbPool {
    let url = database_url_for_env();
    let manager = ConnectionManager::<DbConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

/// Creates a new OIDC user in the database.
///
/// # Parameters
///
/// - `conn`: Mutable reference to a PostgreSQL connection.
/// - `user_name`: Display name for the new user.
/// - `user_email`: Verified email address for the new user.
///
/// # Returns
///
/// Returns `Ok(1)` on success (1 row inserted).
/// Returns `Err` if the database insert fails.
pub fn create_user(
    conn: &mut DbConnection,
    user_name: &str,
    user_email: &str,
) -> Result<usize, Error> {
    ops::user::create_oidc_user(conn, user_name, user_email).map(|_| 1)
}
