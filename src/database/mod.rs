#[cfg(not(any(feature = "postgres", feature = "sqlite")))]
compile_error!("Enable at least one database backend feature: `postgres` or `sqlite`.");

mod common;
pub mod models;
pub use models::{demographic, person, poll, question, response, response_unit, source};

pub mod backend {
    pub mod traits;

    #[cfg(feature = "postgres")]
    pub mod postgres;

    #[cfg(feature = "sqlite")]
    pub mod sqlite;

    pub use traits::BackendTrait;
}

pub use backend::BackendTrait;

#[cfg(feature = "postgres")]
pub use backend::postgres;
#[cfg(feature = "postgres")]
pub use backend::postgres::{PostgresBackend as DefaultBackend, default_backend, init_database};

#[cfg(feature = "sqlite")]
pub use backend::sqlite;
#[cfg(all(not(feature = "postgres"), feature = "sqlite"))]
pub use backend::sqlite::{SqliteBackend as DefaultBackend, default_backend};
