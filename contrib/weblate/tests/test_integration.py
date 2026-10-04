"""End-to-end check against the bundled binary (skipped if not available)."""

from __future__ import annotations

from types import SimpleNamespace

import pytest

from xaml_lang_formatter_addon import addons, binaries

SAMPLE = """<ResourceDictionary
    xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
    xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
    xmlns:sys="clr-namespace:System;assembly=mscorlib"
    xml:space="preserve">

    <sys:String x:Key="Common.Action.Open">Open</sys:String>
    <sys:String x:Key="Meta.Code">en-US</sys:String>
    <sys:String x:Key="Common.App">App</sys:String>

</ResourceDictionary>
"""


class Storage:
    def __init__(self, configuration: dict | None = None) -> None:
        self.configuration = configuration if configuration is not None else {}


class FakeTranslation:
    def __init__(self, filename: str, full_path: str) -> None:
        self._filename = filename
        self.component = SimpleNamespace(full_path=full_path)
        self.store_hash_calls = 0

    def get_filename(self) -> str:
        return self._filename

    def store_hash(self) -> None:
        self.store_hash_calls += 1


@pytest.mark.skipif(binaries.resolve_binary() is None, reason="no bundled binary")
def test_pre_commit_formats_file_on_disk(tmp_path) -> None:
    target = tmp_path / "fr.xaml"
    target.write_text(SAMPLE, encoding="utf-8")

    addon = addons.XamlLangFormatterAddon(Storage())
    translation = FakeTranslation(str(target), str(tmp_path))

    assert addon.pre_commit(translation, "author", True) is None
    assert translation.store_hash_calls == 1

    formatted = target.read_text(encoding="utf-8")
    assert "<!-- Formatted by xaml-lang-formatter" in formatted
    assert "<!-- Meta -->" in formatted
    assert "<!-- Common -->" in formatted
    # Keys are sorted within the section.
    assert formatted.index("Common.Action.Open") < formatted.index("Common.App")
