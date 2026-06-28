use std::path::PathBuf;

use clap::Parser;

use crate::grouping::DEFAULT_GROUP_THRESHOLD;

#[derive(Debug, Parser)]
#[command(name = "xaml-lang-formatter")]
#[command(version)]
#[command(about = "Format WPF XAML ResourceDictionary language files")]
pub struct Cli {
    /// Files or directories to format
    #[arg(required = true)]
    pub paths: Vec<PathBuf>,

    /// Check whether files are already formatted
    #[arg(long)]
    pub check: bool,

    /// Print changed file paths but do not write
    #[arg(long)]
    pub dry_run: bool,

    /// Create .bak before overwriting
    #[arg(long)]
    pub backup: bool,

    /// Minimum number of resources sharing the same prefix before creating a nested group
    #[arg(long, visible_alias = "partition-threshold", default_value_t = DEFAULT_GROUP_THRESHOLD)]
    pub group_threshold: usize,

    /// Do not recursively process directories
    #[arg(long)]
    pub no_recursive: bool,

    /// Fixed timestamp for tests, format: yyyy-MM-ddTHH:mm:ss
    #[arg(long, hide = true)]
    pub timestamp: Option<String>,
}
