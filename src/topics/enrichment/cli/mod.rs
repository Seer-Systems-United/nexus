mod args;
mod parse;
mod usage;

pub use parse::parse_scope;

use super::classifier::classifier_from_env;
use super::index::{classification_inputs, load_index_from_path, save_index_to_path};
use super::{DynError, QuestionEnrichment, QuestionIndex};
use crate::topics::service;
use args::EnrichmentArgs;
use std::collections::HashMap;

pub async fn run_cli(args: Vec<String>) -> Result<(), DynError> {
    if args
        .iter()
        .any(|arg| matches!(arg.as_str(), "--help" | "-h"))
    {
        println!("{}", usage::usage());
        return Ok(());
    }

    let args = EnrichmentArgs::parse(args)?;
    let classifier = classifier_from_env()?;
    let mapped = service::collect_unenriched_source_data(args.scope).await;

    for warning in &mapped.warnings {
        eprintln!("warning: {warning}");
    }

    let mut index = load_index_from_path(&args.index_path)?;
    let mut indexed_records = index
        .records
        .into_iter()
        .map(|record| (record.question_fingerprint.clone(), record))
        .collect::<HashMap<_, _>>();

    let mut inputs = classification_inputs(&mapped.observations, &indexed_records, args.refresh);
    if let Some(limit) = args.limit {
        inputs.truncate(limit);
    }

    if inputs.is_empty() {
        println!(
            "topic enrichment: no new candidate questions for {}",
            args.scope
        );
        return Ok(());
    }

    println!(
        "topic enrichment: classifying {} candidate questions with {}",
        inputs.len(),
        classifier.model_name()
    );

    let mut classified = 0usize;
    for input in inputs {
        let output = classifier.classify(&input).await?;
        indexed_records.insert(
            input.question_fingerprint.clone(),
            QuestionEnrichment::from_classification(input, output, classifier.model_name()),
        );
        classified += 1;
    }

    index = QuestionIndex {
        version: super::INDEX_VERSION,
        records: indexed_records.into_values().collect(),
    };
    index.records.sort_by(|left, right| {
        left.question_fingerprint
            .cmp(&right.question_fingerprint)
            .then_with(|| left.source.cmp(&right.source))
    });

    if args.dry_run {
        println!("topic enrichment: dry run complete, index was not written");
    } else {
        save_index_to_path(&args.index_path, &index)?;
        println!(
            "topic enrichment: wrote {} records to {}",
            index.records.len(),
            args.index_path.display()
        );
    }

    println!("topic enrichment: classified {classified} candidate questions");
    Ok(())
}
