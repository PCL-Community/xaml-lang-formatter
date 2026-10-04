from __future__ import annotations

import os
import stat

from xaml_lang_formatter_addon import binaries


def test_platform_key_aliases() -> None:
    assert binaries.platform_key("Linux", "x86_64") == "linux-x86_64"
    assert binaries.platform_key("Linux", "amd64") == "linux-x86_64"
    assert binaries.platform_key("Linux", "aarch64") == "linux-aarch64"
    assert binaries.platform_key("Darwin", "arm64") == "darwin-arm64"
    assert binaries.platform_key("Darwin", "x86_64") == "darwin-x86_64"
    assert binaries.platform_key("Windows", "AMD64") == "windows-x86_64"


def test_platform_key_unknown() -> None:
    assert binaries.platform_key("Plan9", "x86_64") is None


def test_env_override(tmp_path, monkeypatch) -> None:
    binary = tmp_path / "xaml-lang-formatter"
    binary.write_text("#!/bin/sh\n", encoding="utf-8")

    monkeypatch.setenv(binaries.ENV_OVERRIDE, str(binary))
    binaries.resolve_binary.cache_clear()

    try:
        assert binaries.resolve_binary() == binary
    finally:
        binaries.resolve_binary.cache_clear()


def test_env_override_missing(monkeypatch) -> None:
    monkeypatch.setenv(binaries.ENV_OVERRIDE, "/does/not/exist/xlf")
    binaries.resolve_binary.cache_clear()

    try:
        assert binaries.resolve_binary() is None
    finally:
        binaries.resolve_binary.cache_clear()


def test_is_bundled_unknown_platform() -> None:
    assert binaries.is_bundled("plan9-x86_64") is False


def test_make_executable(tmp_path) -> None:
    binary = tmp_path / "xlf"
    binary.write_text("x", encoding="utf-8")
    binary.chmod(stat.S_IRUSR | stat.S_IWUSR)

    binaries._make_executable(binary)

    assert os.access(binary, os.X_OK)
