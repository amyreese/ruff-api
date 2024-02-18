# Copyright Amethyst Reese
# Licensed under the MIT license

from unittest import TestCase

import ruff_api


CODE_UNFORMATTED = """
import sys
def foo():
    "test function"
    print("something",
          file=sys.stderr)
foo()
"""

CODE_FORMATTED = """\
import sys


def foo():
    "test function"
    print("something", file=sys.stderr)


foo()
"""

CODE_FORMATTED_LL20 = """\
import sys


def foo():
    "test function"
    print(
        "something",
        file=sys.stderr,
    )


foo()
"""


class SmokeTest(TestCase):
    def test_basic(self) -> None:
        self.assertEqual(
            CODE_FORMATTED, ruff_api.format_string("hello.py", CODE_UNFORMATTED)
        )

    def test_basic_options(self) -> None:
        options = ruff_api.FormatOptions(line_width=20)
        self.assertEqual(
            CODE_FORMATTED_LL20,
            ruff_api.format_string("hello.py", CODE_UNFORMATTED, options),
        )
