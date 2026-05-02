//! # CLI argument definitions
//!
//! Parses command-line arguments for the enrichment tool.

use super::parse::{parse_scope, parse_u32, parse_usize, required_arg};
use super::usage::usage;
use crate::sources::Scope;
use crate::topics::enrichment::DynError;
use crate::topics::enrichment::index::index_path_from_env;
use std::io::{Error as IoError, ErrorKind};
use std::path::PathBuf;

/// Arguments for the `enrich-topics` CLI.
///
/// # Fields
/// - `scope`: The scope for data collection.
/// - `index_path`: Path to the question index file.
/// - `refresh`: Whether to refresh all classifications.
/// - `dry_run`: If true, don't write the index.
/// - `limit`: Optional limit on questions to classify.
#[derive(Debug)]
pub(super) struct EnrichmentArgs {
    pub(super) scope: Scope,
    pub(super) index_path: PathBuf,
    pub(super) refresh: bool,
    pub(super) dry_run: bool,
    pub(super) limit: Option<usize>,
}

    impl EnrichmentArgs {
        /// Parse CLI arguments into `EnrichmentArgs`.
        pub(super) fn parse(args: Vec<String>) -> Result<Self, DynError> {
        let mut scope = None;
        let mut count = None;
        let mut index_path = index_path_from_env();
        let mut refresh = false;
        let mut dry_run = false;
        let mut limit = None;
        let mut index = 0usize;

        while index < args.len() {
            let arg = &args[index];
            match arg.as_str() {
                "--help" | "-h" => return Err(Box::new(IoError::other(usage()))),
                "--refresh" => refresh = true,
                "--dry-run" => dry_run = true,
                "--scope" => {
                    index += 1;
                    scope = Some(required_arg(&args, index, "--scope")?);
                }
                "--count" | "--n" => {
                    index += 1;
                    count = Some(parse_u32(required_arg(&args, index, arg)?, arg)?);
                }
                "--index" => {
                    index += 1;
                    index_path = PathBuf::from(required_arg(&args, index, "--index")?);
                }
                "--limit" => {
                    index += 1;
                    limit = Some(parse_usize(required_arg(&args, index, "--limit")?)?);
                }
                value if value.starts_with("--scope=") => {
                    scope = Some(value.trim_start_matches("--scope=").to_string());
                }
                value if value.starts_with("--count=") => {
                    count = Some(parse_u32(value.trim_start_matches("--count="), "--count")?);
                }
                value if value.starts_with("--n=") => {
                    count = Some(parse_u32(value.trim_start_matches("--n="), "--n")?);
                }
                value if value.starts_with("--index=") => {
                    index_path = PathBuf::from(value.trim_start_matches("--index="));
                }
                value if value.starts_with("--limit=") => {
                    limit = Some(parse_usize(value.trim_start_matches("--limit="))?);
                }
                _ => return Err(unsupported_arg(arg)),
            }
            index += 1;
        }

        Ok(Self {
            scope: parse_scope(scope.as_deref(), count)?,
            index_path,
            refresh,
            dry_run,
            limit,
        })
    }
}

fn unsupported_arg(arg: &str) -> DynError {
    Box::new(IoError::new(
        ErrorKind::InvalidInput,
        format!("unsupported enrich-topics argument: {arg}\n{}", usage()),
    ))
}
