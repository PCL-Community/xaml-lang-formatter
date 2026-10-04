from __future__ import annotations

from xaml_lang_formatter_addon.core import (
    DEFAULT_GROUP_THRESHOLD,
    build_command,
    coerce_int,
    resolve_configured_binary,
)


def test_coerce_int_returns_value() -> None:
    assert coerce_int("4", 8) == 4
    assert coerce_int(4, 8) == 4


def test_coerce_int_falls_back() -> None:
    assert coerce_int(None, 8) == 8
    assert coerce_int("", 8) == 8
    assert coerce_int("nope", 8) == 8
    assert coerce_int(object(), DEFAULT_GROUP_THRESHOLD) == DEFAULT_GROUP_THRESHOLD


def test_build_command() -> None:
    assert build_command("/usr/bin/xlf", "/repo/Lang/fr.xaml", 4) == [
        "/usr/bin/xlf",
        "/repo/Lang/fr.xaml",
        "--group-threshold",
        "4",
    ]


def test_resolve_configured_binary_missing() -> None:
    assert resolve_configured_binary(None) is None
    assert resolve_configured_binary("") is None
    assert resolve_configured_binary("/does/not/exist/xlf") is None


def test_resolve_configured_binary_present(tmp_path) -> None:
    binary = tmp_path / "xlf"
    binary.write_text("#!/bin/sh\n", encoding="utf-8")

    resolved = resolve_configured_binary(binary)

    assert resolved == binary
