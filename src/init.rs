use crate::error::{Result, display_relative, fs};
use crate::layout::Layout;
use std::path::Path;

const MEMORY_INDEX: &str = include_str!("templates/memory-index.md");
const SKILLS_INDEX: &str = include_str!("templates/skills-index.md");
const TMP_GITIGNORE: &str = include_str!("templates/tmp-gitignore");

/// The line to paste into `AGENTS.md`.
pub const AGENTS_SNIPPET: &str =
  "Run `damem recall` before you start working, and `damem doctor` before you finish.";

/// Creates `.agents/` without touching files that already exist.
pub fn run(layout: &Layout) -> Result<()> {
  for dir in [layout.memory_dir(), layout.skills_dir(), layout.tmp_dir()] {
    fs::create_dir_all(&dir)?;
  }

  let files = [
    (layout.memory_index(), MEMORY_INDEX),
    (layout.skills_index(), SKILLS_INDEX),
    (layout.tmp_gitignore(), TMP_GITIGNORE),
  ];
  for (path, contents) in &files {
    let written = fs::write_if_absent(path, contents)?;
    report(layout.root(), path, written);
  }

  println!();
  println!("Tell your agents about it in AGENTS.md:");
  println!();
  println!("    {AGENTS_SNIPPET}");
  Ok(())
}

fn report(root: &Path, path: &Path, written: bool) {
  let label = if written { "created" } else { "kept   " };
  println!("{label} {}", display_relative(path, root));
}
