use human_name::Name;
use rake::{Rake, StopWords};

pub fn extract_keywords(text: &str, stop_words: &StopWords) {
    // Create a Rake instance with the stop words and run it on the stemmed text
    let r = Rake::new(stop_words.clone());
    let keywords = r.run(&text);

    println!("Keywords: {:?}", keywords);

    let mut together = "".to_string();

    for keyword in keywords {
        if keyword.score < 1.0 {
            continue;
        }

        together = together.to_string() + " " + &keyword.keyword;
    }

    println!("Together: {}", together);
}

fn main() {
    // Get the list of stop words for English
    let list = stop_words::get(stop_words::LANGUAGE::English);

    // Save the list of stop words to a file
    std::fs::write("stop_words.txt", list.join("\n")).expect("Unable to write stop words to file");

    let ner_model =

    // Create a StopWords instance from the file
    let sw = StopWords::from_file("stop_words.txt").expect("Unable to create StopWords from file");

    // Example text to extract keywords from
    let text = "5. Should Trump Have Sought Congressional Approval Before Strikes in Iran";

    extract_keywords(text, &sw);

    let text = "Favorability of Trump Administration Figures — Kristi Noem";

    extract_keywords(text, &sw);
}
