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

CODE_UNSORTED_IMPORTS = """\
import sys, CUSTOMMOD
from firstparty import a, c,b
from sysmod import Z,x
import __strict__
import somecustom
import __static__
import __future__

def main(): pass
"""

CODE_SORTED_IMPORTS = """\
import __future__

import __static__
import __strict__

import sys

import CUSTOMMOD
import somecustom
from firstparty import a, b, c
from sysmod import x, Z


def main(): pass
"""

CODE_SORTED_IMPORTS_CUSTOM = """\
import __future__

import __static__
import __strict__

import sys
from sysmod import x, Z

import CUSTOMMOD
import somecustom

from firstparty import a, b, c


def main(): pass
"""


class SmokeTest(TestCase):
    def test_format(self) -> None:
        self.assertEqual(
            CODE_FORMATTED, ruff_api.format_string("hello.py", CODE_UNFORMATTED)
        )

    def test_format_basic_options(self) -> None:
        options = ruff_api.FormatOptions(line_width=20)
        self.assertEqual(
            CODE_FORMATTED_LL20,
            ruff_api.format_string("hello.py", CODE_UNFORMATTED, options),
        )

    def test_isort(self) -> None:
        self.assertEqual(
            CODE_SORTED_IMPORTS,
            ruff_api.isort_string("hello.py", CODE_UNSORTED_IMPORTS),
        )

    def test_isort_options(self) -> None:
        # missing sysmod
        options = ruff_api.SortOptions(["firstparty"], [])
        self.assertNotEqual(
            CODE_SORTED_IMPORTS,
            ruff_api.isort_string("hello.py", CODE_UNSORTED_IMPORTS, options),
        )

        # missing firstparty
        options = ruff_api.SortOptions([], ["sysmod"])
        self.assertNotEqual(
            CODE_SORTED_IMPORTS,
            ruff_api.isort_string("hello.py", CODE_UNSORTED_IMPORTS, options),
        )

        options = ruff_api.SortOptions(["firstparty"], ["sysmod"])
        self.assertEqual(
            CODE_SORTED_IMPORTS_CUSTOM,
            ruff_api.isort_string("hello.py", CODE_UNSORTED_IMPORTS, options),
        )
