use crate::error::{Result, display_relative, fs};
use crate::layout::Layout;

const TMP_GITIGNORE: &str = include_str!("templates/tmp-gitignore");

/// The line to paste into `AGENTS.md`.
pub const AGENTS_SNIPPET: &str = concat!(
  "Run [`damem recall`](https://github.com/cunarist/damem/releases) in the terminal ",
  "before starting any session. Run `damem --help` for details."
);

/// Creates `.agents/` without touching files that already exist.
pub fn run(layout: &Layout) -> Result<()> {
  for dir in [layout.memory_dir(), layout.skills_dir(), layout.tmp_dir()] {
    fs::create_dir_all(&dir)?;
    println!("ready   {}", display_relative(&dir, layout.root()));
  }

  let gitignore = layout.tmp_gitignore();
  let written = fs::write_if_absent(&gitignore, TMP_GITIGNORE)?;
  let label = if written { "created" } else { "kept   " };
  println!("{label} {}", display_relative(&gitignore, layout.root()));

  println!();
  println!("Tell your agents about it in AGENTS.md:");
  println!();
  println!("    {AGENTS_SNIPPET}");
  Ok(())
}
