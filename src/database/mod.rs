pub mod ops;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;

pub type DbConnection = PgConnection;
pub type DbPool = Pool<ConnectionManager<DbConnection>>;

pub fn database_url_for_env() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

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

pub fn create_user(
    conn: &mut DbConnection,
    user_name: &str,
    user_email: &str,
) -> Result<usize, Error> {
    ops::user::create_oidc_user(conn, user_name, user_email).map(|_| 1)
}
