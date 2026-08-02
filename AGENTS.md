# damem

A Rust CLI. Two commands, `recall` and `doctor`, both read-only.

Run `damem recall` before you start working, and `damem doctor` before you finish.

## Rules

- **Zero dependencies.** `Cargo.toml` has no `[dependencies]` section and never
  gets one — not for argument parsing, not for YAML, not for error types. A tool
  that agents install from a release binary has to build anywhere, in seconds,
  with nothing to audit. If a crate seems necessary, the feature is too big.
- No `unwrap`, `expect`, or `panic` outside tests. Every failure is an `Error`
  variant in [src/error.rs](src/error.rs) returned through `Result`.
- `cargo fmt` and `cargo clippy --all-targets -- -D warnings` must both pass.
- Two-space indentation, edition 2024.
