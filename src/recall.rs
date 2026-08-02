use crate::entries::{self, Entry};
use crate::error::Result;
use crate::layout::Layout;

const GUIDANCE: &str = include_str!("templates/recall.md");

/// Prints how to manage `.agents/`, followed by what is in it right now.
pub fn run(layout: &Layout) -> Result<()> {
  print!("{GUIDANCE}");
  println!();
  println!("---");
  print_section("Memory", ".agents/memory/", &entries::memories(layout)?, "");
  print_section("Skills", ".agents/skills/", &entries::skills(layout)?, "/");
  Ok(())
}

fn print_section(title: &str, dir: &str, entries: &[Entry], suffix: &str) {
  println!();
  println!("## {title} — `{dir}`");
  println!();
  if entries.is_empty() {
    println!("Empty so far.");
    return;
  }
  for entry in entries {
    match &entry.description {
      Some(description) => println!("- `{}{suffix}` — {description}", entry.name),
      None => println!(
        "- `{}{suffix}` — no description in its frontmatter",
        entry.name
      ),
    }
  }
}
