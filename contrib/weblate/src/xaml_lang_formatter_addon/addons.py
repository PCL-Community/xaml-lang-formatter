"""Weblate pre-commit add-on running xaml-lang-formatter.

Weblate writes the current translation to disk and then fires the
``vcs_pre_commit`` signal before creating the git commit, including
``translation.filenames`` in that commit. Rewriting the file in this hook
therefore lands the formatted content in the same commit and never creates an
extra one.
"""

from __future__ import annotations

import subprocess
from pathlib import Path
from typing import ClassVar

from django.utils.translation import gettext_lazy
from weblate.addons.base import BaseAddon, CompatDict
from weblate.addons.events import (
    AddonActivityLogReason,
    AddonEvent,
    AddonEventOutcome,
    AddonEventResult,
)
from weblate.addons.forms import BaseAddonForm
from weblate.utils.commands import get_clean_env

from .binaries import resolve_binary
from .core import (
    DEFAULT_GROUP_THRESHOLD,
    DEFAULT_TIMEOUT,
    build_command,
    coerce_int,
    resolve_configured_binary,
)
from .forms import XamlLangFormatterForm


class XamlLangFormatterAddon(BaseAddon):
    """Format ResourceDictionary files before they are committed."""

    compat: ClassVar[CompatDict] = {"file_format": {"resourcedictionary"}}
    events: ClassVar[set[AddonEvent]] = {AddonEvent.EVENT_PRE_COMMIT}
    name = "xaml.lang_formatter"
    verbose = gettext_lazy("Format ResourceDictionary (xaml-lang-formatter)")
    description = gettext_lazy(
        "Runs xaml-lang-formatter on the translation file just before it is "
        "committed. The formatted output is part of the same commit, so no "
        "extra commit is created.",
    )
    icon = "cog.svg"
    version_added = "0.1.0"
    settings_form: type[BaseAddonForm] | None = XamlLangFormatterForm

    def _get_binary(self) -> Path | None:
        configured = resolve_configured_binary(self.configuration.get("binary"))
        if configured is not None:
            return configured
        return resolve_binary()

    def _get_int(self, key: str, default: int) -> int:
        return coerce_int(self.configuration.get(key), default)

    def pre_commit(
        self,
        translation,
        author: str,
        store_hash: bool,
        activity_log_id: int | None = None,
    ) -> AddonEventResult:
        filename = translation.get_filename()
        if not filename:
            return AddonEventOutcome.skipped(AddonActivityLogReason.NOT_APPLICABLE)

        binary = self._get_binary()
        if binary is None:
            return AddonEventOutcome.error(
                AddonActivityLogReason.REQUIRED_FILE_MISSING,
                result=(
                    "xaml-lang-formatter binary is not available. Install a "
                    "bundled platform build, set the binary path in the add-on "
                    "configuration, or point XAML_LANG_FORMATTER_BINARY at a "
                    "custom build."
                ),
            )

        group_threshold = self._get_int("group_threshold", DEFAULT_GROUP_THRESHOLD)
        timeout = self._get_int("timeout", DEFAULT_TIMEOUT)

        cmd = build_command(binary, filename, group_threshold)

        try:
            result = subprocess.run(
                cmd,
                cwd=translation.component.full_path,
                env=get_clean_env(),
                capture_output=True,
                text=True,
                timeout=timeout,
                check=False,
            )
        except (OSError, subprocess.SubprocessError) as error:
            return AddonEventOutcome.error(result=str(error))

        if result.returncode != 0:
            message = (result.stderr or result.stdout or "formatter failed").strip()
            return AddonEventOutcome.error(result=message)

        if store_hash:
            translation.store_hash()

        return None
