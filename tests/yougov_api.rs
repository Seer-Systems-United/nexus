use nexus::poll::question::is_non_question_text;
use nexus::poll::source::yougov::api::parse::{parse_page, parse_pages};
use nexus::poll::source::yougov::api::{models::SurveyApiResponse, models::SurveyResponseItem};
use nexus::poll::{
    response::{demographic::Demographic, unit::Unit},
    response::{demographic::partisan_affiliation::PartisanAffiliation, demographic::sex::Sex},
};

#[test]
fn parses_mixed_yougov_survey_response_items() {
    let json = r#"{
        "data": [
            {
                "cms_document_type": "survey_result",
                "created_at": "2026-05-12T08:00:00Z",
                "description": "The Economist /YouGov Poll May 9 - 11, 2026",
                "doc_type": "cms_document_editorial",
                "filetype": "pdf",
                "id": 32088,
                "language": { "id": 435, "name": "en-us" },
                "primary_category": {
                    "default_short_name": "Politics",
                    "id": 515,
                    "name": "Politics & current affairs",
                    "search_description": "Explore what America thinks with YouGov's popularity rankings, articles and survey results on %category_long_name%.",
                    "seo_title": "%category_long_name% | YouGov",
                    "short_name": "Politics",
                    "slug": "politics"
                },
                "region": { "id": 184, "name": "International" },
                "site": { "id": 3, "name": "yougov.com" },
                "title": "Economist Tables May 11 2026",
                "url": "https://d3nkl3psvxxpe9.cloudfront.net/documents/econTabReport_ONiR6Nb.pdf"
            },
            {
                "doc_type": "result",
                "primary_category": "politics",
                "published_at": "2026-05-09T14:45:00Z",
                "question_id": 1,
                "summary": [
                    { "label": "Completely fair", "value": 23 },
                    { "label": "Completely unfair", "value": 20 }
                ],
                "survey_id": "35e85",
                "survey_uuid": "9f235e85-4b2e-11f1-8624-91e488a41ab2",
                "title": "If a state redraws their districts for the U.S. House of Representatives to favor one party, do you think it is fair or unfair for other states to redraw their districts to favor the other party?",
                "total": 5061
            }
        ],
        "hits": 1944,
        "totals": {
            "categories": { "politics": 1942 },
            "types": { "survey": 1944 }
        }
    }"#;

    let response: SurveyApiResponse = serde_json::from_str(json).unwrap();

    assert_eq!(response.hits, 1944);
    assert_eq!(response.data.len(), 2);

    let SurveyResponseItem::CmsDocumentEditorial(document) = &response.data[0] else {
        panic!("expected editorial document");
    };
    assert_eq!(document.filetype, "pdf");

    let SurveyResponseItem::Result(result) = &response.data[1] else {
        panic!("expected survey result");
    };
    assert_eq!(result.summary[0].label, "Completely fair");
    assert_eq!(result.total, 5061);
}

#[test]
fn default_question_parsing_uses_parallel_pages_serial_split() {
    let pages = vec![
        "Header\nSubheader\nPoll question\nQuestion one?\nYes 52\nNo 48".to_string(),
        "Header\nSubheader\nQuestion two?\nTotal Male Female\nApprove 44% 45% 43%\nDisapprove 50% 49% 51%".to_string(),
        "Table . . . contents\nQuestion skipped?".to_string(),
    ];

    let parallel_outer: Vec<String> = parse_pages(&pages)
        .into_iter()
        .map(|question| question.text)
        .collect();
    let parallel_outer_default: Vec<String> = parse_pages(&pages)
        .into_iter()
        .map(|question| question.text)
        .collect();

    assert_eq!(parallel_outer, parallel_outer_default);
    assert_eq!(parallel_outer, vec!["Question two?"]);
}

#[test]
fn parses_yougov_table_responses() {
    let page = r#"
The Economist/YouGov Poll
May 9 - 11, 2026 - 1549 U.S. Adult Citizens
3. Would You Ever Vote Third Party
Would you ever vote for a candidate for president who was not a Democrat or Republican?
Sex Race Age Education
Total Male Female No degree College grad
Yes 39% 46% 33% 38% 42%
No 24% 23% 24% 24% 22%
Totals 100% 100% 100% 100% 100%
Unweighted N (1,548) (700) (848) (997) (551)
2024 Vote Reg Ideology MAGA Party ID
Total Harris Trump Voters Lib Mod Con Supporter Dem Ind Rep
Yes 39% 43% 39% 42% 38% 49% 37% 31% 31% 50% 34%
No 24% 22% 26% 22% 26% 18% 26% 30% 27% 19% 27%
Totals 100% 100% 100% 100% 100% 100% 100% 100% 100% 100% 101%
Unweighted N (1,548) (610) (540) (1,409) (478) (482) (478) (335) (510) (596) (442)
5
"#;

    let question = parse_page(page).unwrap();

    assert_eq!(
        question.text,
        "Would you ever vote for a candidate for president who was not a Democrat or Republican?"
    );
    assert_eq!(question.responses.len(), 32);

    assert_eq!(question.responses[0].answer.as_ref(), "Yes");
    assert_eq!(question.responses[0].value, 39);
    assert!(matches!(question.responses[0].unit, Unit::Percent));
    assert!(matches!(
        question.responses[0].demographic,
        Demographic::All
    ));

    assert_eq!(question.responses[1].answer.as_ref(), "Yes");
    assert_eq!(question.responses[1].value, 46);
    assert!(matches!(
        question.responses[1].demographic,
        Demographic::Sex { sex: Sex::Male }
    ));

    assert_eq!(question.responses[30].answer.as_ref(), "No");
    assert_eq!(question.responses[30].value, 19);
    assert!(matches!(
        question.responses[30].demographic,
        Demographic::PartisanAffiliation {
            partisan_affiliation: PartisanAffiliation::Independent
        }
    ));
}

#[test]
fn parses_yougov_rows_split_across_pdf_extraction_lines() {
    let page = r#"
9 - 2026 - 1549
16. Trump a Threat to Democracy
Do you think Donald Trump is a threat to democracy?

Sex Race
Age
Education

Total Male Female White Black Hispanic 18-29 30-44 45-64 65+ No degree College grad

Yes
52% 46% 57% 47%

Reg

69% 54% 61%

MAGA

49% 51% 48% 49% 58%

ID

No
38% 46% 30% 44% 13% 34% 26% 34% 44% 45% 39% 36%
Not sure
10% 7% 13% 9% 17% 12% 13% 17% 5% 7% 12% 6%

Totals
100% 99% 100% 100% 99% 100% 100% 100% 100% 100% 100% 100%
Unweighted N (1,549)
(701) (848) (1,011) (198) (233) (313) (405) (508) (323) (998) (551)

2024 Vote
Ideology
Party

Total Harris Trump
Lib Mod
Supporter Dem Ind

Yes
52% 94% 5% 50%

Voters

89% 59%

Con

10% 2% 93% 57%

Rep

8%
No
38% 3% 85% 42% 5% 29% 83% 90% 3% 28% 83%
Not sure
10% 4% 9% 8% 6% 12% 7% 8% 4% 15% 9%

Totals
100% 101% 99% 100% 100% 100% 100% 100% 100% 100% 100%
Unweighted N (1,549)
(610) (541) (1,410) (478) (482) (479) (335) (510) (596) (443)

34
"#;

    let question = parse_page(page).unwrap();

    assert_eq!(
        question.text,
        "Do you think Donald Trump is a threat to democracy?"
    );
    assert_eq!(question.responses.len(), 69);
    assert_eq!(question.responses[0].answer.as_ref(), "Yes");
    assert_eq!(question.responses[0].value, 52);
    assert_eq!(question.responses[1].answer.as_ref(), "Yes");
    assert_eq!(question.responses[1].value, 46);

    let harris_yes = question
        .responses
        .iter()
        .find(|response| {
            response.answer.as_ref() == "Yes"
                && matches!(
                    &response.demographic,
                    Demographic::Other { description } if description.as_ref() == "2024 vote: Harris"
                )
                && response.value == 94
        })
        .expect("expected Harris yes response");

    assert!(matches!(harris_yes.unit, Unit::Percent));
    assert!(
        !question
            .responses
            .iter()
            .any(|response| response.answer.as_ref().ends_with('%'))
    );
}

#[test]
fn strips_table_artifacts_from_question_suffixes() {
    let page = format!(
        "\
The Economist/YouGov Poll
May 9 - 11, 2026 - 1549 U.S. Adult Citizens
15A. Which of these words would you [use / NOT use] to describe Donald Trump? Please check all that apply. {} Honest
that apply Sex Race Age
Total Male Female White Black
Yes 41% 42% 40% 39% 48%
No 50% 49% 51% 52% 42%
",
        '\u{2014}'
    );

    let question = parse_page(&page).unwrap();

    assert_eq!(
        question.text,
        "Which of these words would you [use / NOT use] to describe Donald Trump? Please check all that apply. \u{2014} Honest"
    );
}

#[test]
fn skips_stem_only_matrix_questions() {
    let page = "\
The Economist/YouGov Poll
May 9 - 11, 2026 - 1549 U.S. Adult Citizens
Do you think Donald Trump...
Sex Race Age
Total Male Female White Black
Honest 41% 42% 40% 39% 48%
Dishonest 50% 49% 51% 52% 42%
";

    assert!(parse_page(page).is_none());
}

#[test]
fn skips_yougov_methodology_pages() {
    let page = "\
The Economist
Fieldwork
YouGov
Interviewing Dates
May 9 - 11, 2026
Target population
U.S. Citizens, age 18 and over
Sampling method
Respondents were selected from YouGov's opt-in panel.
Weighting
The sample was weighted according to gender, age, race, education, and region.
Number of respondents
1549
Margin of error
+/- 3.5% adjusted for weighting
Survey mode
Web-based interviews
Questions not reported
2 questions not reported.
66
";

    assert!(parse_page(page).is_none());
}

#[test]
fn rejects_flattened_yougov_methodology_text() {
    let text = "The Economist Fieldwork YouGov Interviewing Dates May 9 - 11, 2026 Target population U.S. Citizens, age 18 and over Sampling method Respondents were selected from YouGov's opt-in panel to be representative of adult U.S. citizens. Sample composition 1,953 started the survey after screenouts. 181 were deleted due to breakoffs, 22 were removed for interview completion time under 5 minutes, 5 were removed for attention check failures, and 135 were removed for data quality control. The reporting is based on the remaining 1,610 respondents. Weighting The sample was weighted according to gender, age, race, education, U.S. region of residence, 2024 election turnout and presidential vote, 2020 election turnout and presidential vote, baseline party identification, and current voter registration status. Number of respondents 1549 1410 (Registered voters) Margin of error +/- 3.5% (adjusted for weighting) +/- 3.2% (Registered voters) Survey mode Web-based interviews Questions not reported 2 questions not reported. 66";

    assert!(is_non_question_text(text));
    assert!(parse_page(text).is_none());
}

#[test]
fn keeps_wrapped_question_text_that_looks_like_a_suffix() {
    let page = "\
The Economist/YouGov Poll
May 9 - 11, 2026 - 1549 U.S. Adult Citizens
If a state redraws their districts for the U.S. House of Representatives to favor one party, do you think it is fair or unfair for other states to redraw their districts to favor the
other party?
Sex Race Age
Total Male Female White Black
Fair 41% 42% 40% 39% 48%
Unfair 50% 49% 51% 52% 42%
";

    let question = parse_page(page).unwrap();

    assert!(question.text.ends_with("other party?"));
    assert!(!question.text.contains("Sex Race Age"));
}
