# xaml-lang-formatter

`xaml-lang-formatter` is a formatter for WPF `ResourceDictionary` localization files. It is designed for language resources that use `x:Key` values such as `Common.Action.Open`, then groups and orders the entries deterministically.

The project provides both a Rust CLI and a browser UI powered by Rust WebAssembly, Nuxt 4, Nuxt UI 4, and Bun.

## Features

- Format `.xaml` language resource files.
- Support single files and recursive directory formatting in the CLI.
- Keep the `Meta` section first.
- Group resources by dotted `x:Key` prefixes.
- Configure the grouping threshold with `--group-threshold`; the default is `5`.
- Regenerate grouping comments consistently.
- Add a top formatter comment with a timestamp.
- Escape XML text and attribute values, including newline entities.
- Run the same formatter core in the CLI and in the browser through WebAssembly.
- Deploy the web app to GitHub Pages through GitHub Actions.

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

Output:

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

## CLI usage

```bash
cargo run -- ./Lang/en-US.xaml
cargo run -- ./Lang
cargo run -- ./Lang --check
cargo run -- ./Lang --dry-run
cargo run -- ./Lang --backup
cargo run -- ./Lang --group-threshold 3
```

`--partition-threshold` is also accepted as an alias of `--group-threshold`.

## Formatting rules

- Top-level prefixes always become sections.
- `Meta` is always sorted before other top-level sections.
- A nested prefix becomes its own section only when the number of resources under that prefix reaches the group threshold.
- The default group threshold is `5`.
- Existing formatter comments keep `--check` stable, but normal formatting writes a fresh timestamp.
- Existing grouping comments are ignored and regenerated.
- Duplicate `x:Key` values are rejected.

## Supported XAML shape

The first implementation intentionally targets plain localization resources:

```xml
<ResourceDictionary ...>
    <sys:String x:Key="Common.App">App</sys:String>
</ResourceDictionary>
```

Nested XAML objects, CDATA, and complex resource trees are not supported yet.

## Rust development

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Build the CLI:

```bash
cargo build --release
```

Build the WebAssembly package:

```bash
cd crates/wasm
wasm-pack build \
  --target web \
  --release \
  --out-dir ../../web/public/wasm
```

## Web UI development

The web app uses Bun as the JavaScript package manager and runtime.

```bash
cd web
bun install
bun run wasm:build
bun --bun run dev
```

Open the Nuxt dev server URL printed by the command.

## Web production build

```bash
cd web
bun run wasm:build
NUXT_APP_BASE_URL=/xaml-lang-formatter/ bun run build
```

For local preview after building:

```bash
bun run preview
```

## GitHub Pages deployment

The repository includes `.github/workflows/pages.yml` for GitHub Pages deployment.

Repository settings:

1. Open **Settings → Pages**.
2. Set **Source** to **GitHub Actions**.
3. Push to `master` or run the workflow manually.

The expected GitHub Pages URL is:

```text
https://pcl-community.github.io/xaml-lang-formatter/
```

If you use a custom domain, remove or adjust `NUXT_APP_BASE_URL` in the Pages workflow.

## Project layout

```text
xaml-lang-formatter/
├── src/              # Rust formatter core and CLI
├── crates/wasm/      # wasm-bindgen wrapper
├── web/              # Nuxt 4 + Nuxt UI 4 web app
└── .github/workflows # CI and GitHub Pages deployment
```

## License

This project is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.
