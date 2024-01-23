# Simple script to initialize template project

import re
import shlex
import subprocess
import sys
from pathlib import Path

CONFIG_REGEX = re.compile(r"^(\w+)\s*=\s*(.*)\s*$")
REPONAME_REGEX = re.compile(r"[^A-Z0-9._-]", re.IGNORECASE)
NAMESPACE_REGEX = re.compile(r"[-_.\W]+")

YES = object()
NO = object()


def prompt(prompt, value=None):
    if value is YES:
        prompt = f"{prompt} [Y/n] "
        value = "y"
    elif value is NO:
        prompt = f"{prompt} [y/N] "
        value = "n"
    elif value:
        prompt = f"{prompt} [{value}]: "
    else:
        prompt = f"{prompt}: "

    result = input(prompt)
    if value and not result:
        result = value

    return result


def run(cmd, **kwargs):
    line = " ".join(shlex.quote(c) for c in cmd)
    print(f"$ {line}")
    return subprocess.run(cmd, **kwargs)


def load_config():
    path = Path.home() / ".config" / "pyproject-template.conf"
    config = {}

    if path.is_file():
        for line in path.read_text().splitlines():
            if match := CONFIG_REGEX.match(line):
                key, value = match.groups()
                config[key] = value

        author = config.get("author", "")
        email = config.get("email", "")
        username = config.get("username", "")

        return author, email, username

    return "", "", ""


def main():
    author, email, username = load_config()
    reponame = Path(__file__).parent.name
    project = namespace = description = correct = ""

    while correct.lower() != "y":
        author = prompt("Author name", author)
        email = prompt("Email address", email)
        username = prompt("Github username/organization", username)
        reponame = reponame or REPONAME_REGEX.sub("-", project.lower()).strip("-")
        reponame = prompt("Repository name", reponame)
        project = prompt("Project name", project or reponame)
        namespace = namespace or NAMESPACE_REGEX.sub("_", reponame.lower()).strip("_")
        namespace = prompt("Package namespace", namespace)
        normalized = namespace.replace("_", "-")
        description = prompt("Short description", description)

        print()
        print(f"{username = !r}")
        print(f"{author = !r}")
        print(f"{email = !r}")
        print(f"{reponame = !r}")
        print(f"{project = !r}")
        print(f"{namespace = !r} ({normalized = !r})")
        print(f"{description = !r}")
        correct = prompt("\nCorrect?", NO)
        print()

    root = Path(__file__).parent
    queue = list(root.iterdir())

    pkgdir = root / "PACKAGE"
    if pkgdir.is_dir():
        run(["git", "mv", pkgdir.as_posix(), pkgdir.with_name(namespace).as_posix()])

    proc = run(["git", "ls-files"], encoding="utf-8", capture_output=True)
    queue = [Path(p) for p in proc.stdout.splitlines()]
    for path in queue:
        if path.name == "init.py":
            continue

        original = path.read_text()
        updated = (
            original.replace("GITHUB_USERNAME", username)
            .replace("GITHUB_PROJECT", reponame)
            .replace("AUTHOR_NAME", author)
            .replace("AUTHOR_EMAIL", email)
            .replace("PROJECT_NAME", project)
            .replace("PACKAGE_NAME", namespace)
            .replace("PACKAGE_NORMALIZED", normalized)
            .replace("PROJECT_DESCRIPTION", description)
        )

        if path.name == "makefile" and "@python init.py" in updated:
            updated = "\n".join(updated.splitlines()[:-2])

        if original != updated:
            print(f"Writing {path}")
            path.write_text(updated)

    run(["git", "add", "."])
    run(["git", "rm", __file__])

    commit = prompt("\nCommit changes?", YES)
    if commit.lower() == "y":
        run(["git", "commit"])


if __name__ == "__main__":
    main()
