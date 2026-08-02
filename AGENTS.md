At the start of every new session, run `damem recall` in the terminal and treat
its output as the context for this repository. If the `damem` command is not
available, do not proceed. Instead, download the latest version from the
[releases page](https://github.com/cunarist/damem/releases), install the binary
into `./.agents/scratch`, and then run `damem recall` before reasoning or
responding. Run `damem --help` for usage details.

# Rules

- Four dependencies, and adding a fifth needs a reason: `clap` for the command
  line, `anstream` and `anstyle` for color, `thiserror` for the error type.
  Parsing Markdown, reading files, and walking directories stay hand-written.
- Print with `anstream::println!`, never `std::println!`. anstream drops the
  escape codes when the output is piped, which is how `damem recall` stays clean
  inside an agent's context.
- Colors come from [src/style.rs](src/style.rs). No ad-hoc `\x1b[` anywhere.
- No `unwrap`, `expect`, or `panic` outside tests. Every failure is an `Error`
  variant in [src/error.rs](src/error.rs) returned through `Result`.
- `cargo fmt` and `cargo clippy --all-targets -- -D warnings` must both pass.
- Two-space indentation, edition 2024.
