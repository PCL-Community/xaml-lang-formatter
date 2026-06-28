# xaml-lang-formatter

[![CI](https://github.com/PCL-Community/xaml-lang-formatter/actions/workflows/ci.yml/badge.svg)](https://github.com/PCL-Community/xaml-lang-formatter/actions/workflows/ci.yml)
[![GitHub Pages](https://github.com/PCL-Community/xaml-lang-formatter/actions/workflows/pages.yml/badge.svg)](https://github.com/PCL-Community/xaml-lang-formatter/actions/workflows/pages.yml)

`xaml-lang-formatter` is a formatter for WPF `ResourceDictionary` localization files. It understands language-resource
keys such as `Meta.Name`, `Common.Action.Open`, and `Application.Window.Title`, then rewrites the dictionary into
deterministic, review-friendly sections.

The repository currently contains:

- a Rust CLI binary, `xaml-lang-formatter`;
- a reusable Rust library API used by both CLI and WASM;
- a `wasm-bindgen` wrapper crate;
- a Nuxt 4 + Nuxt UI 4 browser app using Bun and CodeMirror 6;
- GitHub Actions workflows for CI and GitHub Pages deployment.

The current package version is **0.1.1**.

## Features

- Formats `.xaml` WPF localization dictionaries.
- Supports individual files and recursive directory formatting.
- Defaults to the current working directory when no path is provided.
- Groups resources by dotted `x:Key` prefixes.
- Keeps the `Meta` top-level section first.
- Uses a configurable nested-group threshold. The default is `8`.
- Regenerates grouping comments consistently.
- Adds a top formatter comment with a timestamp.
- Rejects duplicate `x:Key` values.
- Escapes XML text and attributes, including newline entities.
- Provides the same formatter behavior in CLI and browser via Rust WebAssembly.
- Ships a modern web UI with a single CodeMirror editor and in-place formatting.

## Formatting example

Input:

```xml

<ResourceDictionary
        xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
        xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
        xmlns:sys="clr-namespace:System;assembly=mscorlib"
        xml:space="preserve">

    <sys:String x:Key="Common.Action.Open">Open</sys:String>
    <sys:String x:Key="Meta.Code">en-US</sys:String>
    <sys:String x:Key="Common.App">App</sys:String>

</ResourceDictionary>
```

Output with the default threshold `8`:

```xml
<!-- Formatted by xaml-lang-formatter at 2026-06-28T15:21:30. -->
<ResourceDictionary
        xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
        xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
        xmlns:sys="clr-namespace:System;assembly=mscorlib"
        xml:space="preserve">

    <!-- Meta -->

    <sys:String x:Key="Meta.Code">en-US</sys:String>

    <!-- Common -->

    <sys:String x:Key="Common.Action.Open">Open</sys:String>
    <sys:String x:Key="Common.App">App</sys:String>

</ResourceDictionary>
```

A nested section such as `Common.Action` is only created when at least `8` resources share that prefix, unless you
override the threshold.

## CLI usage

Run from source:

```bash
cargo run -- ./Lang/en-US.xaml
cargo run -- ./Lang
cargo run -- ./Lang --check
cargo run -- ./Lang --dry-run
cargo run -- ./Lang --backup
cargo run -- ./Lang --group-threshold 8
```

When no path is passed, the formatter uses the current working directory:

```bash
cargo run
```

After installing or building the binary:

```bash
xaml-lang-formatter
xaml-lang-formatter ./Lang
xaml-lang-formatter ./Lang/en-US.xaml --check
```

### CLI options

| Option                      | Description                                                                                  |
|-----------------------------|----------------------------------------------------------------------------------------------|
| `[paths...]`                | Files or directories to format. Defaults to the current directory when omitted.              |
| `--check`                   | Check whether files are already formatted. Does not write changes.                           |
| `--dry-run`                 | Print files that would change. Does not write changes.                                       |
| `--backup`                  | Write a `.xaml.bak` backup before overwriting a file.                                        |
| `--group-threshold <N>`     | Minimum number of resources sharing a prefix before creating a nested section. Default: `8`. |
| `--partition-threshold <N>` | Alias for `--group-threshold`.                                                               |
| `--no-recursive`            | Only format direct `.xaml` files in a directory.                                             |
| `--help`                    | Show the full CLI help.                                                                      |

## Formatting rules

- Only the root `ResourceDictionary` is formatted.
- Existing grouping comments are ignored and regenerated.
- A formatter comment is written at the top of the file.
- Top-level key prefixes always become sections.
- `Meta` is always sorted before other top-level sections.
- Other top-level sections are sorted alphabetically.
- A nested prefix becomes a section only when its resource count reaches the group threshold.
- Items that do not reach the nested threshold stay under the closest parent section.
- Items inside the same section are sorted by full `x:Key`.
- Duplicate `x:Key` values are rejected.
- `--check` remains stable for already formatted files by reusing an existing formatter timestamp.

## Supported XAML shape

This formatter intentionally targets plain localization resources:

```xml
<ResourceDictionary ...>
    <sys:String x:Key="Common.App">App</sys:String>
    <sys:String x:Key="Common.Action.Open">Open</sys:String>
</ResourceDictionary>
```

Supported:

- `ResourceDictionary` as the root element;
- direct child resources with an `x:Key` or `*:Key` attribute;
- plain text content;
- self-closing keyed child elements;
- XML declarations and root attributes;
- existing formatter comments.

Currently unsupported:

- nested XAML objects inside resource values;
- CDATA sections;
- arbitrary non-keyed child resources;
- preserving original hand-written grouping comments.

## Rust library API

The root crate exposes a library API in `src/api.rs` so the CLI and WASM wrapper share one formatter implementation.

```rust
use xaml_lang_formatter::api::{format_xaml, FormatOptions};

let output = format_xaml(
    input,
    &FormatOptions {
        group_threshold: 8,
        timestamp: "2026-06-28T15:21:30".to_string(),
    },
)?;
```

The CLI feature is enabled by default. For non-CLI consumers such as WASM, use the crate with default features disabled.

## Web UI

The web app lives in `web/` and uses:

- Nuxt 4;
- Nuxt UI 4;
- Bun;
- CodeMirror 6 for XML editing;
- a local WASM build generated from `crates/wasm`.

The UI formats the current CodeMirror editor content in place. Files are read locally through the browser and are not
uploaded.

### Web development

Install dependencies and build the WASM package:

```bash
cd web
bun install
bun run wasm:build
bun --bun run dev
```

Open the Nuxt dev server URL printed by the command.

### Web typecheck and production build

```bash
cd web
bun run typecheck
bun run wasm:build
NUXT_APP_BASE_URL=/xaml-lang-formatter/ bun run build
bun run preview
```

### WASM build details

The web build command runs:

```bash
cd crates/wasm
wasm-pack build --target web --release --out-dir ../../web/public/wasm
```

`wasm-opt` is disabled in `crates/wasm/Cargo.toml` to avoid requiring Binaryen downloads during local builds and CI.

## GitHub Pages deployment

The repository includes `.github/workflows/pages.yml`.

To enable deployment:

1. Open the repository on GitHub.
2. Go to **Settings → Pages**.
3. Set **Source** to **GitHub Actions**.
4. Push to `master` or run the workflow manually.

The default Pages URL is expected to be:

```text
https://pcl-community.github.io/xaml-lang-formatter/
```

For a custom domain, adjust or remove `NUXT_APP_BASE_URL` in the Pages workflow.

## Development checks

Run the Rust checks:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Run the WASM check/build:

```bash
cargo check --manifest-path crates/wasm/Cargo.toml --target wasm32-unknown-unknown
cd web
bun run wasm:build
```

Run the web checks:

```bash
cd web
bun install
bun run typecheck
bun run build
```

## Project layout

```text
xaml-lang-formatter/
├── src/
│   ├── api.rs          # Shared formatter API
│   ├── cli.rs          # clap CLI definition
│   ├── formatter.rs    # File and directory processing
│   ├── grouping.rs     # x:Key prefix tree and grouping rules
│   ├── lib.rs          # Library exports
│   ├── main.rs         # CLI entry point
│   ├── model.rs        # Document model
│   ├── parser.rs       # ResourceDictionary parser
│   └── writer.rs       # Deterministic XML writer
├── crates/
│   └── wasm/           # wasm-bindgen wrapper crate
├── web/
│   ├── app/            # Nuxt 4 app source
│   ├── public/wasm/    # Generated WASM output
│   ├── bun.lock
│   ├── nuxt.config.ts
│   └── package.json
└── .github/workflows/
    ├── ci.yml          # Rust, WASM, and web checks
    └── pages.yml       # GitHub Pages deployment
```

## Troubleshooting

### `wasm-pack` tries to download Binaryen

The WASM crate disables `wasm-opt` in release builds:

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

If you still see Binaryen download errors, make sure you are running `bun run wasm:build` from the current project
files.

### GitHub Pages loads a blank page or cannot find WASM

Use the repository base URL when building for Pages:

```bash
cd web
NUXT_APP_BASE_URL=/xaml-lang-formatter/ bun run build
```

The runtime imports WASM from `public/wasm`, so the base URL must match the deployment path.

### `--check` reports files as changed after a previous format

`--check` reuses the existing formatter timestamp when it detects one. If a file was edited manually or the formatter
comment was removed, run the formatter once before using `--check` in CI.

## License

This project is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.
