# damem

A Rust CLI with no dependencies. Three commands: `init`, `recall`, `doctor`.

Run `damem recall` before you start working, and `damem doctor` before you finish.

## Rules

- `cargo fmt` and `cargo clippy --all-targets -- -D warnings` must both pass.
- No `unwrap`, `expect`, or `panic` outside tests. Every failure is an
  `Error` variant in [src/error.rs](src/error.rs) returned through `Result`.
- Two-space indentation, edition 2024.
- Keep the dependency list empty.
