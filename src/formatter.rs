use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::api::{FormatOptions, format_xaml, stable_format_xaml};
use crate::cli::Cli;
use crate::grouping::DEFAULT_GROUP_THRESHOLD;

pub fn run(cli: Cli) -> Result<()> {
    if cli.group_threshold == 0 {
        bail!("--group-threshold must be greater than 0");
    }

    let timestamp = cli
        .timestamp
        .clone()
        .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string());

    let paths = if cli.paths.is_empty() {
        vec![std::env::current_dir().context("failed to resolve current directory")?]
    } else {
        cli.paths.clone()
    };

    let files = collect_files(&paths, !cli.no_recursive)?;
    let mut changed = 0usize;
    let mut failed = 0usize;

    for file in files {
        match process_file(&file, &cli, &timestamp) {
            Ok(true) => changed += 1,
            Ok(false) => {}
            Err(err) => {
                failed += 1;
                eprintln!("error: {}: {err:#}", file.display());
            }
        }
    }

    if cli.check && changed > 0 {
        bail!("{changed} file(s) are not formatted");
    }

    if failed > 0 {
        bail!("{failed} file(s) failed");
    }

    Ok(())
}

pub fn format_content(input: &str, timestamp: &str) -> Result<String> {
    format_content_with_group_threshold(input, timestamp, DEFAULT_GROUP_THRESHOLD)
}

pub fn format_content_with_group_threshold(
    input: &str,
    timestamp: &str,
    group_threshold: usize,
) -> Result<String> {
    format_xaml(
        input,
        &FormatOptions {
            group_threshold,
            timestamp: timestamp.to_string(),
        },
    )
}

pub fn stable_format_content(input: &str, timestamp: &str) -> Result<String> {
    stable_format_content_with_group_threshold(input, timestamp, DEFAULT_GROUP_THRESHOLD)
}

pub fn stable_format_content_with_group_threshold(
    input: &str,
    timestamp: &str,
    group_threshold: usize,
) -> Result<String> {
    stable_format_xaml(
        input,
        &FormatOptions {
            group_threshold,
            timestamp: timestamp.to_string(),
        },
    )
}

pub fn process_file(path: &Path, cli: &Cli, timestamp: &str) -> Result<bool> {
    let input = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    let stable_output =
        stable_format_content_with_group_threshold(&input, timestamp, cli.group_threshold)
            .with_context(|| format!("failed to format {}", path.display()))?;

    if normalize_newline(&input) == normalize_newline(&stable_output) {
        return Ok(false);
    }

    if cli.check {
        println!("not formatted: {}", path.display());
        return Ok(true);
    }

    if cli.dry_run {
        println!("would format: {}", path.display());
        return Ok(true);
    }

    let output = format_content_with_group_threshold(&input, timestamp, cli.group_threshold)?;

    if cli.backup {
        let backup_path = path.with_extension("xaml.bak");
        std::fs::write(&backup_path, &input)
            .with_context(|| format!("failed to write backup {}", backup_path.display()))?;
    }

    std::fs::write(path, output).with_context(|| format!("failed to write {}", path.display()))?;
    println!("formatted: {}", path.display());

    Ok(true)
}

pub fn collect_files(paths: &[PathBuf], recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for path in paths {
        if path.is_file() {
            if is_xaml(path) {
                files.push(path.clone());
            }
        } else if path.is_dir() {
            if recursive {
                for entry in walkdir::WalkDir::new(path) {
                    let entry = entry?;
                    let p = entry.path();

                    if p.is_file() && is_xaml(p) {
                        files.push(p.to_path_buf());
                    }
                }
            } else {
                for entry in std::fs::read_dir(path)? {
                    let entry = entry?;
                    let p = entry.path();

                    if p.is_file() && is_xaml(&p) {
                        files.push(p);
                    }
                }
            }
        } else {
            bail!("path does not exist: {}", path.display());
        }
    }

    files.sort();
    files.dedup();

    Ok(files)
}

pub fn is_xaml(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s.eq_ignore_ascii_case("xaml"))
        .unwrap_or(false)
}

pub fn normalize_newline(s: &str) -> String {
    s.replace("\r\n", "\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn formats_sample_input_with_default_threshold_eight() {
        let input = r#"<ResourceDictionary
    xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
    xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
    xmlns:sys="clr-namespace:System;assembly=mscorlib"
    xml:space="preserve">

    <sys:String x:Key="Common.Action.Test.Test2">2</sys:String>
    <sys:String x:Key="Common.App">App</sys:String>
    <sys:String x:Key="Meta.Name">English (US)</sys:String>
    <sys:String x:Key="Common.Action.Close">Close</sys:String>
    <sys:String x:Key="Meta.Code">en-US</sys:String>
    <sys:String x:Key="Common.Action.Open">Open</sys:String>
    <sys:String x:Key="Common.Action.Test.Test1">1</sys:String>

</ResourceDictionary>
"#;

        let expected = r#"<!-- Formatted by xaml-lang-formatter at 2026-06-28T15:21:30. -->
<ResourceDictionary
    xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
    xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
    xmlns:sys="clr-namespace:System;assembly=mscorlib"
    xml:space="preserve">

    <!-- Meta -->

    <sys:String x:Key="Meta.Code">en-US</sys:String>
    <sys:String x:Key="Meta.Name">English (US)</sys:String>

    <!-- Common -->

    <sys:String x:Key="Common.Action.Close">Close</sys:String>
    <sys:String x:Key="Common.Action.Open">Open</sys:String>
    <sys:String x:Key="Common.Action.Test.Test1">1</sys:String>
    <sys:String x:Key="Common.Action.Test.Test2">2</sys:String>
    <sys:String x:Key="Common.App">App</sys:String>

</ResourceDictionary>
"#;

        let output = format_content(input, "2026-06-28T15:21:30").unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn formats_sample_input_with_threshold_two() {
        let input = r#"<ResourceDictionary
    xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
    xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
    xmlns:sys="clr-namespace:System;assembly=mscorlib"
    xml:space="preserve">

    <sys:String x:Key="Common.Action.Test.Test2">2</sys:String>
    <sys:String x:Key="Common.App">App</sys:String>
    <sys:String x:Key="Meta.Name">English (US)</sys:String>
    <sys:String x:Key="Common.Action.Close">Close</sys:String>
    <sys:String x:Key="Meta.Code">en-US</sys:String>
    <sys:String x:Key="Common.Action.Open">Open</sys:String>
    <sys:String x:Key="Common.Action.Test.Test1">1</sys:String>

</ResourceDictionary>
"#;

        let expected = r#"<!-- Formatted by xaml-lang-formatter at 2026-06-28T15:21:30. -->
<ResourceDictionary
    xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
    xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
    xmlns:sys="clr-namespace:System;assembly=mscorlib"
    xml:space="preserve">

    <!-- Meta -->

    <sys:String x:Key="Meta.Code">en-US</sys:String>
    <sys:String x:Key="Meta.Name">English (US)</sys:String>

    <!-- Common -->

    <sys:String x:Key="Common.App">App</sys:String>

    <!-- Common.Action -->

    <sys:String x:Key="Common.Action.Close">Close</sys:String>
    <sys:String x:Key="Common.Action.Open">Open</sys:String>

    <!-- Common.Action.Test -->

    <sys:String x:Key="Common.Action.Test.Test1">1</sys:String>
    <sys:String x:Key="Common.Action.Test.Test2">2</sys:String>

</ResourceDictionary>
"#;

        let output = format_content_with_group_threshold(input, "2026-06-28T15:21:30", 2).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn already_formatted_content_is_stable_when_timestamp_exists() {
        let input = r#"<!-- Formatted by xaml-lang-formatter at 2026-01-01T00:00:00. -->
<ResourceDictionary>

    <!-- Common -->

    <sys:String x:Key="Common.App">App</sys:String>

</ResourceDictionary>
"#;

        let output = stable_format_content(input, "2026-06-28T15:21:30").unwrap();

        assert_eq!(output, input);
    }
}
