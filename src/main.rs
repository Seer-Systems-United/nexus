pub mod database;
pub mod expr;
pub mod nlp;
pub mod poll;
pub mod schema;
pub mod utils;

use crate::{database::init_database, expr::get::get, utils::logging::init_tracing};

fn main() {
    init_tracing();
    // init_nlp();
    init_database();

    match get().polls().execute() {
        Ok(polls) => {
            dbg!(polls);
        }
        Err(error) => {
            eprintln!("failed to fetch polls: {error}");
            std::process::exit(1);
        }
    }
}
