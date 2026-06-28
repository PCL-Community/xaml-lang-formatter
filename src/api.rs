use anyhow::{bail, Result};

use crate::grouping::DEFAULT_GROUP_THRESHOLD;
use crate::parser::parse_document;
use crate::writer::write_document_with_group_threshold;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatOptions {
    pub group_threshold: usize,
    pub timestamp: String,
}

impl FormatOptions {
    pub fn new(timestamp: impl Into<String>) -> Self {
        Self {
            group_threshold: DEFAULT_GROUP_THRESHOLD,
            timestamp: timestamp.into(),
        }
    }
}

pub fn format_xaml(input: &str, options: &FormatOptions) -> Result<String> {
    validate_options(options)?;

    let doc = parse_document(input)?;
    write_document_with_group_threshold(&doc, &options.timestamp, options.group_threshold)
}

pub fn stable_format_xaml(input: &str, options: &FormatOptions) -> Result<String> {
    validate_options(options)?;

    let doc = parse_document(input)?;
    let timestamp = doc
        .existing_formatter_timestamp
        .as_deref()
        .unwrap_or(&options.timestamp);

    write_document_with_group_threshold(&doc, timestamp, options.group_threshold)
}

fn validate_options(options: &FormatOptions) -> Result<()> {
    if options.group_threshold == 0 {
        bail!("group threshold must be greater than 0");
    }

    if options.timestamp.trim().is_empty() {
        bail!("timestamp must not be empty");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_group_threshold() {
        let options = FormatOptions {
            group_threshold: 0,
            timestamp: "2026-06-28T15:21:30".to_string(),
        };

        let err = format_xaml("", &options).unwrap_err().to_string();

        assert!(err.contains("group threshold must be greater than 0"));
    }

    #[test]
    fn rejects_empty_timestamp() {
        let options = FormatOptions {
            group_threshold: 5,
            timestamp: "".to_string(),
        };

        let err = format_xaml("", &options).unwrap_err().to_string();

        assert!(err.contains("timestamp must not be empty"));
    }
}
