use crate::topics::types::{TopicStatus, TopicSummary};

#[derive(Debug, Clone, Copy)]
pub(crate) struct TopicDefinition {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub endpoint: Option<&'static str>,
}

pub(crate) const PRESIDENTIAL_APPROVAL_ID: &str = "presidential-approval";
pub(crate) const RIGHT_DIRECTION_ID: &str = "right-direction";
pub(crate) const GENERIC_BALLOT_ID: &str = "generic-ballot";
pub(crate) const IMPORTANT_PROBLEM_ID: &str = "important-problem";
pub(crate) const ECONOMY_APPROVAL_ID: &str = "economy-approval";
pub(crate) const INFLATION_APPROVAL_ID: &str = "inflation-approval";
pub(crate) const IMMIGRATION_APPROVAL_ID: &str = "immigration-approval";
pub(crate) const FOREIGN_POLICY_APPROVAL_ID: &str = "foreign-policy-approval";
pub(crate) const TRUMP_FAVORABILITY_ID: &str = "trump-favorability";

const STABLE_TOPICS: &[TopicDefinition] = &[
    TopicDefinition {
        id: PRESIDENTIAL_APPROVAL_ID,
        label: "Presidential approval",
        description: "Approval or disapproval of the president's job performance.",
        endpoint: Some("/api/v1/topics/presidential-approval"),
    },
    TopicDefinition {
        id: RIGHT_DIRECTION_ID,
        label: "Right direction / wrong track",
        description: "Whether the country is headed in the right direction or off on the wrong track.",
        endpoint: Some("/api/v1/topics/right-direction"),
    },
    TopicDefinition {
        id: GENERIC_BALLOT_ID,
        label: "Generic congressional ballot",
        description: "Democratic versus Republican congressional vote preference.",
        endpoint: Some("/api/v1/topics/generic-ballot"),
    },
    TopicDefinition {
        id: IMPORTANT_PROBLEM_ID,
        label: "Most important problem",
        description: "The issue respondents identify as the most important problem facing the country.",
        endpoint: Some("/api/v1/topics/important-problem"),
    },
    TopicDefinition {
        id: ECONOMY_APPROVAL_ID,
        label: "Economy approval",
        description: "Approval or disapproval of the president's handling of the economy.",
        endpoint: None,
    },
    TopicDefinition {
        id: INFLATION_APPROVAL_ID,
        label: "Inflation and cost of living approval",
        description: "Approval or disapproval of the president's handling of inflation, prices, or cost of living.",
        endpoint: None,
    },
    TopicDefinition {
        id: IMMIGRATION_APPROVAL_ID,
        label: "Immigration approval",
        description: "Approval or disapproval of the president's handling of immigration.",
        endpoint: None,
    },
    TopicDefinition {
        id: FOREIGN_POLICY_APPROVAL_ID,
        label: "Foreign policy approval",
        description: "Approval or disapproval of the president's handling of foreign policy.",
        endpoint: None,
    },
    TopicDefinition {
        id: TRUMP_FAVORABILITY_ID,
        label: "Donald Trump favorability",
        description: "Favorable or unfavorable views of Donald Trump.",
        endpoint: None,
    },
];

pub fn stable_topics() -> Vec<TopicSummary> {
    STABLE_TOPICS.iter().map(|topic| topic.summary()).collect()
}

pub(crate) fn stable_topic(id: &str) -> Option<TopicSummary> {
    STABLE_TOPICS
        .iter()
        .find(|topic| topic.id == id)
        .map(TopicDefinition::summary)
}

impl TopicDefinition {
    pub(crate) fn summary(&self) -> TopicSummary {
        TopicSummary {
            id: self.id.to_string(),
            label: self.label.to_string(),
            status: TopicStatus::Stable,
            description: Some(self.description.to_string()),
            endpoint: self.endpoint.map(str::to_string),
        }
    }
}
