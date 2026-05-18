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
        .questions()
        .from_question("Trump")
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
