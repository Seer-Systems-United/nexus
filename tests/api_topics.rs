use nexus::api::topics::{TopicQuery, parse_topic_scope};
use nexus::sources::Scope;

#[test]
fn defaults_to_latest_scope() {
    let scope = parse_topic_scope(&TopicQuery {
        scope: None,
        count: None,
        n: None,
    })
    .unwrap();

    assert_eq!(scope, Scope::Latest);
}

#[test]
fn parses_counted_scopes() {
    let scope = parse_topic_scope(&TopicQuery {
        scope: Some("last_entries".to_string()),
        count: Some(3),
        n: None,
    })
    .unwrap();

    assert_eq!(scope, Scope::LastNEntries(3));
}
