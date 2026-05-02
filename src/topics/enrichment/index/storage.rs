use crate::topics::enrichment::{DEFAULT_INDEX_PATH, DynError, QuestionIndex};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub(super) fn load_index() -> Result<QuestionIndex, DynError> {
    load_index_from_path(&index_path_from_env())
}

pub(in crate::topics::enrichment) fn load_index_from_path(
    path: &Path,
) -> Result<QuestionIndex, DynError> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(QuestionIndex::default()),
        Err(error) => return Err(Box::new(error)),
    };

    Ok(serde_json::from_str::<QuestionIndex>(&content)?)
}

pub(in crate::topics::enrichment) fn save_index_to_path(
    path: &Path,
    index: &QuestionIndex,
) -> Result<(), DynError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(index)?;
    fs::write(path, format!("{content}\n"))?;
    Ok(())
}

pub(in crate::topics::enrichment) fn index_path_from_env() -> PathBuf {
    std::env::var("NEXUS_TOPIC_INDEX_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_INDEX_PATH))
}
