//! # Ipsos download parsing
//!
//! Parses Ipsos landing pages and article pages to extract download stubs.

mod article;
mod landing;

pub use article::parse_article_details;
pub use landing::parse_landing_stubs;
