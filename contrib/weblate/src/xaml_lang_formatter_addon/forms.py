"""Configuration form for the xaml-lang-formatter add-on."""

from __future__ import annotations

from django import forms
from django.utils.translation import gettext_lazy
from weblate.addons.forms import BaseAddonForm


class XamlLangFormatterForm(BaseAddonForm):
    public_configuration_fields = frozenset({"binary", "group_threshold", "timeout"})

    binary = forms.CharField(
        label=gettext_lazy("xaml-lang-formatter binary path"),
        required=False,
        help_text=gettext_lazy(
            "Leave empty to use the bundled binary for this platform. "
            "Set an absolute path to use a custom build."
        ),
    )
    group_threshold = forms.IntegerField(
        label=gettext_lazy("Nested group threshold"),
        min_value=1,
        initial=8,
        required=False,
        help_text=gettext_lazy(
            "Minimum number of resources sharing a prefix before a nested "
            "section is created."
        ),
    )
    timeout = forms.IntegerField(
        label=gettext_lazy("Timeout (seconds)"),
        min_value=1,
        initial=60,
        required=False,
        help_text=gettext_lazy(
            "Maximum time allowed for formatting a single translation file."
        ),
    )
