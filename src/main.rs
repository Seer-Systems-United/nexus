pub mod database;
pub mod expr;
pub mod nlp;
pub mod poll;
pub mod utils;

use crate::{database::default_backend, expr::get::get, utils::logging::init_tracing};

fn main() {
    init_tracing();

    let backend = match default_backend() {
        Ok(backend) => backend,
        Err(error) => {
            eprintln!("failed to initialize backend: {error}");
            std::process::exit(1);
        }
    };

    match get()
        .responses()
        .from_question("Trump")
        .from_demographic(poll::response::demographic::Demographic::Sex {
            sex: poll::response::demographic::sex::Sex::Female,
        })
        .execute_with(&backend)
    {
        Ok(responses) => {
            dbg!(responses);
        }
        Err(error) => {
            eprintln!("failed to fetch responses: {error}");
            std::process::exit(1);
        }
    }
}
