use super::{TopicClassifier, parse_classifier_output};
use crate::topics::enrichment::{
    ClassificationInput, ClassificationOutput, DynError, SYSTEM_PROMPT,
};
use serde::Serialize;
use std::io::{Error as IoError, ErrorKind, Write};
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub(super) struct CommandClassifier {
    command: Vec<String>,
    model: String,
}

#[derive(Debug, Clone, Serialize)]
struct CommandPayload<'a> {
    system_prompt: &'a str,
    input: &'a ClassificationInput,
}

impl CommandClassifier {
    pub(super) fn new(command: String) -> Result<Self, DynError> {
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

        parse_classifier_output(&String::from_utf8(output.stdout)?)
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}
