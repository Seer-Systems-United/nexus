use crate::sources::Scope;
use crate::topics::catalog;
use crate::topics::nlp;
use crate::topics::service;
use crate::topics::types::{Compatibility, TopicObservation};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::{Error as IoError, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

type DynError = Box<dyn Error + Send + Sync>;

const INDEX_VERSION: u32 = 1;
const DEFAULT_INDEX_PATH: &str = "data/topics/question-index.json";
const DEFAULT_LLM_ENDPOINT: &str = "http://127.0.0.1:11434/v1/chat/completions";
const DEFAULT_LLM_MODEL: &str = "qwen3:0.6b";
const MIN_APPLY_CONFIDENCE: f32 = 0.55;

const SYSTEM_PROMPT: &str = r#"You classify polling questions into canonical pooling topics.

Return only one JSON object with this shape:
{
  "canonical_topic_id": "headline-iran-military-action-support",
  "canonical_label": "Iran Military Action Support",
  "intent": "support_oppose",
  "subject": ["iran", "military_action"],
  "confidence": 0.0,
  "exclude_reason": null
}

Rules:
- The topic id must include both the subject and the answer intent. Avoid vague ids such as "headline-economy-opinion".
- If the question is an item in a battery, classify the item subject at the end of the question, not the shared battery wording.
- Do not group different answer intents together. Support/oppose, approve/disapprove, favorable/unfavorable, right/wrong direction, success/failure, vote choice, and importance are separate intents.
- Keep stable recurring topics aligned with existing ids when obvious: presidential-approval, right-direction, generic-ballot, important-problem, economy-approval, inflation-approval, immigration-approval, foreign-policy-approval, trump-favorability.
- Use exclude_reason for demographics, methodology text, source labels, duplicate helper rows, empty/ambiguous text, or questions that cannot be pooled by answer wording.
- Confidence should be 0 to 1.
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QuestionIndex {
    #[serde(default = "default_index_version")]
    pub(crate) version: u32,
    #[serde(default)]
    pub(crate) records: Vec<QuestionEnrichment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QuestionEnrichment {
    pub(crate) question_fingerprint: String,
    pub(crate) source: String,
    pub(crate) poll_date: Option<String>,
    pub(crate) source_collection: String,
    pub(crate) question_title: String,
    pub(crate) prompt: String,
    #[serde(default)]
    pub(crate) answer_labels: Vec<String>,
    pub(crate) canonical_topic_id: String,
    pub(crate) canonical_label: String,
    pub(crate) intent: String,
    #[serde(default)]
    pub(crate) subject: Vec<String>,
    pub(crate) confidence: f32,
    pub(crate) model: String,
    pub(crate) review_status: String,
    pub(crate) exclude_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ClassificationInput {
    pub(crate) question_fingerprint: String,
    pub(crate) source: String,
    pub(crate) poll_date: Option<String>,
    pub(crate) source_collection: String,
    pub(crate) question_title: String,
    pub(crate) prompt: String,
    pub(crate) answer_labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ClassificationOutput {
    #[serde(default)]
    pub(crate) canonical_topic_id: String,
    #[serde(default)]
    pub(crate) canonical_label: String,
    #[serde(default)]
    pub(crate) intent: String,
    #[serde(default)]
    pub(crate) subject: Vec<String>,
    #[serde(default)]
    pub(crate) confidence: f32,
    #[serde(default)]
    pub(crate) exclude_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct CommandPayload<'a> {
    system_prompt: &'a str,
    input: &'a ClassificationInput,
}

#[derive(Debug)]
struct EnrichmentArgs {
    scope: Scope,
    index_path: PathBuf,
    refresh: bool,
    dry_run: bool,
    limit: Option<usize>,
}

#[async_trait::async_trait]
trait TopicClassifier {
    async fn classify(&self, input: &ClassificationInput)
    -> Result<ClassificationOutput, DynError>;
    fn model_name(&self) -> &str;
}

#[derive(Debug, Clone)]
struct OpenAiChatClassifier {
    client: reqwest::Client,
    endpoint: String,
    model: String,
    api_key: Option<String>,
}

#[derive(Debug, Clone)]
struct CommandClassifier {
    command: Vec<String>,
    model: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct ChatMessage {
    content: String,
}

pub(crate) async fn run_cli(args: Vec<String>) -> Result<(), DynError> {
    if args
        .iter()
        .any(|arg| matches!(arg.as_str(), "--help" | "-h"))
    {
        println!("{}", usage());
        return Ok(());
    }

    let args = EnrichmentArgs::parse(args)?;
    let classifier = classifier_from_env()?;
    let mapped = service::collect_unenriched_source_data(args.scope).await;

    for warning in &mapped.warnings {
        eprintln!("warning: {warning}");
    }

    let mut index = load_index_from_path(&args.index_path)?;
    let mut indexed_records = index
        .records
        .into_iter()
        .map(|record| (record.question_fingerprint.clone(), record))
        .collect::<HashMap<_, _>>();

    let mut inputs = classification_inputs(&mapped.observations, &indexed_records, args.refresh);
    if let Some(limit) = args.limit {
        inputs.truncate(limit);
    }

    if inputs.is_empty() {
        println!(
            "topic enrichment: no new candidate questions for {}",
            args.scope
        );
        return Ok(());
    }

    println!(
        "topic enrichment: classifying {} candidate questions with {}",
        inputs.len(),
        classifier.model_name()
    );

    let mut classified = 0usize;
    for input in inputs {
        let output = classifier.classify(&input).await?;
        indexed_records.insert(
            input.question_fingerprint.clone(),
            QuestionEnrichment::from_classification(input, output, classifier.model_name()),
        );
        classified += 1;
    }

    index = QuestionIndex {
        version: INDEX_VERSION,
        records: indexed_records.into_values().collect(),
    };
    index.records.sort_by(|left, right| {
        left.question_fingerprint
            .cmp(&right.question_fingerprint)
            .then_with(|| left.source.cmp(&right.source))
    });

    if args.dry_run {
        println!("topic enrichment: dry run complete, index was not written");
    } else {
        save_index_to_path(&args.index_path, &index)?;
        println!(
            "topic enrichment: wrote {} records to {}",
            index.records.len(),
            args.index_path.display()
        );
    }

    println!("topic enrichment: classified {classified} candidate questions");
    Ok(())
}

pub(crate) fn apply_index_to_observations(
    observations: &mut [TopicObservation],
) -> Result<usize, DynError> {
    let index = load_index()?;
    if index.records.is_empty() {
        return Ok(0);
    }

    let records = index
        .records
        .iter()
        .map(|record| (record.question_fingerprint.as_str(), record))
        .collect::<HashMap<_, _>>();
    let mut applied = 0usize;

    for observation in observations
        .iter_mut()
        .filter(|observation| observation.topic_id.starts_with("headline-candidate-"))
    {
        let fingerprint = observation_fingerprint(observation);
        let Some(record) = records.get(fingerprint.as_str()) else {
            continue;
        };
        let Some(topic_id) = applicable_topic_id(record) else {
            continue;
        };

        observation.topic_id = topic_id.clone();
        observation.topic_label = applicable_label(record, &topic_id);
        observation.compatibility = Compatibility::RollupCompatible;
        applied += 1;
    }

    Ok(applied)
}

fn classification_inputs(
    observations: &[TopicObservation],
    indexed_records: &HashMap<String, QuestionEnrichment>,
    refresh: bool,
) -> Vec<ClassificationInput> {
    let mut seen = HashSet::new();
    let mut inputs = Vec::new();

    for observation in observations {
        if !observation.topic_id.starts_with("headline-candidate-") {
            continue;
        }

        let input = classification_input_from_observation(observation);
        if !refresh && indexed_records.contains_key(&input.question_fingerprint) {
            continue;
        }
        if seen.insert(input.question_fingerprint.clone()) {
            inputs.push(input);
        }
    }

    inputs.sort_by(|left, right| left.question_fingerprint.cmp(&right.question_fingerprint));
    inputs
}

fn classification_input_from_observation(observation: &TopicObservation) -> ClassificationInput {
    ClassificationInput {
        question_fingerprint: observation_fingerprint(observation),
        source: observation.source.id.clone(),
        poll_date: observation.poll_date.clone(),
        source_collection: observation.source_collection.clone(),
        question_title: observation.question_title.clone(),
        prompt: observation.prompt.clone(),
        answer_labels: answer_labels_from_observation(observation),
    }
}

fn observation_fingerprint(observation: &TopicObservation) -> String {
    let question_key = nlp::normalized_question_key(&observation_question_text(observation));
    let answers = answer_labels_from_observation(observation).join("|");
    short_hash(&format!("v1|{question_key}|{answers}"))
}

fn observation_question_text(observation: &TopicObservation) -> String {
    let title = observation.question_title.trim();
    let prompt = observation.prompt.trim();

    if prompt.is_empty() || title.eq_ignore_ascii_case(prompt) || title.contains(prompt) {
        title.to_string()
    } else if title.is_empty() {
        prompt.to_string()
    } else {
        format!("{title}: {prompt}")
    }
}

fn answer_labels_from_observation(observation: &TopicObservation) -> Vec<String> {
    let mut answers = HashSet::new();

    for demographic in &observation.demographics {
        for answer in &demographic.answers {
            answers.insert(format!("{}: {}", answer.id, answer.label));
        }
    }

    let mut answers = answers.into_iter().collect::<Vec<_>>();
    answers.sort();
    answers
}

fn load_index() -> Result<QuestionIndex, DynError> {
    load_index_from_path(&index_path_from_env())
}

fn load_index_from_path(path: &Path) -> Result<QuestionIndex, DynError> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(QuestionIndex::default()),
        Err(error) => return Err(Box::new(error)),
    };

    let index = serde_json::from_str::<QuestionIndex>(&content)?;
    Ok(index)
}

fn save_index_to_path(path: &Path, index: &QuestionIndex) -> Result<(), DynError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(index)?;
    fs::write(path, format!("{content}\n"))?;
    Ok(())
}

fn index_path_from_env() -> PathBuf {
    std::env::var("NEXUS_TOPIC_INDEX_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_INDEX_PATH))
}

fn applicable_topic_id(record: &QuestionEnrichment) -> Option<String> {
    if record.exclude_reason.is_some() || record.confidence < MIN_APPLY_CONFIDENCE {
        return None;
    }

    let raw_id = if record.canonical_topic_id.trim().is_empty() {
        record.canonical_label.trim()
    } else {
        record.canonical_topic_id.trim()
    };
    let mut topic_id = slug_id(raw_id);

    if topic_id.is_empty() {
        return None;
    }
    if catalog::stable_topic(&topic_id).is_some() {
        return Some(topic_id);
    }
    if !topic_id.starts_with("headline-") {
        topic_id = format!("headline-{topic_id}");
    }

    Some(topic_id)
}

fn applicable_label(record: &QuestionEnrichment, topic_id: &str) -> String {
    let label = record.canonical_label.trim();
    if !label.is_empty() {
        return label.to_string();
    }

    label_from_topic_id(topic_id)
}

fn classifier_from_env() -> Result<Box<dyn TopicClassifier + Send + Sync>, DynError> {
    if let Ok(command) = std::env::var("NEXUS_TOPIC_LLM_COMMAND")
        && !command.trim().is_empty()
    {
        return Ok(Box::new(CommandClassifier::new(command)?));
    }

    let endpoint =
        std::env::var("NEXUS_TOPIC_LLM_ENDPOINT").unwrap_or_else(|_| DEFAULT_LLM_ENDPOINT.into());
    let model = std::env::var("NEXUS_TOPIC_LLM_MODEL").unwrap_or_else(|_| DEFAULT_LLM_MODEL.into());
    let api_key = std::env::var("NEXUS_TOPIC_LLM_API_KEY")
        .ok()
        .filter(|value| !value.trim().is_empty());

    Ok(Box::new(OpenAiChatClassifier {
        client: reqwest::Client::new(),
        endpoint,
        model,
        api_key,
    }))
}

#[async_trait::async_trait]
impl TopicClassifier for OpenAiChatClassifier {
    async fn classify(
        &self,
        input: &ClassificationInput,
    ) -> Result<ClassificationOutput, DynError> {
        let request_body = serde_json::json!({
            "model": self.model,
            "temperature": 0,
            "response_format": { "type": "json_object" },
            "messages": [
                { "role": "system", "content": SYSTEM_PROMPT },
                { "role": "user", "content": serde_json::to_string(input)? }
            ]
        });

        let mut request = self
            .client
            .post(&self.endpoint)
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_vec(&request_body)?);
        if let Some(api_key) = &self.api_key {
            request = request.bearer_auth(api_key);
        }

        let response = request.send().await.map_err(|error| {
            IoError::other(format!(
                "failed to reach topic LLM endpoint {} for model {}: {error}. \
                 Start a local OpenAI-compatible server, for example `ollama serve`, \
                 make sure the model exists with `ollama pull {}`, or set \
                 NEXUS_TOPIC_LLM_ENDPOINT / NEXUS_TOPIC_LLM_MODEL / NEXUS_TOPIC_LLM_COMMAND.",
                self.endpoint, self.model, self.model
            ))
        })?;
        let status = response.status();
        let response_text = response.text().await?;
        if !status.is_success() {
            return Err(Box::new(IoError::other(format!(
                "topic LLM endpoint {} returned HTTP {status} for model {}: {}",
                self.endpoint,
                self.model,
                response_text.trim()
            ))));
        }

        let response = serde_json::from_str::<ChatCompletionResponse>(&response_text)?;
        let content = response
            .choices
            .first()
            .ok_or_else(|| IoError::new(ErrorKind::InvalidData, "LLM response had no choices"))?
            .message
            .content
            .clone();

        parse_classifier_output(&content)
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

impl CommandClassifier {
    fn new(command: String) -> Result<Self, DynError> {
        let command = command
            .split_whitespace()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        if command.is_empty() {
            return Err(Box::new(IoError::new(
                ErrorKind::InvalidInput,
                "NEXUS_TOPIC_LLM_COMMAND was empty",
            )));
        }

        let model = std::env::var("NEXUS_TOPIC_LLM_MODEL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| command.join(" "));

        Ok(Self { command, model })
    }
}

#[async_trait::async_trait]
impl TopicClassifier for CommandClassifier {
    async fn classify(
        &self,
        input: &ClassificationInput,
    ) -> Result<ClassificationOutput, DynError> {
        let mut child = Command::new(&self.command[0])
            .args(&self.command[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            let payload = serde_json::to_vec(&CommandPayload {
                system_prompt: SYSTEM_PROMPT,
                input,
            })?;
            stdin.write_all(&payload)?;
        }

        let output = child.wait_with_output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Box::new(IoError::other(format!(
                "topic LLM command failed: {stderr}"
            ))));
        }

        let stdout = String::from_utf8(output.stdout)?;
        parse_classifier_output(&stdout)
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

fn parse_classifier_output(raw: &str) -> Result<ClassificationOutput, DynError> {
    let json_text = extract_json_object(raw).ok_or_else(|| {
        IoError::new(
            ErrorKind::InvalidData,
            "LLM response did not contain a JSON object",
        )
    })?;
    let mut output = serde_json::from_str::<ClassificationOutput>(json_text)?;

    output.confidence = normalize_confidence(output.confidence);
    output.canonical_topic_id = output.canonical_topic_id.trim().to_string();
    output.canonical_label = output.canonical_label.trim().to_string();
    output.intent = output.intent.trim().to_string();
    output.subject = normalized_subject(output.subject);
    output.exclude_reason = output
        .exclude_reason
        .map(|reason| reason.trim().to_string())
        .filter(|reason| !reason.is_empty());

    if output.canonical_topic_id.is_empty() && !output.canonical_label.is_empty() {
        output.canonical_topic_id = format!("headline-{}", slug_id(&output.canonical_label));
    }
    if output.canonical_topic_id.is_empty() && output.exclude_reason.is_none() {
        output.exclude_reason = Some("model returned no canonical topic id".to_string());
    }

    Ok(output)
}

fn extract_json_object(raw: &str) -> Option<&str> {
    let after_reasoning = raw.rsplit("</think>").next().unwrap_or(raw).trim();
    let start = after_reasoning.find('{')?;
    let end = after_reasoning.rfind('}')?;

    (start <= end).then(|| &after_reasoning[start..=end])
}

fn normalize_confidence(confidence: f32) -> f32 {
    let confidence = if confidence > 1.0 && confidence <= 100.0 {
        confidence / 100.0
    } else {
        confidence
    };

    confidence.clamp(0.0, 1.0)
}

fn normalized_subject(subject: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut normalized = Vec::new();

    for value in subject {
        let value = slug_id(&value);
        if !value.is_empty() && seen.insert(value.clone()) {
            normalized.push(value);
        }
    }

    normalized
}

fn short_hash(input: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in input.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

fn slug_id(input: &str) -> String {
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

fn label_from_topic_id(topic_id: &str) -> String {
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

impl QuestionEnrichment {
    fn from_classification(
        input: ClassificationInput,
        output: ClassificationOutput,
        model: &str,
    ) -> Self {
        let review_status = if output.exclude_reason.is_some() {
            "excluded"
        } else if output.confidence < MIN_APPLY_CONFIDENCE {
            "needs-review"
        } else {
            "accepted"
        };

        Self {
            question_fingerprint: input.question_fingerprint,
            source: input.source,
            poll_date: input.poll_date,
            source_collection: input.source_collection,
            question_title: input.question_title,
            prompt: input.prompt,
            answer_labels: input.answer_labels,
            canonical_topic_id: output.canonical_topic_id,
            canonical_label: output.canonical_label,
            intent: output.intent,
            subject: output.subject,
            confidence: output.confidence,
            model: model.to_string(),
            review_status: review_status.to_string(),
            exclude_reason: output.exclude_reason,
        }
    }
}

impl EnrichmentArgs {
    fn parse(args: Vec<String>) -> Result<Self, DynError> {
        let mut scope = None;
        let mut count = None;
        let mut index_path = index_path_from_env();
        let mut refresh = false;
        let mut dry_run = false;
        let mut limit = None;

        let mut index = 0usize;
        while index < args.len() {
            let arg = &args[index];
            match arg.as_str() {
                "--help" | "-h" => return Err(Box::new(IoError::other(usage()))),
                "--refresh" => refresh = true,
                "--dry-run" => dry_run = true,
                "--scope" => {
                    index += 1;
                    scope = Some(required_arg(&args, index, "--scope")?);
                }
                "--count" | "--n" => {
                    index += 1;
                    count = Some(parse_u32(required_arg(&args, index, arg)?, arg)?);
                }
                "--index" => {
                    index += 1;
                    index_path = PathBuf::from(required_arg(&args, index, "--index")?);
                }
                "--limit" => {
                    index += 1;
                    limit = Some(parse_usize(
                        required_arg(&args, index, "--limit")?,
                        "--limit",
                    )?);
                }
                value if value.starts_with("--scope=") => {
                    scope = Some(value.trim_start_matches("--scope=").to_string());
                }
                value if value.starts_with("--count=") => {
                    count = Some(parse_u32(value.trim_start_matches("--count="), "--count")?);
                }
                value if value.starts_with("--n=") => {
                    count = Some(parse_u32(value.trim_start_matches("--n="), "--n")?);
                }
                value if value.starts_with("--index=") => {
                    index_path = PathBuf::from(value.trim_start_matches("--index="));
                }
                value if value.starts_with("--limit=") => {
                    limit = Some(parse_usize(
                        value.trim_start_matches("--limit="),
                        "--limit",
                    )?);
                }
                _ => {
                    return Err(Box::new(IoError::new(
                        ErrorKind::InvalidInput,
                        format!("unsupported enrich-topics argument: {arg}\n{}", usage()),
                    )));
                }
            }
            index += 1;
        }

        Ok(Self {
            scope: parse_scope(scope.as_deref(), count)?,
            index_path,
            refresh,
            dry_run,
            limit,
        })
    }
}

impl Default for QuestionIndex {
    fn default() -> Self {
        Self {
            version: INDEX_VERSION,
            records: Vec::new(),
        }
    }
}

fn default_index_version() -> u32 {
    INDEX_VERSION
}

fn required_arg(args: &[String], index: usize, flag: &str) -> Result<String, DynError> {
    args.get(index).cloned().ok_or_else(|| {
        Box::new(IoError::new(
            ErrorKind::InvalidInput,
            format!("{flag} requires a value"),
        )) as DynError
    })
}

fn parse_u32(value: impl AsRef<str>, flag: &str) -> Result<u32, DynError> {
    let parsed = value.as_ref().parse::<u32>().map_err(|error| {
        IoError::new(
            ErrorKind::InvalidInput,
            format!("{flag} must be a positive integer: {error}"),
        )
    })?;

    if parsed == 0 {
        return Err(Box::new(IoError::new(
            ErrorKind::InvalidInput,
            format!("{flag} must be greater than zero"),
        )));
    }

    Ok(parsed)
}

fn parse_usize(value: impl AsRef<str>, flag: &str) -> Result<usize, DynError> {
    let parsed = value.as_ref().parse::<usize>().map_err(|error| {
        IoError::new(
            ErrorKind::InvalidInput,
            format!("{flag} must be a positive integer: {error}"),
        )
    })?;

    if parsed == 0 {
        return Err(Box::new(IoError::new(
            ErrorKind::InvalidInput,
            format!("{flag} must be greater than zero"),
        )));
    }

    Ok(parsed)
}

fn parse_scope(scope: Option<&str>, count: Option<u32>) -> Result<Scope, DynError> {
    let normalized = scope
        .unwrap_or("last_entries")
        .trim()
        .to_ascii_lowercase()
        .replace('-', "_");
    let count = count.unwrap_or(5);

    match normalized.as_str() {
        "" | "latest" => Ok(Scope::Latest),
        "last_n_entries" | "last_entries" | "entries" => Ok(Scope::LastNEntries(count)),
        "last_days" | "days" => Ok(Scope::LastDays(count)),
        "last_weeks" | "weeks" => Ok(Scope::LastWeeks(count)),
        "last_months" | "months" => Ok(Scope::LastMonths(count)),
        "last_years" | "years" => Ok(Scope::LastYears(count)),
        _ => Err(Box::new(IoError::new(
            ErrorKind::InvalidInput,
            format!("unsupported enrich-topics scope: {normalized}"),
        ))),
    }
}

fn usage() -> String {
    [
        "usage: cargo run -- enrich-topics [--scope latest|last_entries|last_days|last_weeks|last_months|last_years] [--count N]",
        "       [--index data/topics/question-index.json] [--refresh] [--dry-run] [--limit N]",
        "",
        "model providers:",
        "  OpenAI-compatible local endpoint: NEXUS_TOPIC_LLM_ENDPOINT, NEXUS_TOPIC_LLM_MODEL",
        "  Command/Burn runner: NEXUS_TOPIC_LLM_COMMAND reads JSON from stdin and returns JSON to stdout",
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{
        MIN_APPLY_CONFIDENCE, QuestionEnrichment, applicable_topic_id, parse_classifier_output,
        parse_scope,
    };
    use crate::sources::Scope;

    #[test]
    fn parses_classifier_json_after_reasoning_text() {
        let output = parse_classifier_output(
            r#"<think>not part of the answer</think>
            ```json
            {
              "canonical_topic_id": "iran military action support",
              "canonical_label": "Iran Military Action Support",
              "intent": "support_oppose",
              "subject": ["Iran", "Military Action"],
              "confidence": 84,
              "exclude_reason": null
            }
            ```"#,
        )
        .unwrap();

        assert_eq!(output.canonical_topic_id, "iran military action support");
        assert_eq!(output.subject, vec!["iran", "military-action"]);
        assert!((output.confidence - 0.84).abs() < f32::EPSILON);
    }

    #[test]
    fn applies_only_accepted_or_stable_topic_ids() {
        let headline_record = record("iran military action support", MIN_APPLY_CONFIDENCE);
        assert_eq!(
            applicable_topic_id(&headline_record).as_deref(),
            Some("headline-iran-military-action-support")
        );

        let stable = record("presidential-approval", MIN_APPLY_CONFIDENCE);
        assert_eq!(
            applicable_topic_id(&stable).as_deref(),
            Some("presidential-approval")
        );

        let low_confidence = record("iran military action support", MIN_APPLY_CONFIDENCE - 0.01);
        assert!(applicable_topic_id(&low_confidence).is_none());
    }

    #[test]
    fn parses_default_enrichment_scope() {
        assert_eq!(parse_scope(None, None).unwrap(), Scope::LastNEntries(5));
        assert_eq!(
            parse_scope(Some("last_days"), Some(30)).unwrap(),
            Scope::LastDays(30)
        );
    }

    fn record(topic_id: &str, confidence: f32) -> QuestionEnrichment {
        QuestionEnrichment {
            question_fingerprint: "fingerprint".to_string(),
            source: "source".to_string(),
            poll_date: None,
            source_collection: "collection".to_string(),
            question_title: "question".to_string(),
            prompt: "prompt".to_string(),
            answer_labels: Vec::new(),
            canonical_topic_id: topic_id.to_string(),
            canonical_label: String::new(),
            intent: "support_oppose".to_string(),
            subject: Vec::new(),
            confidence,
            model: "test".to_string(),
            review_status: "accepted".to_string(),
            exclude_reason: None,
        }
    }
}
