use std::{env, sync::OnceLock};

use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenvy::dotenv;
use tracing::{debug, error, info, instrument};

pub type DbConnectionManager = ConnectionManager<PgConnection>;
pub type DbPool = Pool<DbConnectionManager>;
pub type DbConnection = PooledConnection<DbConnectionManager>;

static DB_POOL: OnceLock<DbPool> = OnceLock::new();

#[instrument]
pub fn init_database() {
    info!("Initializing database connection pool");
    let _ = pool();
    info!("Database connection pool initialized");
}

fn pool() -> &'static DbPool {
    DB_POOL.get_or_init(establish_connection_pool)
}

#[instrument]
fn establish_connection_pool() -> DbPool {
    debug!("Loading environment via dotenv (if present)");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        error!("DATABASE_URL must be set");
        panic!("DATABASE_URL must be set");
    });

    debug!("Creating Postgres connection manager");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .map_err(|e| {
            error!("Error creating database connection pool: {}", e);
            e
        })
        .expect("Error creating database connection pool")
}

#[instrument(skip_all)]
pub(super) fn get_connection() -> DbConnection {
    debug!("Acquiring database connection from pool");
    pool()
        .get()
        .map_err(|e| {
            error!("Failed to get database connection from pool: {}", e);
            e
        })
        .expect("Failed to get database connection")
}
