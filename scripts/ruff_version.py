#!/usr/bin/env python

import sys
from pathlib import Path
from textwrap import dedent

try:
    import tomllib
except ImportError:
    import tomli as tomllib


if __name__ == "__main__":
    version_file = Path(__file__).parent.parent / "ruff_api" / "__ruff_version__.py"
    version_tpl = dedent(
        '''\
        """
        Automatically generated by scripts/ruff_version.py

        Run `make version` after updating Cargo.lock to regenerate.
        """

        ruff_version = "{version}"
        '''
    )

    cargo_lock = Path(__file__).parent.parent / "Cargo.lock"
    cargo_data = tomllib.loads(cargo_lock.read_text())

    for package_data in cargo_data.get("package", []):
        package_name = package_data.get("name", "")
        if package_name == "ruff":
            package_version = package_data.get("version", "0")
            print(f"found ruff version {package_version}")
            version_file.write_text(version_tpl.format(version=package_version))
            sys.exit(0)

    print("ruff_version update failed")
    sys.exit(1)
