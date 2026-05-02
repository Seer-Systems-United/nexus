pub(super) fn short_hash(input: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in input.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

pub(super) fn slug_id(input: &str) -> String {
    let input = input
        .trim()
        .trim_start_matches("/api/v1/topics/")
        .trim_start_matches("api/v1/topics/")
        .trim_start_matches("topics/");
    let mut output = String::new();
    let mut last_dash = false;

    for ch in input.to_ascii_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch);
            last_dash = false;
        } else if !last_dash {
            output.push('-');
            last_dash = true;
        }
    }

    output.trim_matches('-').to_string()
}

pub(super) fn label_from_topic_id(topic_id: &str) -> String {
    topic_id
        .trim_start_matches("headline-")
        .split('-')
        .filter(|part| !part.is_empty())
        .map(|part| match part {
            "ai" => "AI".to_string(),
            "us" => "US".to_string(),
            "uk" => "UK".to_string(),
            other => {
                let mut chars = other.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                    None => String::new(),
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
