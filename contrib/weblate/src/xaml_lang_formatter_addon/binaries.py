"""Locate the ``xaml-lang-formatter`` binary bundled with this package.

Prebuilt release binaries are shipped inside ``bin/<platform-key>/``. The
platform key is derived from :func:`platform.system` and
:func:`platform.machine`, for example ``linux-x86_64`` or ``darwin-arm64``.

Wheels are zip archives and do not reliably preserve Unix execute
permissions, so the selected file is made executable at runtime. If the
package directory is not writable, the binary is copied into a temporary
cache directory first.
"""

from __future__ import annotations

import os
import platform
import shutil
import stat
import tempfile
from functools import cache
from pathlib import Path

BIN_DIR = Path(__file__).resolve().parent / "bin"
BINARY_STEM = "xaml-lang-formatter"

#: Environment variable that overrides the bundled binary lookup.
ENV_OVERRIDE = "XAML_LANG_FORMATTER_BINARY"

_SYSTEM_ALIASES = {
    "darwin": "darwin",
    "linux": "linux",
    "windows": "windows",
}

_MACHINE_ALIASES = {
    "x86_64": "x86_64",
    "amd64": "x86_64",
    "aarch64": "aarch64",
    "arm64": "arm64",
}


def platform_key(system: str | None = None, machine: str | None = None) -> str | None:
    """Return the normalized ``<system>-<machine>`` key for a platform.

    ``None`` is returned when the platform is not recognized.
    """
    system = (system or platform.system()).lower()
    machine = (machine or platform.machine()).lower()

    normalized_system = _SYSTEM_ALIASES.get(system)
    normalized_machine = _MACHINE_ALIASES.get(machine, machine)

    if normalized_system is None or not normalized_machine:
        return None

    return f"{normalized_system}-{normalized_machine}"


def _binary_name(key: str) -> str:
    suffix = ".exe" if key.startswith("windows") else ""
    return f"{BINARY_STEM}{suffix}"


def _make_executable(path: Path) -> Path:
    mode = path.stat().st_mode
    path.chmod(mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    return path


def _cache_binary(candidate: Path, key: str) -> Path:
    cache_dir = Path(tempfile.gettempdir()) / BINARY_STEM / key
    cache_dir.mkdir(parents=True, exist_ok=True)
    target = cache_dir / candidate.name

    if not target.is_file() or target.stat().st_size != candidate.stat().st_size:
        shutil.copy2(candidate, target)

    return _make_executable(target)


@cache
def resolve_binary() -> Path | None:
    """Return a usable path to the formatter binary, or ``None``.

    Resolution order:

    1. The :data:`ENV_OVERRIDE` environment variable, if it points to a file.
    2. The bundled binary for the current platform.
    """
    override = os.environ.get(ENV_OVERRIDE)
    if override:
        override_path = Path(override).expanduser()
        return override_path if override_path.is_file() else None

    key = platform_key()
    if key is None:
        return None

    candidate = BIN_DIR / key / _binary_name(key)
    if not candidate.is_file():
        return None

    try:
        return _make_executable(candidate)
    except OSError:
        return _cache_binary(candidate, key)


def bundled_platforms() -> list[str]:
    """Return the platform keys for which a binary is bundled."""
    if not BIN_DIR.is_dir():
        return []
    return sorted(entry.name for entry in BIN_DIR.iterdir() if entry.is_dir())


def is_bundled(key: str | None = None) -> bool:
    """Return whether a bundled binary exists for ``key`` (or this platform)."""
    key = key or platform_key()
    if key is None:
        return False
    return (BIN_DIR / key / _binary_name(key)).is_file()
