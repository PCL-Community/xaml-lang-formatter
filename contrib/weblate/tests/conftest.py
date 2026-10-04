"""Test configuration.

The add-on is meant to run inside Weblate, which is not installed in the
development/CI environment used for the Rust project. To keep the pure logic
under test we install lightweight stubs for the ``django`` and ``weblate``
modules before importing the add-on package.
"""

from __future__ import annotations

import sys
import types
from pathlib import Path

SRC = Path(__file__).resolve().parent.parent / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))


def _module(name: str, **attrs: object) -> types.ModuleType:
    module = types.ModuleType(name)
    for key, value in attrs.items():
        setattr(module, key, value)
    sys.modules[name] = module
    return module


def _install_django_stubs() -> None:
    def gettext_lazy(value: object) -> object:
        return value

    def gettext(value: object) -> object:
        return value

    class _Field:
        def __init__(self, *args: object, **kwargs: object) -> None:
            self.args = args
            self.kwargs = kwargs

    translation = _module(
        "django.utils.translation",
        gettext_lazy=gettext_lazy,
        gettext=gettext,
    )
    forms = _module(
        "django.forms",
        CharField=_Field,
        IntegerField=_Field,
    )
    _module("django.utils", translation=translation)
    _module("django", utils=sys.modules["django.utils"], forms=forms)


def _install_weblate_stubs() -> None:
    class BaseAddon:
        def __init__(self, storage: object = None) -> None:
            self.instance = storage
            self.extra_files: list[str] = []
            self.alerts: list[dict] = []

        @property
        def configuration(self) -> dict:
            configuration = getattr(self.instance, "configuration", None)
            return configuration if configuration is not None else {}

    class CompatDict(dict):
        pass

    class AddonEvent:
        EVENT_PRE_COMMIT = "pre-commit"

    class AddonActivityLogReason:
        NOT_APPLICABLE = "not-applicable"
        REQUIRED_FILE_MISSING = "required-file-missing"

    class _Outcome:
        def __init__(self, status, reason=None, result=None) -> None:
            self.status = status
            self.reason = reason
            self.result = result

    class AddonEventOutcome:
        @staticmethod
        def skipped(reason):
            return _Outcome("skipped", reason=reason)

        @staticmethod
        def error(reason=None, result=None):
            return _Outcome("error", reason=reason, result=result)

    class BaseAddonForm:
        public_configuration_fields: frozenset[str] = frozenset()

        def __init__(self, *args: object, **kwargs: object) -> None:
            pass

    base = _module(
        "weblate.addons.base",
        BaseAddon=BaseAddon,
        CompatDict=CompatDict,
    )
    events = _module(
        "weblate.addons.events",
        AddonEvent=AddonEvent,
        AddonActivityLogReason=AddonActivityLogReason,
        AddonEventOutcome=AddonEventOutcome,
        AddonEventResult=object,
    )
    addon_forms = _module("weblate.addons.forms", BaseAddonForm=BaseAddonForm)
    commands = _module(
        "weblate.utils.commands",
        get_clean_env=lambda extra=None, extra_path=None: {},
    )
    _module("weblate.addons", base=base, events=events, forms=addon_forms)
    _module("weblate.utils", commands=commands)
    _module("weblate")


if "weblate" not in sys.modules:
    _install_django_stubs()
    _install_weblate_stubs()
