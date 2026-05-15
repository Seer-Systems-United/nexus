use std::{env, sync::OnceLock};

use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenvy::dotenv;
use tracing::{debug, info};

pub type DbConnectionManager = ConnectionManager<PgConnection>;
pub type DbPool = Pool<DbConnectionManager>;
pub type DbConnection = PooledConnection<DbConnectionManager>;

static DB_POOL: OnceLock<DbPool> = OnceLock::new();

pub fn init_database() {
    info!("Initializing database connection pool");
    let _ = pool();
    info!("Database connection pool initialized");
}

fn pool() -> &'static DbPool {
    DB_POOL.get_or_init(establish_connection_pool)
}

fn establish_connection_pool() -> DbPool {
    debug!("Loading environment via dotenv (if present)");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    debug!("Creating Postgres connection manager");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Error creating database connection pool")
}

pub(super) fn get_connection() -> DbConnection {
    debug!("Acquiring database connection from pool");
    pool().get().expect("Failed to get database connection")
}
