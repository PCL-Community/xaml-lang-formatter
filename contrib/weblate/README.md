# xaml-lang-formatter Weblate add-on

A [Weblate](https://weblate.org/) add-on that runs
[`xaml-lang-formatter`](../..) on WPF `ResourceDictionary` (`.xaml`)
localization files **right before Weblate commits them**.

Because Weblate writes the current translation to disk and then fires the
`vcs_pre_commit` signal before creating the git commit, rewriting the file in this hook lands the
formatted content in the **same commit**. No extra commit is created.

## How it works

1. Weblate persists the edited translation to disk.
2. The `Repository pre-commit` event runs this add-on.
3. The add-on invokes the bundled `xaml-lang-formatter` binary on the translation file, rewriting it
   in place.
4. Weblate commits `translation.filenames` (which contains the formatted file)
   in a single commit and records the new file hash.

The add-on is compatible only with components using the
`ResourceDictionary file` format (Weblate format id `resourcedictionary`) and subscribes only to
`EVENT_PRE_COMMIT`.

## Requirements

- Weblate 5.x / 2026.x (uses the current `BaseAddon` API).
- A platform for which a binary is bundled (see below) or a custom binary path.

Bundled platform binaries are built from the sources in this repository. The wheel ships them under
`xaml_lang_formatter_addon/bin/<platform-key>/`; the correct one is selected at runtime from
`platform.system()` /
`platform.machine()`.

| Platform key     | Typical Weblate deployment |
|------------------|----------------------------|
| `linux-x86_64`   | Docker / most servers      |
| `linux-aarch64`  | ARM servers                |
| `windows-x86_64` | Windows (development)      |

On any other platform, set the `binary` option to an absolute path, or set the
`XAML_LANG_FORMATTER_BINARY` environment variable.

## Installation

### 1. Build the wheel

From `contrib/weblate`:

```bash
# Bundle language binary for the current host (optional if already present).
./scripts/build-binaries.sh

# Build the wheel (include every platform you want inside the wheel).
python -m build --wheel
```

Built wheels are written to `dist/`.

To bundle additional platforms, cross-compile them explicitly before building the wheel:

```bash
./scripts/build-binaries.sh \
  x86_64-unknown-linux-musl:linux-x86_64 \
  aarch64-unknown-linux-musl:linux-aarch64
```

> Linux musl targets produce static binaries with no runtime glibc
> dependency. macOS and Windows binaries must be built on their own runners
> (see `.github/workflows/weblate-addon.yml`).

Binaries under `bin/` are intentionally gitignored; the wheel build includes them via
`tool.hatch.build.artifacts`.

### 2. Install into Weblate

```bash
uv pip install xaml_lang_formatter_weblate-0.1.0-py3-none-any.whl
# or, for development:
uv pip install -e .
```

### 3. Register the add-on

Add the class to `WEBLATE_ADDONS` in your Weblate settings override (`local_settings.py`, or
`/app/data/settings-override.py` in Docker):

```python
WEBLATE_ADDONS += ("xaml_lang_formatter_addon.addons.XamlLangFormatterAddon",)
```

Restart Weblate. The add-on then appears as **Format ResourceDictionary (xaml-lang-formatter)** on
compatible components (Operations → Add-ons).

### Docker

The add-on ships a settings module, so a custom image only needs to install the
wheel and point `DJANGO_SETTINGS_MODULE` at it. The binary is inside the wheel,
so no Rust toolchain is needed in the runtime image.

**Option 1 — custom image (recommended)**

Create a `Dockerfile` next to the downloaded wheel:

```dockerfile
FROM weblate/weblate:latest

USER root

COPY xaml_lang_formatter_weblate-0.1.0-py3-none-any.whl /usr/src/
RUN source /app/venv/bin/activate \
    && uv pip install --no-cache-dir /usr/src/xaml_lang_formatter_weblate-0.1.0-py3-none-any.whl

ENV DJANGO_SETTINGS_MODULE=xaml_lang_formatter_addon.settings

USER 1000
```

Then copy the official `docker-compose.yml` and replace `image: weblate/weblate`
with `build: .` on the `weblate` service, or build and tag the image yourself
and reference that tag:

```bash
docker build -t my-weblate .
```

The bundled binary matching the container architecture
(`linux-x86_64` / `linux-aarch64`) is selected automatically.

**Option 2 — no rebuild, via the data volume**

Weblate adds `/app/data/python` to the import path, so you can mount the package
and register it through the settings override:

```bash
unzip xaml_lang_formatter_weblate-0.1.0-py3-none-any.whl -d /tmp/xlf
```

```yaml
# docker-compose.override.yml
services:
  weblate:
    volumes:
      - ./xlf/xaml_lang_formatter_addon:/app/data/python/xaml_lang_formatter_addon:ro
      - ./settings-override.py:/app/data/settings-override.py:ro
```

```python
# settings-override.py
WEBLATE_ADDONS += ("xaml_lang_formatter_addon.addons.XamlLangFormatterAddon",)
```

Recreate the container afterwards so the mount and settings are applied:

```bash
docker compose up -d
```

Use Option 1 when possible: the wheel installs the package properly and keeps
the settings module versioned with it.


## Configuration

| Option            | Description                                                                         | Default   |
|-------------------|-------------------------------------------------------------------------------------|-----------|
| `binary`          | Absolute path to a custom `xaml-lang-formatter` binary. Empty uses the bundled one. | *(empty)* |
| `group_threshold` | Minimum resources sharing a prefix before a nested section is created.              | `8`       |
| `timeout`         | Maximum seconds allowed to format one file.                                         | `60`      |

## Development

```bash
cd contrib/weblate
python -m venv .venv && . .venv/bin/activate
pip install -e '.[dev]'
pytest
ruff check . && ruff format --check .
```

`tests/conftest.py` installs lightweight stubs for the `django` and `weblate`
modules so the add-on logic can be tested without a full Weblate installation.
`tests/test_integration.py` runs the real bundled binary end to end.

## Troubleshooting

| Symptom                                       | Cause / fix                                                                                                                                  |
|-----------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
| Add-on activity shows `required-file-missing` | No binary for this platform. Build one (`scripts/build-binaries.sh`) or set `binary` / `XAML_LANG_FORMATTER_BINARY`.                         |
| `Permission denied` running the binary        | The wheel did not preserve the execute bit. The add-on chmods the file and falls back to a temp copy; ensure the temp dir is writable.       |
| Formatting fails with a parse error           | The file has content the formatter does not support (nested objects, CDATA). The original commit still proceeds.                             |
| File is reformatted on every change           | Expected: Weblate rewrites files in its own layout, and the add-on reformats before each commit. Already-formatted files are left untouched. |
