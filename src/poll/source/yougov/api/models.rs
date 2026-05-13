use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SurveyApiResponse {
    pub data: Vec<SurveyResponseItem>,
    pub hits: u64,
    pub totals: SurveyTotals,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "doc_type")]
pub enum SurveyResponseItem {
    #[serde(rename = "cms_document_editorial")]
    CmsDocumentEditorial(CmsDocumentEditorial),
    #[serde(rename = "result")]
    Result(SurveyResult),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CmsDocumentEditorial {
    pub cms_document_type: String,
    pub created_at: String,
    pub description: String,
    pub filetype: String,
    pub id: u64,
    pub language: NamedEntity,
    pub primary_category: PrimaryCategory,
    pub region: NamedEntity,
    pub site: NamedEntity,
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SurveyResult {
    pub primary_category: String,
    pub published_at: String,
    pub question_id: u64,
    pub summary: Vec<SurveySummary>,
    pub survey_id: String,
    pub survey_uuid: String,
    pub title: String,
    pub total: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SurveySummary {
    pub label: String,
    pub value: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NamedEntity {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PrimaryCategory {
    pub default_short_name: String,
    pub id: u64,
    pub name: String,
    pub search_description: String,
    pub seo_title: String,
    pub short_name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SurveyTotals {
    pub categories: BTreeMap<String, u64>,
    pub types: BTreeMap<String, u64>,
}
