pub mod demographic;
pub mod person;
pub mod poll;
pub mod question;
pub mod response;
pub mod response_unit;
pub mod source;

use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenvy::dotenv;
use std::{env, sync::OnceLock};
use tracing::{debug, info};

pub type DbConnectionManager = ConnectionManager<PgConnection>;
pub type DbPool = Pool<DbConnectionManager>;
pub type DbConnection = PooledConnection<DbConnectionManager>;

pub static DB_POOL: OnceLock<DbPool> = OnceLock::new();

pub fn init_database() {
    info!("Initializing database connection pool");

    let pool = establish_connection_pool();
    DB_POOL
        .set(pool)
        .expect("Failed to set database connection pool");

    info!("Database connection pool initialized");
}

pub fn establish_connection_pool() -> DbPool {
    debug!("Loading environment via dotenv (if present)");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    debug!("Creating Postgres connection manager");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    debug!("Building r2d2 connection pool");
    Pool::builder()
        .build(manager)
        .expect("Error creating database connection pool")
}

pub fn get_connection() -> DbConnection {
    debug!("Acquiring database connection from pool");
    DB_POOL
        .get()
        .expect("Database connection pool not initialized")
        .get()
        .expect("Failed to get database connection from pool")
}
