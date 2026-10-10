---
description: Dropping doctor behavior is a minor bump; GitHub allows squash merges only
---

Before 1.0, removing a user-visible doctor behavior gets a minor version bump
(0.2.2 to 0.3.0 for CLAUDE.md, 0.3.0 to 0.4.0 for GEMINI.md). The bump touches
`Cargo.toml` and `Cargo.lock` and is committed separately as `Version X.Y.Z`.

The GitHub repository only allows squash merges, so a PR's commits, including
the version bump, land on `main` as one commit titled after the PR. CI runs
format, lint, and tests on Ubuntu, macOS, and Windows for every PR.

Run the `release` skill for both steps, including the tag that publishes
binaries. Related: [[no-per-agent-instruction-files]].
