//! Agents that do not read `AGENTS.md` on their own.
//!
//! Most tools read it directly. A few look for their own file first, so a
//! project that only has `AGENTS.md` is invisible to them. Each of those can
//! import it in one line, and `doctor` says so when the tool is installed here.

use crate::error::{Result, fs};
use crate::layout::Layout;
use std::env;
use std::path::{Path, PathBuf};

/// A one-line fix for an agent that is installed but cannot see `AGENTS.md`.
#[derive(Debug)]
pub struct Suggestion {
  pub path: String,
  pub message: String,
}

struct Agent {
  /// The tool's own instruction file, relative to the project root.
  file: &'static str,
  /// Where the tool keeps its configuration, in the project and in `$HOME`.
  marker: &'static str,
  /// What the file needs to contain.
  import: &'static str,
  label: &'static str,
}

const AGENTS: [Agent; 2] = [
  Agent {
    file: "CLAUDE.md",
    marker: ".claude",
    import: "@AGENTS.md",
    label: "Claude Code",
  },
  Agent {
    file: "GEMINI.md",
    marker: ".gemini",
    import: "@./AGENTS.md",
    label: "Gemini CLI",
  },
];

/// Nothing to suggest until the project actually has an `AGENTS.md`.
pub fn suggestions(layout: &Layout) -> Result<Vec<Suggestion>> {
  let root = layout.root();
  if !root.join("AGENTS.md").is_file() {
    return Ok(Vec::new());
  }

  let mut suggestions = Vec::new();
  for agent in &AGENTS {
    if !installed(root, agent.marker) {
      continue;
    }
    let file = root.join(agent.file);
    if !file.exists() {
      suggestions.push(Suggestion {
        path: agent.file.to_owned(),
        message: format!(
          "missing; {} does not read AGENTS.md. One line is enough: `{}`",
          agent.label, agent.import
        ),
      });
      continue;
    }
    // A symlink to AGENTS.md needs no import line.
    if std::fs::symlink_metadata(&file).is_ok_and(|meta| meta.is_symlink()) {
      continue;
    }
    if !imports_agents_md(&file)? {
      suggestions.push(Suggestion {
        path: agent.file.to_owned(),
        message: format!("does not import AGENTS.md; add `{}`", agent.import),
      });
    }
  }
  Ok(suggestions)
}

/// Whether the tool is configured for this project or for this user.
fn installed(root: &Path, marker: &str) -> bool {
  root.join(marker).is_dir() || home().is_some_and(|home| home.join(marker).is_dir())
}

fn home() -> Option<PathBuf> {
  env::var_os("HOME")
    .or_else(|| env::var_os("USERPROFILE"))
    .map(PathBuf::from)
}

/// An import is any `@…AGENTS.md` outside a code span, which is how both
/// Claude Code and Gemini CLI read it.
fn imports_agents_md(path: &Path) -> Result<bool> {
  let text = fs::read_to_string(path)?;
  let mut fenced = false;
  for line in text.lines() {
    if line.trim_start().starts_with("```") {
      fenced = !fenced;
      continue;
    }
    if fenced {
      continue;
    }
    // Odd-numbered parts sit inside backticks, so only even ones count.
    let mut outside_code_spans = line.split('`').step_by(2);
    if outside_code_spans.any(|part| part.contains("@AGENTS.md") || part.contains("@./AGENTS.md")) {
      return Ok(true);
    }
  }
  Ok(false)
}
