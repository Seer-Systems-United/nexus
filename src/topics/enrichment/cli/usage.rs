pub(super) fn usage() -> String {
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
