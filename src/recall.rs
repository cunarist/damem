use crate::error::{Result, display_relative, fs};
use crate::layout::Layout;
use std::path::Path;

const GUIDANCE: &str = include_str!("templates/recall.md");

/// Prints how to manage `.agents/`, followed by what is in it right now.
pub fn run(layout: &Layout) -> Result<()> {
  print!("{GUIDANCE}");
  println!();
  println!("---");
  for index in [layout.memory_index(), layout.skills_index()] {
    print_index(layout.root(), &index)?;
  }
  Ok(())
}

fn print_index(root: &Path, path: &Path) -> Result<()> {
  println!();
  println!("## `{}`", display_relative(path, root));
  println!();
  if path.exists() {
    print!("{}", fs::read_to_string(path)?);
  } else {
    println!("Missing. Run `damem init`.");
  }
  Ok(())
}
