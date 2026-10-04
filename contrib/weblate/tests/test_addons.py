from __future__ import annotations

import subprocess
from pathlib import Path
from types import SimpleNamespace

import pytest

from xaml_lang_formatter_addon import addons


class Storage:
    def __init__(self, configuration: dict | None = None) -> None:
        self.configuration = configuration if configuration is not None else {}


class FakeTranslation:
    def __init__(self, filename: str | None, full_path: str = "/repo") -> None:
        self._filename = filename
        self.component = SimpleNamespace(full_path=full_path)
        self.store_hash_calls = 0

    def get_filename(self) -> str | None:
        return self._filename

    def store_hash(self) -> None:
        self.store_hash_calls += 1


@pytest.fixture
def binary(monkeypatch, tmp_path) -> Path:
    path = tmp_path / "xaml-lang-formatter"
    path.write_text("#!/bin/sh\n", encoding="utf-8")
    monkeypatch.setattr(addons, "resolve_binary", lambda: path)
    return path


def test_success_uses_bundled_binary(binary, monkeypatch) -> None:
    captured: dict = {}

    def fake_run(cmd, **kwargs):
        captured["cmd"] = cmd
        captured["kwargs"] = kwargs
        return subprocess.CompletedProcess(cmd, 0, "", "")

    monkeypatch.setattr(addons.subprocess, "run", fake_run)

    addon = addons.XamlLangFormatterAddon(Storage())
    translation = FakeTranslation("/repo/Lang/fr.xaml")

    assert addon.pre_commit(translation, "author", True) is None
    assert translation.store_hash_calls == 1
    assert captured["cmd"] == [
        str(binary),
        "/repo/Lang/fr.xaml",
        "--group-threshold",
        "8",
    ]
    assert captured["kwargs"]["cwd"] == "/repo"


def test_group_threshold_configuration(binary, monkeypatch) -> None:
    captured: dict = {}

    def fake_run(cmd, **kwargs):
        captured["cmd"] = cmd
        return subprocess.CompletedProcess(cmd, 0, "", "")

    monkeypatch.setattr(addons.subprocess, "run", fake_run)

    addon = addons.XamlLangFormatterAddon(Storage({"group_threshold": 4}))
    translation = FakeTranslation("/repo/Lang/fr.xaml")

    addon.pre_commit(translation, "author", True)

    assert captured["cmd"][-1] == "4"


def test_store_hash_skipped(monkeypatch, binary) -> None:
    monkeypatch.setattr(
        addons.subprocess,
        "run",
        lambda cmd, **kwargs: subprocess.CompletedProcess(cmd, 0, "", ""),
    )

    addon = addons.XamlLangFormatterAddon(Storage())
    translation = FakeTranslation("/repo/Lang/fr.xaml")

    assert addon.pre_commit(translation, "author", False) is None
    assert translation.store_hash_calls == 0


def test_missing_binary_returns_error(monkeypatch) -> None:
    monkeypatch.setattr(addons, "resolve_binary", lambda: None)

    addon = addons.XamlLangFormatterAddon(Storage())
    translation = FakeTranslation("/repo/Lang/fr.xaml")

    outcome = addon.pre_commit(translation, "author", True)

    assert outcome is not None
    assert outcome.status == "error"
    assert outcome.reason == "required-file-missing"
    assert translation.store_hash_calls == 0


def test_no_filename_is_skipped(binary) -> None:
    addon = addons.XamlLangFormatterAddon(Storage())
    translation = FakeTranslation(None)

    outcome = addon.pre_commit(translation, "author", True)

    assert outcome is not None
    assert outcome.status == "skipped"


def test_nonzero_exit_returns_error(binary, monkeypatch) -> None:
    monkeypatch.setattr(
        addons.subprocess,
        "run",
        lambda cmd, **kwargs: subprocess.CompletedProcess(cmd, 1, "", "boom"),
    )

    addon = addons.XamlLangFormatterAddon(Storage())
    translation = FakeTranslation("/repo/Lang/fr.xaml")

    outcome = addon.pre_commit(translation, "author", True)

    assert outcome is not None
    assert outcome.status == "error"
    assert "boom" in outcome.result
    assert translation.store_hash_calls == 0


def test_timeout_returns_error(binary, monkeypatch) -> None:
    def fake_run(cmd, **kwargs):
        raise subprocess.TimeoutExpired(cmd, 1)

    monkeypatch.setattr(addons.subprocess, "run", fake_run)

    addon = addons.XamlLangFormatterAddon(Storage())
    translation = FakeTranslation("/repo/Lang/fr.xaml")

    outcome = addon.pre_commit(translation, "author", True)

    assert outcome is not None
    assert outcome.status == "error"
    assert translation.store_hash_calls == 0


def test_configured_binary_missing_file(tmp_path, monkeypatch) -> None:
    monkeypatch.setattr(addons, "resolve_binary", lambda: None)

    addon = addons.XamlLangFormatterAddon(
        Storage({"binary": str(tmp_path / "missing")}),
    )
    translation = FakeTranslation("/repo/Lang/fr.xaml")

    outcome = addon.pre_commit(translation, "author", True)

    assert outcome is not None
    assert outcome.status == "error"


def test_configured_binary_present(tmp_path, monkeypatch) -> None:
    configured = tmp_path / "custom-xlf"
    configured.write_text("#!/bin/sh\n", encoding="utf-8")
    captured: dict = {}

    def fake_run(cmd, **kwargs):
        captured["cmd"] = cmd
        return subprocess.CompletedProcess(cmd, 0, "", "")

    monkeypatch.setattr(addons.subprocess, "run", fake_run)

    addon = addons.XamlLangFormatterAddon(Storage({"binary": str(configured)}))
    translation = FakeTranslation("/repo/Lang/fr.xaml")

    assert addon.pre_commit(translation, "author", True) is None
    assert captured["cmd"][0] == str(configured)
