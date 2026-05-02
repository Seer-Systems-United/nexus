//! # Chat-based topic classifier
//!
//! Uses LLM chat API to classify polling questions into topics.

use super::{TopicClassifier, parse_classifier_output};
use crate::topics::enrichment::{
    ClassificationInput, ClassificationOutput, DynError, SYSTEM_PROMPT,
};
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use std::io::{Error as IoError, ErrorKind};

#[derive(Debug, Clone)]
pub(super) struct OpenAiChatClassifier {
    client: reqwest::Client,
    endpoint: String,
    model: String,
    api_key: Option<String>,
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

impl OpenAiChatClassifier {
    pub(super) fn new(endpoint: String, model: String, api_key: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint,
            model,
            api_key,
        }
    }
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
                "failed to reach topic LLM endpoint {} for model {}: {error}",
                self.endpoint, self.model
            ))
        })?;
        let status = response.status();
        let response_text = response.text().await?;
        if !status.is_success() {
            return Err(Box::new(IoError::other(format!(
                "topic LLM endpoint {} returned HTTP {status}: {}",
                self.endpoint,
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
