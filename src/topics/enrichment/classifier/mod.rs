mod chat;
mod command;
mod parse;

pub use parse::parse_classifier_output;

use super::{
    ClassificationInput, ClassificationOutput, DEFAULT_LLM_ENDPOINT, DEFAULT_LLM_MODEL, DynError,
};
use chat::OpenAiChatClassifier;
use command::CommandClassifier;

#[async_trait::async_trait]
pub(super) trait TopicClassifier {
    async fn classify(&self, input: &ClassificationInput)
    -> Result<ClassificationOutput, DynError>;
    fn model_name(&self) -> &str;
}

pub(super) fn classifier_from_env() -> Result<Box<dyn TopicClassifier + Send + Sync>, DynError> {
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

    Ok(Box::new(OpenAiChatClassifier::new(
        endpoint, model, api_key,
    )))
}
