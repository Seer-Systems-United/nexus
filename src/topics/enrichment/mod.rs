mod classifier;
mod cli;
mod index;
mod models;
mod text;

pub use classifier::parse_classifier_output;
pub use cli::{parse_scope, run_cli};
pub use index::{applicable_topic_id, apply_index_to_observations};
pub use models::{ClassificationInput, ClassificationOutput, QuestionEnrichment, QuestionIndex};

use std::error::Error;

pub type DynError = Box<dyn Error + Send + Sync>;

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
