"""Weblate add-on for xaml-lang-formatter.

Formats WPF ``ResourceDictionary`` localization files with
``xaml-lang-formatter`` right before Weblate commits them, so the formatted
content lands in the same commit and no extra commit is created.
"""

from __future__ import annotations

__all__ = ["__version__"]

__version__ = "0.1.0"
