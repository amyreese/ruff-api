# Copyright Amethyst Reese
# Licensed under the MIT license

"""
Validate project formatting using ruff_api directly
"""

import sys
from difflib import unified_diff
from itertools import chain
from pathlib import Path

from ruff_api import format_string, isort_string

ROOT = Path(__file__).parent.parent.resolve()
PROJECT_DIR = ROOT / "ruff_api"
SCRIPTS_DIR = ROOT / "scripts"


def diff(original: str, modified: str) -> str:
    a = original.splitlines()
    b = modified.splitlines()
    d = unified_diff(a, b, "original", "modified", n=2, lineterm="")
    return "\n".join(d)


def validate(path: Path) -> int:
    original = path.read_text("utf-8")
    filename = path.relative_to(ROOT).as_posix()

    exit_code = 0
    if (modified := format_string(filename, original)) != original:
        print(f"ruff format {filename}:")
        print(diff(original, modified))
        exit_code = 1

    if (modified := isort_string(filename, original)) != original:
        print(f"ruff isort {filename}:")
        print(diff(original, modified))
        exit_code = 1

    return exit_code


def main() -> None:
    exit_code = 0
    for file_path in chain(
        SCRIPTS_DIR.glob("**/*.py"),
        PROJECT_DIR.glob("**/*.py"),
    ):
        exit_code |= validate(file_path)

    print("error" if exit_code else "ok")
    sys.exit(exit_code)


if __name__ == "__main__":
    main()
