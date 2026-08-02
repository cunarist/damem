mod doctor;
mod entries;
mod error;
mod frontmatter;
mod layout;
mod markdown;
mod recall;

use error::{Error, Result};
use layout::Layout;
use std::env;
use std::process::ExitCode;

const HELP: &str = "\
damem tells agents to manage your project memory and skills without branding them.

Usage: damem <command>

Commands:
  recall   Print how to manage .agents/, and what every file there describes
  doctor   Report anything inconsistent in .agents/

Options:
  -h, --help       Print this help
  -V, --version    Print the version

damem never writes to your project. The agent creates the files.
";

fn main() -> ExitCode {
  match run() {
    Ok(code) => code,
    Err(error) => {
      eprintln!("damem: {error}");
      if matches!(error, Error::Usage { .. }) {
        eprintln!("try `damem --help`");
      }
      ExitCode::FAILURE
    }
  }
}

fn run() -> Result<ExitCode> {
  let args: Vec<String> = env::args().skip(1).collect();
  let Some(command) = args.first() else {
    print!("{HELP}");
    return Ok(ExitCode::SUCCESS);
  };
  if let Some(extra) = args.get(1) {
    return Err(Error::usage(format!("unexpected argument `{extra}`")));
  }

  match command.as_str() {
    "-h" | "--help" | "help" => print!("{HELP}"),
    "-V" | "--version" => println!("damem {}", env!("CARGO_PKG_VERSION")),
    "recall" => recall::run(&Layout::discover()?)?,
    "doctor" => {
      if !doctor::run(&Layout::discover()?)? {
        return Ok(ExitCode::FAILURE);
      }
    }
    other => return Err(Error::usage(format!("unknown command `{other}`"))),
  }
  Ok(ExitCode::SUCCESS)
}
