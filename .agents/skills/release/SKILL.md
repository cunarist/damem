---
name: release
description: Bump the version, then publish a GitHub release once it is merged
---

A release is two steps, because `main` only takes squash merges and the
release must point at the squashed commit.

1. On the feature branch, run `python .agents/skills/release/release.py bump X.Y.Z`.
   It sets the version in `Cargo.toml`, refreshes `Cargo.lock`, runs fmt,
   clippy, and tests, and commits `Version X.Y.Z`. Push and merge the PR.
2. After the merge, run `python .agents/skills/release/release.py publish`. It
   reads the version on `origin/main` and creates the `vX.Y.Z` release page on
   GitHub with generated notes, which also creates the tag. Ask the user first:
   the tag starts `.github/workflows/release.yml`, which attaches binaries to
   that page, and `INSTALL.md` downloads them as `latest`. If no binaries
   appear, run `gh workflow run release.yml -f tag=vX.Y.Z`.

Pick the number with [[releases]] in mind: removing a doctor behavior is a
minor bump before 1.0.
