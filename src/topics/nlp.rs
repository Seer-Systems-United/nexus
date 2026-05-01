use crate::sources::DataStructure;
use crate::topics::mappings::{self, TopicMatch};
use crate::topics::types::{Compatibility, SourceId, TopicObservation, TopicStatus, TopicSummary};
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;
use stop_words::LANGUAGE;
use strsim::jaro_winkler;

const MIN_TERMS: usize = 2;
const MAX_LABEL_TERMS: usize = 5;

#[derive(Debug, Clone)]
struct Term {
    key: String,
    label: String,
}

#[derive(Debug, Clone)]
struct CandidateTerms {
    terms: Vec<Term>,
    features: HashSet<String>,
    signature: String,
    intent: Option<String>,
}

#[derive(Debug, Clone)]
struct Candidate {
    index: usize,
    terms: CandidateTerms,
}

#[derive(Debug, Clone)]
struct ClusterTerm {
    label: String,
    count: usize,
}

#[derive(Debug, Clone)]
struct Cluster {
    candidates: Vec<Candidate>,
    term_counts: HashMap<String, ClusterTerm>,
}

pub(crate) fn headline_candidate_match(
    source: SourceId,
    structure: &DataStructure,
) -> Option<TopicMatch> {
    if !matches!(
        structure,
        DataStructure::BarGraph { .. }
            | DataStructure::LineGraph { .. }
            | DataStructure::PieChart { .. }
            | DataStructure::Crosstab { .. }
    ) {
        return None;
    }

    let text = mappings::common::structure_text(structure);
    if is_duplicate_source_question(source, &text) {
        return None;
    }

    let terms = extract_terms(&text)?;
    let label = label_from_terms(&terms.terms, None);
    let topic_id = format!("headline-candidate-{}", short_hash(&terms.signature));

    Some(TopicMatch {
        topic: TopicSummary {
            id: topic_id.clone(),
            label,
            status: TopicStatus::Headline,
            description: Some("Dynamically generated from poll question wording.".to_string()),
            endpoint: Some(format!("/api/v1/topics/{topic_id}")),
        },
        compatibility: Compatibility::RollupCompatible,
    })
}

pub(crate) fn cluster_headline_observations(observations: &mut [TopicObservation]) {
    let candidates = observations
        .iter()
        .enumerate()
        .filter(|(_, observation)| observation.topic_id.starts_with("headline-candidate-"))
        .filter_map(|(index, observation)| {
            let mut terms = extract_terms(&format!(
                "{} {}",
                observation.question_title, observation.prompt
            ))?;
            terms.intent = intent_from_observation(observation).or(terms.intent);
            Some(Candidate { index, terms })
        })
        .collect::<Vec<_>>();

    let clusters = cluster_candidates(candidates);
    let mut used_ids = HashSet::new();

    for cluster in clusters {
        let label = cluster_label(&cluster);
        let mut topic_id = format!("headline-{}", slug(&label));
        if topic_id == "headline-" {
            topic_id = format!("headline-{}", short_hash(&cluster_key(&cluster)));
        }
        if !used_ids.insert(topic_id.clone()) {
            topic_id = format!("{topic_id}-{}", short_hash(&cluster_key(&cluster)));
            used_ids.insert(topic_id.clone());
        }

        for candidate in cluster.candidates {
            let observation = &mut observations[candidate.index];
            observation.topic_id = topic_id.clone();
            observation.topic_label = label.clone();
            observation.compatibility = Compatibility::RollupCompatible;
        }
    }
}

pub(crate) fn normalized_question_key(text: &str) -> String {
    normalized_search_text(&focus_question_text(text))
}

fn is_duplicate_source_question(source: SourceId, text: &str) -> bool {
    matches!(source, SourceId::Ipsos)
        && (text.contains("approval5_1.") || text.contains("approval5_2."))
}

fn cluster_candidates(candidates: Vec<Candidate>) -> Vec<Cluster> {
    let mut clusters = Vec::new();

    for candidate in candidates {
        let mut best_index = None;
        let mut best_score = 0.0;

        for (index, cluster) in clusters.iter().enumerate() {
            let score = cluster_similarity(&candidate, cluster);
            if score > best_score {
                best_score = score;
                best_index = Some(index);
            }
        }

        if let Some(index) = best_index.filter(|_| best_score >= 0.45) {
            clusters[index].add(candidate);
        } else {
            clusters.push(Cluster::new(candidate));
        }
    }

    clusters
}

fn cluster_similarity(candidate: &Candidate, cluster: &Cluster) -> f32 {
    cluster
        .candidates
        .iter()
        .filter_map(|existing| similarity_score(&candidate.terms, &existing.terms))
        .fold(0.0, f32::max)
}

fn similarity_score(left: &CandidateTerms, right: &CandidateTerms) -> Option<f32> {
    if let (Some(left_intent), Some(right_intent)) = (&left.intent, &right.intent)
        && left_intent != right_intent
    {
        return None;
    }

    let shared = left.features.intersection(&right.features).count();
    if shared == 0 {
        return None;
    }

    let union = left.features.union(&right.features).count();
    let smaller = left.features.len().min(right.features.len());
    if union == 0 || smaller == 0 {
        return None;
    }

    let jaccard = shared as f32 / union as f32;
    let containment = shared as f32 / smaller as f32;
    let fuzzy = jaro_winkler(&left.signature, &right.signature) as f32;

    if shared >= 2 && (jaccard >= 0.25 || containment >= 0.50 || fuzzy >= 0.86) {
        return Some(jaccard.max(containment * 0.8).max(fuzzy * 0.65));
    }

    if shared == 1 && fuzzy >= 0.92 {
        return Some(fuzzy * 0.55);
    }

    None
}

fn extract_terms(text: &str) -> Option<CandidateTerms> {
    let focused = focus_question_text(text);
    let normalized = normalized_search_text(&focused);
    let stemmer = Stemmer::create(Algorithm::English);
    let mut terms = Vec::new();
    let mut seen = HashSet::new();
    let intent = intent_from_text(&normalized);

    for term in phrase_terms(&normalized) {
        push_term(&mut terms, &mut seen, term);
    }

    for token in token_regex()
        .find_iter(&normalized)
        .map(|match_| match_.as_str())
    {
        if let Some(term) = canonical_term(token, &stemmer) {
            push_term(&mut terms, &mut seen, term);
        }
    }

    if terms.len() < MIN_TERMS {
        return None;
    }

    let features = terms
        .iter()
        .map(|term| term.key.clone())
        .collect::<HashSet<_>>();
    let mut signature_terms = features.iter().cloned().collect::<Vec<_>>();
    signature_terms.sort();

    Some(CandidateTerms {
        terms,
        features,
        signature: signature_terms.join(" "),
        intent,
    })
}

fn focus_question_text(text: &str) -> String {
    if let Some(match_) = question_code_regex().find(text) {
        return text[match_.start()..]
            .trim_start_matches(|ch: char| ch == ':' || ch.is_whitespace())
            .to_string();
    }

    if let Some((_, tail)) = text.rsplit_once(':')
        && tail.split_whitespace().count() >= 2
    {
        return tail.trim().to_string();
    }

    text.to_string()
}

fn token_regex() -> &'static Regex {
    static TOKEN_REGEX: OnceLock<Regex> = OnceLock::new();
    TOKEN_REGEX.get_or_init(|| Regex::new(r"[a-z][a-z0-9]+").expect("valid token regex"))
}

fn question_code_regex() -> &'static Regex {
    static QUESTION_CODE_REGEX: OnceLock<Regex> = OnceLock::new();
    QUESTION_CODE_REGEX.get_or_init(|| {
        Regex::new(r"(?i)(?:^|[:\s])(?:[a-z][a-z0-9_-]{0,24}|\d{1,3})\.\s+")
            .expect("valid question code regex")
    })
}

fn normalized_search_text(text: &str) -> String {
    let lower = text
        .to_ascii_lowercase()
        .replace("u.s.", " us ")
        .replace("u. s.", " us ")
        .replace("united states", " us ")
        .replace("don't", " dont ")
        .replace("don’t", " dont ");

    lower.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn phrase_terms(text: &str) -> Vec<Term> {
    let mut terms = Vec::new();

    if phrase_match(text, r"\b(artificial intelligence|generative ai|ai)\b") {
        terms.push(term("ai", "AI"));
    }

    if phrase_match(text, r"\bcost of living\b") {
        terms.push(term("price", "price"));
    }

    if phrase_match(text, r"\b(sports betting|online betting|online wagering)\b") {
        terms.push(term("sports", "sports"));
        terms.push(term("betting", "betting"));
    }

    if phrase_match(
        text,
        r"\b(military action|military strike|military strikes|air strike|air strikes|send troops|ground troops|war)\b",
    ) {
        terms.push(term("military-conflict", "military conflict"));
    }

    terms
}

fn phrase_match(text: &str, pattern: &str) -> bool {
    Regex::new(pattern)
        .expect("valid headline phrase regex")
        .is_match(text)
}

fn intent_from_text(text: &str) -> Option<String> {
    if text.contains("support") && text.contains("oppose") {
        return Some("support-oppose".to_string());
    }
    if text.contains("approve") && text.contains("disapprove") {
        return Some("approval".to_string());
    }
    if text.contains("favorable") && text.contains("unfavorable") {
        return Some("favorability".to_string());
    }
    if text.contains("right direction") && text.contains("wrong track") {
        return Some("direction".to_string());
    }
    if text.contains("success") && text.contains("failure") {
        return Some("success-failure".to_string());
    }
    if text.contains("worth it") || text.contains("not worth") {
        return Some("worth-it".to_string());
    }
    if text.contains("vote for") || text.contains("would you vote") {
        return Some("vote-choice".to_string());
    }

    None
}

fn intent_from_observation(observation: &TopicObservation) -> Option<String> {
    let mut answer_text = String::new();
    for demographic in &observation.demographics {
        for answer in &demographic.answers {
            answer_text.push(' ');
            answer_text.push_str(&answer.id);
            answer_text.push(' ');
            answer_text.push_str(&answer.label);
        }
    }

    intent_from_text(&normalized_search_text(&answer_text))
}

fn canonical_term(token: &str, stemmer: &Stemmer) -> Option<Term> {
    if token.chars().all(|ch| ch.is_ascii_digit()) || is_question_code(token) || is_stopword(token)
    {
        return None;
    }

    let aliased = match token {
        "ai" => Some(term("ai", "AI")),
        "covid19" | "covid" | "coronavirus" => Some(term("covid", "COVID")),
        "sport" | "sports" => Some(term("sports", "sports")),
        "wager" | "wagers" | "wagering" | "gamble" | "gambles" | "gambling" | "bet" | "bets"
        | "betting" => Some(term("betting", "betting")),
        "price" | "prices" | "pricing" | "cost" | "costs" => Some(term("price", "price")),
        "subscription" | "subscriptions" | "subscribers" => {
            Some(term("subscription", "subscription"))
        }
        "regulate" | "regulates" | "regulated" | "regulating" | "regulation" | "regulations" => {
            Some(term("regulation", "regulation"))
        }
        "strike" | "strikes" | "airstrike" | "airstrikes" | "troop" | "troops" | "war" | "wars"
        | "military" | "conflict" | "conflicts" => {
            Some(term("military-conflict", "military conflict"))
        }
        "immigration" | "immigrant" | "immigrants" | "migrant" | "migrants" | "border" => {
            Some(term("immigration", "immigration"))
        }
        "economic" | "economics" => Some(term("economy", "economy")),
        "policy" | "policies" => Some(term("policy", "policy")),
        "ukrainian" => Some(term("ukraine", "Ukraine")),
        "israeli" => Some(term("israel", "Israel")),
        "palestinian" | "palestinians" => Some(term("palestine", "Palestine")),
        "republicans" => Some(term("republican", "Republican")),
        "democrats" => Some(term("democrat", "Democrat")),
        _ => None,
    };

    if let Some(term) = aliased {
        return Some(term);
    }

    if token.len() < 3 {
        return None;
    }

    let key = stemmer.stem(token).to_string();
    if key.len() < 3 || is_stopword(&key) {
        return None;
    }

    Some(Term {
        key,
        label: display_label(token),
    })
}

fn term(key: &str, label: &str) -> Term {
    Term {
        key: key.to_string(),
        label: label.to_string(),
    }
}

fn push_term(terms: &mut Vec<Term>, seen: &mut HashSet<String>, term: Term) {
    if seen.insert(term.key.clone()) {
        terms.push(term);
    }
}

fn is_question_code(token: &str) -> bool {
    let has_digit = token.chars().any(|ch| ch.is_ascii_digit());
    let has_alpha = token.chars().any(|ch| ch.is_ascii_alphabetic());
    has_digit && has_alpha && (token.len() <= 16 || token.starts_with("tm"))
}

fn display_label(token: &str) -> String {
    match token {
        "policies" => "policy".to_string(),
        "companies" => "company".to_string(),
        "countries" => "country".to_string(),
        other if other.len() > 4 && other.ends_with('s') && !other.ends_with("ss") => {
            other[..other.len() - 1].to_string()
        }
        other => other.to_string(),
    }
}

fn english_stopwords() -> &'static HashSet<&'static str> {
    static ENGLISH_STOPWORDS: OnceLock<HashSet<&'static str>> = OnceLock::new();
    ENGLISH_STOPWORDS.get_or_init(|| stop_words::get(LANGUAGE::English).iter().copied().collect())
}

fn is_stopword(token: &str) -> bool {
    english_stopwords().contains(token) || is_polling_stopword(token)
}

fn is_polling_stopword(token: &str) -> bool {
    matches!(
        token,
        "adult"
            | "adults"
            | "action"
            | "american"
            | "americans"
            | "answer"
            | "approve"
            | "asked"
            | "believe"
            | "candidate"
            | "citizen"
            | "citizens"
            | "country"
            | "current"
            | "direction"
            | "disapprove"
            | "doing"
            | "dont"
            | "election"
            | "feel"
            | "following"
            | "generally"
            | "heading"
            | "important"
            | "increase"
            | "increasingly"
            | "ipsos"
            | "issue"
            | "issues"
            | "january"
            | "february"
            | "march"
            | "april"
            | "may"
            | "june"
            | "july"
            | "august"
            | "september"
            | "october"
            | "november"
            | "december"
            | "latest"
            | "less"
            | "more"
            | "most"
            | "overall"
            | "ballot"
            | "congress"
            | "held"
            | "primary"
            | "senate"
            | "vote"
            | "voted"
            | "poll"
            | "polling"
            | "polls"
            | "question"
            | "right"
            | "reuter"
            | "reuters"
            | "respond"
            | "respondent"
            | "respondents"
            | "say"
            | "saying"
            | "speaking"
            | "somewhat"
            | "strong"
            | "strongly"
            | "support"
            | "survey"
            | "thing"
            | "things"
            | "think"
            | "today"
            | "total"
            | "track"
            | "trump"
            | "views"
            | "voter"
            | "voters"
            | "washington"
            | "wrong"
            | "worth"
    )
}

fn cluster_label(cluster: &Cluster) -> String {
    let first_terms = cluster
        .candidates
        .first()
        .map(|candidate| candidate.terms.terms.as_slice())
        .unwrap_or_default();

    let label = label_from_terms(first_terms, Some(&cluster.term_counts));
    if label.is_empty() {
        "Recent poll question".to_string()
    } else {
        label
    }
}

fn label_from_terms(
    ordered_terms: &[Term],
    term_counts: Option<&HashMap<String, ClusterTerm>>,
) -> String {
    let mut selected = Vec::new();
    let mut seen = HashSet::new();

    for term in ordered_terms {
        if term_counts
            .map(|counts| {
                counts
                    .get(&term.key)
                    .map(|term| term.count)
                    .unwrap_or_default()
                    > 1
            })
            .unwrap_or(true)
            || selected.is_empty()
        {
            if seen.insert(term.key.clone()) {
                selected.push(term.label.clone());
            }
        }
        if selected.len() >= MAX_LABEL_TERMS {
            break;
        }
    }

    if let Some(counts) = term_counts {
        let mut frequent = counts.iter().collect::<Vec<_>>();
        frequent.sort_by(|(left_key, left_term), (right_key, right_term)| {
            right_term
                .count
                .cmp(&left_term.count)
                .then_with(|| left_key.cmp(right_key))
        });

        for (key, term) in frequent {
            if selected.len() >= MAX_LABEL_TERMS {
                break;
            }
            if seen.insert(key.clone()) {
                selected.push(term.label.clone());
            }
        }
    }

    selected
        .into_iter()
        .map(|term| title_term(&term))
        .collect::<Vec<_>>()
        .join(" ")
}

fn title_term(term: &str) -> String {
    match term {
        "AI" | "COVID" | "U.S." => term.to_string(),
        _ => term
            .split('-')
            .filter(|part| !part.is_empty())
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
    }
}

fn slug(input: &str) -> String {
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

fn short_hash(input: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in input.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{:08x}", hash as u32)
}

fn cluster_key(cluster: &Cluster) -> String {
    let mut terms = cluster.term_counts.keys().cloned().collect::<Vec<_>>();
    terms.sort();
    terms.join("-")
}

impl Cluster {
    fn new(candidate: Candidate) -> Self {
        let mut cluster = Self {
            candidates: Vec::new(),
            term_counts: HashMap::new(),
        };
        cluster.add(candidate);
        cluster
    }

    fn add(&mut self, candidate: Candidate) {
        for term in &candidate.terms.terms {
            self.term_counts
                .entry(term.key.clone())
                .and_modify(|existing| existing.count += 1)
                .or_insert(ClusterTerm {
                    label: term.label.clone(),
                    count: 1,
                });
        }
        self.candidates.push(candidate);
    }
}

#[cfg(test)]
mod tests {
    use super::{cluster_headline_observations, extract_terms, headline_candidate_match};
    use crate::sources::DataStructure;
    use crate::topics::types::{
        Compatibility, SourceId, TopicObservation, TopicSource, TopicStatus,
    };

    #[test]
    fn creates_dynamic_candidate_from_question_text() {
        let structure = DataStructure::BarGraph {
            title: "Do you support or oppose U.S. military strikes against Iran?".to_string(),
            x: vec!["Support".to_string(), "Oppose".to_string()],
            y: vec![45.0, 40.0],
            y_unit: "%".to_string(),
        };

        let topic = headline_candidate_match(SourceId::Emerson, &structure)
            .expect("question should produce dynamic headline candidate");

        assert_eq!(topic.topic.status, TopicStatus::Headline);
        assert!(topic.topic.id.starts_with("headline-candidate-"));
    }

    #[test]
    fn clusters_equivalent_headline_questions() {
        let mut observations = vec![
            observation("Do you support U.S. military action against Iran?"),
            observation("Was it worth it for the United States to go to war with Iran?"),
        ];

        cluster_headline_observations(&mut observations);

        assert_eq!(observations[0].topic_id, observations[1].topic_id);
        assert!(observations[0].topic_id.starts_with("headline-"));
        assert!(!observations[0].topic_id.starts_with("headline-candidate-"));
    }

    #[test]
    fn extracts_salient_terms_without_poll_boilerplate() {
        let terms = extract_terms(
            "Latest Reuters/Ipsos poll: Americans increasingly support sports betting",
        )
        .expect("terms should parse");

        assert!(terms.features.contains("sports"));
        assert!(terms.features.contains("betting"));
        assert!(!terms.features.contains("ipsos"));
        assert!(!terms.features.contains("poll"));
    }

    #[test]
    fn strips_article_prefix_before_question_terms() {
        let terms = extract_terms(
            "2026-04-28: Americans increasingly feel the economy is on the wrong track: AB2_2. Generally speaking, would you say the following things are heading in the right direction, or are they off on the wrong track? Immigration policy",
        )
        .expect("terms should parse");

        assert!(terms.features.contains("immigration"));
        assert!(terms.features.contains("policy"));
        assert!(!terms.features.contains("economy"));
    }

    #[test]
    fn does_not_chain_same_prompt_different_issue_batteries() {
        let mut observations = vec![
            observation(
                "AB2_2. Generally speaking, would you say the following things are heading in the right direction, or are they off on the wrong track? Immigration policy",
            ),
            observation(
                "AB2_8. Generally speaking, would you say the following things are heading in the right direction, or are they off on the wrong track? American foreign policy",
            ),
        ];

        cluster_headline_observations(&mut observations);

        assert_ne!(observations[0].topic_id, observations[1].topic_id);
    }

    #[test]
    fn does_not_cluster_different_answer_intents() {
        let mut observations = vec![
            observation(
                "Do you think that US military action in Iran has been more of a success, or more of a failure?",
            ),
            observation("Do you support or oppose the U.S. military action against Iran?"),
        ];

        cluster_headline_observations(&mut observations);

        assert_ne!(observations[0].topic_id, observations[1].topic_id);
    }

    fn observation(question: &str) -> TopicObservation {
        TopicObservation {
            id: question.to_string(),
            topic_id: "headline-candidate-test".to_string(),
            topic_label: question.to_string(),
            source: TopicSource {
                id: "test".to_string(),
                name: "Test".to_string(),
            },
            source_collection: "Test".to_string(),
            source_subtitle: None,
            question_title: question.to_string(),
            prompt: question.to_string(),
            poll_date: None,
            compatibility: Compatibility::RollupCompatible,
            demographics: Vec::new(),
        }
    }
}
