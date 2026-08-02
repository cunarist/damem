use crate::entries::{self, Entry};
use crate::error::Result;
use crate::layout::Layout;
use crate::style;

const GUIDANCE: &str = include_str!("templates/recall.md");

/// Prints how to manage `.agents/`, followed by what is in it right now.
pub fn run(layout: &Layout) -> Result<()> {
  anstream::print!("{GUIDANCE}");
  anstream::println!();
  anstream::println!("---");
  section("Memory", ".agents/memory/", &entries::memories(layout)?, "");
  section("Skills", ".agents/skills/", &entries::skills(layout)?, "/");
  Ok(())
}

fn section(title: &str, dir: &str, entries: &[Entry], suffix: &str) {
  let heading = style::HEADING;
  let path = style::PATH;
  let dim = style::DIM;

  anstream::println!();
  anstream::println!("{heading}## {title} — `{dir}`{heading:#}");
  anstream::println!();
  if entries.is_empty() {
    anstream::println!("Empty so far.");
    return;
  }
  for entry in entries {
    match &entry.description {
      Some(description) => {
        anstream::println!("- {path}`{}{suffix}`{path:#} — {description}", entry.name);
      }
      None => anstream::println!(
        "- {path}`{}{suffix}`{path:#} — {dim}no description in its frontmatter{dim:#}",
        entry.name
      ),
    }
  }
}
