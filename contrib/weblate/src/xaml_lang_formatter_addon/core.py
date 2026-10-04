"""Framework-independent helpers for the xaml-lang-formatter add-on."""

from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from os import PathLike

DEFAULT_GROUP_THRESHOLD = 8
DEFAULT_TIMEOUT = 60


def coerce_int(value: object, default: int) -> int:
    """Return ``value`` as ``int``, falling back to ``default``."""
    if value is None or value == "":
        return default
    try:
        return int(value)  # type: ignore[arg-type]
    except (TypeError, ValueError):
        return default


def build_command(
    binary: str | PathLike[str],
    filename: str | PathLike[str],
    group_threshold: int,
) -> list[str]:
    """Build the xaml-lang-formatter argv for a single translation file."""
    return [
        str(binary),
        str(filename),
        "--group-threshold",
        str(group_threshold),
    ]


def resolve_configured_binary(configured: object) -> Path | None:
    """Return the configured binary path if it points at a file."""
    if not configured:
        return None
    binary = Path(str(configured)).expanduser()
    return binary if binary.is_file() else None
