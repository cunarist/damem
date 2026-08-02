mod doctor;
mod error;
mod init;
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
  init     Create .agents/{memory,skills,tmp} in this project
  recall   Print how to manage .agents/, and what is in it now
  doctor   Report anything inconsistent in .agents/

Options:
  -h, --help       Print this help
  -V, --version    Print the version
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
    "init" => init::run(&Layout::discover()?)?,
    "recall" => recall::run(&Layout::discover_initialized()?)?,
    "doctor" => {
      if !doctor::run(&Layout::discover_initialized()?)? {
        return Ok(ExitCode::FAILURE);
      }
    }
    other => return Err(Error::usage(format!("unknown command `{other}`"))),
  }
  Ok(ExitCode::SUCCESS)
}
