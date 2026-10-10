---
name: release
description: Bump the version, then tag the merged commit so binaries get published
---

A release is two steps, because `main` only takes squash merges and the tag
must point at the squashed commit.

1. On the feature branch, run `python .agents/skills/release/release.py bump X.Y.Z`.
   It sets the version in `Cargo.toml`, refreshes `Cargo.lock`, runs fmt,
   clippy, and tests, and commits `Version X.Y.Z`. Push and merge the PR.
2. After the merge, run `python .agents/skills/release/release.py tag`. It
   reads the version on `origin/main`, tags that commit `vX.Y.Z`, and pushes
   the tag. Ask the user first: the tag starts `.github/workflows/release.yml`,
   which publishes binaries that `INSTALL.md` downloads as `latest`.

Pick the number with [[releases]] in mind: removing a doctor behavior is a
minor bump before 1.0.
