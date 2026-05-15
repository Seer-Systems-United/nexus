use nexus::{Nexus, expr::get::get, poll, utils::logging::init_tracing};

fn main() {
    init_tracing();

    let nexus = match Nexus::new() {
        Ok(nexus) => nexus,
        Err(error) => {
            eprintln!("failed to initialize nexus: {error}");
            std::process::exit(1);
        }
    };

    match get()
        .responses()
        .from_question("Trump")
        .from_demographic(poll::response::demographic::Demographic::Sex {
            sex: poll::response::demographic::sex::Sex::Female,
        })
        .execute_with(nexus.backend())
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
