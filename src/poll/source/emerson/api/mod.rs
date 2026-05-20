pub mod get;
pub mod links;
mod models;
mod parse;
pub mod url;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use tracing::{debug, info, instrument, warn};

use crate::{
    database::{BackendTrait, default_backend},
    poll::{
        Poll,
        location::PollLocation,
        source::emerson::api::{
            get::api_get, links::find_spreadsheet_links, parse::parse_full_crosstabs,
            url::get_latest_poll_url,
        },
    },
};

#[instrument(level = "info", skip_all)]
pub fn latest_poll() -> Poll {
    let posts = match api_get(&get_latest_poll_url()) {
        Ok(posts) => posts,
        Err(error) => {
            warn!(%error, "failed to fetch latest Emerson post");
            return empty_poll(
                Utc::now(),
                PollLocation::Other {
                    label: "Unknown Emerson poll".to_string(),
                },
            );
        }
    };

    let Some(post) = posts.first() else {
        warn!("Emerson latest-post endpoint returned no posts");
        return empty_poll(
            Utc::now(),
            PollLocation::Other {
                label: "Unknown Emerson poll".to_string(),
            },
        );
    };

    let published_timestamp = parse_post_timestamp(&post.date).unwrap_or_else(|| {
        warn!(date = %post.date, "failed to parse Emerson post date; using current time");
        Utc::now()
    });
    let location = location_from_post(post);

    let Some(sheet_link) = find_spreadsheet_links(&posts).into_iter().next() else {
        warn!(
            post_id = post.id,
            "Emerson post did not contain a Google Sheet link"
        );
        return empty_poll(published_timestamp, location);
    };

    let csv = sheet_link.get_as_csv(Some("full crosstabs"));
    let questions = parse_full_crosstabs(&csv);

    info!(
        post_id = post.id,
        questions_len = questions.len(),
        published_timestamp = %published_timestamp,
        "parsed latest Emerson poll"
    );

    Poll {
        questions,
        published_timestamp,
        location,
    }
}

#[instrument(level = "info", skip_all)]
pub fn has_new_poll() -> bool {
    let posts = match api_get(&get_latest_poll_url()) {
        Ok(posts) => posts,
        Err(error) => {
            warn!(%error, "failed to fetch latest Emerson post");
            return false;
        }
    };

    let Some(post) = posts.first() else {
        warn!("Emerson latest-post endpoint returned no posts");
        return false;
    };

    let Some(latest) = parse_post_timestamp(&post.date) else {
        warn!(date = %post.date, "failed to parse Emerson post date");
        return false;
    };

    debug!(post_id = post.id, %latest, "checking Emerson poll freshness");

    match default_backend().and_then(|backend| backend.poll_exists_by_timestamp(latest)) {
        Ok(exists) => !exists,
        Err(error) => {
            warn!(%error, "failed to check Emerson poll existence");
            false
        }
    }
}

fn empty_poll(published_timestamp: DateTime<Utc>, location: PollLocation) -> Poll {
    Poll {
        questions: Vec::new(),
        published_timestamp,
        location,
    }
}

fn parse_post_timestamp(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .ok()
        .or_else(|| {
            NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S")
                .ok()
                .map(|timestamp| Utc.from_utc_datetime(&timestamp))
        })
}

fn location_from_post(post: &models::Post) -> PollLocation {
    let title = clean_text(&post.title.rendered);
    let haystack = format!("{} {}", title.to_lowercase(), post.link.to_lowercase());

    if haystack.contains("national") {
        return PollLocation::National;
    }

    for state in US_STATES {
        if haystack.contains(&state.to_lowercase()) {
            return PollLocation::State {
                state: (*state).to_string(),
            };
        }
    }

    PollLocation::Other { label: title }
}

fn clean_text(value: &str) -> String {
    value
        .replace("&nbsp;", " ")
        .replace("&#160;", " ")
        .replace("&amp;", "&")
        .replace('\u{a0}', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

const US_STATES: &[&str] = &[
    "Alabama",
    "Alaska",
    "Arizona",
    "Arkansas",
    "California",
    "Colorado",
    "Connecticut",
    "Delaware",
    "Florida",
    "Georgia",
    "Hawaii",
    "Idaho",
    "Illinois",
    "Indiana",
    "Iowa",
    "Kansas",
    "Kentucky",
    "Louisiana",
    "Maine",
    "Maryland",
    "Massachusetts",
    "Michigan",
    "Minnesota",
    "Mississippi",
    "Missouri",
    "Montana",
    "Nebraska",
    "Nevada",
    "New Hampshire",
    "New Jersey",
    "New Mexico",
    "New York",
    "North Carolina",
    "North Dakota",
    "Ohio",
    "Oklahoma",
    "Oregon",
    "Pennsylvania",
    "Rhode Island",
    "South Carolina",
    "South Dakota",
    "Tennessee",
    "Texas",
    "Utah",
    "Vermont",
    "Virginia",
    "Washington",
    "West Virginia",
    "Wisconsin",
    "Wyoming",
];

#[cfg(test)]
mod tests {
    use super::get::api_get;
    use super::links::find_spreadsheet_links;
    use super::url::{get_latest_n_polls_url, get_latest_poll_url};

    #[test]
    fn test_get_latest_n_polls_url() {
        let url = get_latest_n_polls_url(5);
        assert_eq!(
            url,
            "https://emersoncollegepolling.com/wp-json/wp/v2/posts?per_page=5&orderby=date&order=desc&_fields=id,date,link,title.rendered,content.rendered"
        );
    }

    #[test]
    fn test_get_latest_poll_url() {
        let url = get_latest_poll_url();
        assert_eq!(
            url,
            "https://emersoncollegepolling.com/wp-json/wp/v2/posts?per_page=1&orderby=date&order=desc&_fields=id,date,link,title.rendered,content.rendered"
        );
    }

    #[test]
    #[ignore = "hits Emerson and Google Sheets over the network"]
    fn test_api_get() {
        let url = get_latest_poll_url();
        let call = api_get(&url);

        assert!(call.is_ok(), "API call failed: {:?}", call.err());
        let posts = call.expect("Failed to unwrap posts from API response");

        dbg!(&posts);

        assert_eq!(posts.len(), 1);

        let links = find_spreadsheet_links(&posts);
        dbg!(&links);

        let csv = links[0].get_as_csv(Some("full crosstabs"));

        dbg!(&csv);
    }
}
