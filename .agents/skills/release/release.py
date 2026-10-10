"""Bump damem's version, or publish a GitHub release for what landed on main."""

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
VERSION_LINE = re.compile(r'^version = "([^"]+)"$', re.MULTILINE)


def run(*args: str) -> str:
    """Run a command in the repository root and fail loudly if it fails."""
    result = subprocess.run(args, cwd=ROOT, check=True, capture_output=True, text=True)
    return result.stdout.strip()


def bump(version: str) -> None:
    manifest = ROOT / "Cargo.toml"
    text = manifest.read_text()
    # The first `version = ` line is the package's own; dependencies sit below it.
    manifest.write_text(VERSION_LINE.sub(f'version = "{version}"', text, count=1))
    run("cargo", "update", "--workspace")  # rewrites only damem's entry in Cargo.lock
    run("cargo", "fmt", "--all", "--check")
    run("cargo", "clippy", "--all-targets", "--", "-D", "warnings")
    run("cargo", "test", "--all-targets")
    run("git", "add", "Cargo.toml", "Cargo.lock")
    run("git", "commit", "-m", f"Version {version}")
    print(f"Committed Version {version}; push and merge the PR, then run `publish`.")


def publish() -> None:
    run("git", "fetch", "--tags", "origin", "main")
    manifest = run("git", "show", "origin/main:Cargo.toml")
    match = VERSION_LINE.search(manifest)
    if match is None:
        sys.exit("No version line in Cargo.toml on origin/main.")
    name = f"v{match.group(1)}"
    if run("git", "tag", "--list", name):
        sys.exit(f"{name} already exists; bump the version first.")
    commit = run("git", "rev-parse", "origin/main")
    # Creating the release creates the tag too, and the tag push starts the
    # Release workflow, which uploads binaries to this same release page.
    url = run(
        "gh", "release", "create", name,
        "--target", commit, "--title", name, "--generate-notes",
    )
    print(f"Published {url}; the Release workflow is attaching binaries now.")


def main(argv: list[str]) -> None:
    match argv:
        case ["bump", version] if re.fullmatch(r"\d+\.\d+\.\d+", version):
            bump(version)
        case ["publish"]:
            publish()
        case _:
            sys.exit("usage: release.py bump X.Y.Z | release.py publish")


if __name__ == "__main__":
    main(sys.argv[1:])
