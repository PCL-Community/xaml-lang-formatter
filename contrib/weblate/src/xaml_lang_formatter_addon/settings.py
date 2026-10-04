"""Docker settings module for the xaml-lang-formatter add-on.

Point ``DJANGO_SETTINGS_MODULE`` at this module to enable the add-on in a
custom Weblate Docker image::

    ENV DJANGO_SETTINGS_MODULE=xaml_lang_formatter_addon.settings

It imports the official Docker settings and appends the add-on class to
``WEBLATE_ADDONS``.
"""

from __future__ import annotations

import weblate.settings_docker as _weblate_settings
from weblate.settings_docker import *  # noqa: F403

WEBLATE_ADDONS = (
    *_weblate_settings.WEBLATE_ADDONS,
    "xaml_lang_formatter_addon.addons.XamlLangFormatterAddon",
)
