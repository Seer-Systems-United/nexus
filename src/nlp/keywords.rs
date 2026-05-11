use rake::{Rake, StopWords};
use tracing::{debug, info, trace, warn};

pub fn init_keywords() {
    info!("initializing keywords module");

    debug!("getting stop words for English");

    // Get the list of stop words for English
    let list = stop_words::get(stop_words::LANGUAGE::English);

    trace!(?list, "got stop words for English");

    debug!("saving stop words to file");

    // Save the list of stop words to a file
    std::fs::write("stop_words.txt", list.join("\n")).expect("Unable to write stop words to file");

    info!("saved stop words to file");
}

pub fn extract_keywords(text: &str) {
    info!(?text, "extracting keywords from text");

    debug!("getting stop words from file");

    // Create a StopWords instance from the file
    let stop_words =
        StopWords::from_file("stop_words.txt").expect("Unable to create StopWords from file");

    trace!(?stop_words, "loaded stop words");

    debug!("running RAKE");

    // Create a Rake instance with the stop words and run it on the stemmed text
    let r = Rake::new(stop_words.clone());
    let keywords = r.run(&text);

    trace!(?keywords, "extracted keyword scores");

    debug!("filtering keywords by score threshold");

    let mut together = String::new();
    let mut kept = 0usize;
    let mut skipped = 0usize;

    for keyword in keywords {
        trace!(
            keyword = %keyword.keyword,
            score = keyword.score,
            "considering keyword"
        );

        if keyword.score < 1.0 {
            skipped += 1;
            continue;
        }

        kept += 1;
        together.push(' ');
        together.push_str(&keyword.keyword);
    }

    debug!(kept, skipped, "filtered keywords");

    let filtered = together.trim();
    if filtered.is_empty() {
        warn!(kept, skipped, "no keywords met the score threshold");
    } else {
        info!(keywords = %filtered, kept, skipped, "filtered keywords");
    }
}
