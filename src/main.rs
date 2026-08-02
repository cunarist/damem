mod agents;
mod doctor;
mod entries;
mod error;
mod frontmatter;
mod layout;
mod markdown;
mod recall;
mod style;

use clap::{Parser, Subcommand};
use error::Result;
use layout::Layout;
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
  name = "damem",
  bin_name = "damem",
  version,
  about = "damem tells agents to manage your project memory and skills without branding them.",
  after_help = "damem never writes to your project. The agent creates the files.",
  styles = style::clap_styles()
)]
struct Cli {
  #[command(subcommand)]
  command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
  /// Print how to manage .agents/, and what every file there describes
  Recall,
  /// Report anything inconsistent in .agents/
  Doctor,
}

fn main() -> ExitCode {
  match run() {
    Ok(code) => code,
    Err(error) => {
      let red = style::PROBLEM;
      anstream::eprintln!("{red}damem:{red:#} {error}");
      ExitCode::FAILURE
    }
  }
}

fn run() -> Result<ExitCode> {
  let cli = Cli::parse();
  let layout = Layout::discover()?;
  match cli.command {
    Command::Recall => recall::run(&layout)?,
    Command::Doctor => {
      if !doctor::run(&layout)? {
        return Ok(ExitCode::FAILURE);
      }
    }
  }
  Ok(ExitCode::SUCCESS)
}
