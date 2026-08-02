use crate::agents;
use crate::entries;
use crate::error::{Result, display_relative, fs};
use crate::layout::Layout;
use crate::markdown;
use crate::style;
use anstyle::Style;
use std::collections::BTreeSet;
use std::path::Path;

/// One thing that is wrong with `.agents/`.
#[derive(Debug)]
pub struct Problem {
  pub path: String,
  pub message: String,
}

impl Problem {
  fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
    Self {
      path: path.into(),
      message: message.into(),
    }
  }
}

/// Prints every problem, then anything worth suggesting. Returns whether
/// `.agents/` is consistent; suggestions do not make it fail.
pub fn run(layout: &Layout) -> Result<bool> {
  let problems = check(layout)?;
  let suggestions = agents::suggestions(layout)?;

  anstream::println!();
  for problem in &problems {
    entry(style::PROBLEM, "✗", &problem.path, &problem.message);
  }
  for suggestion in &suggestions {
    entry(
      style::SUGGESTION,
      "→",
      &suggestion.path,
      &suggestion.message,
    );
  }
  if problems.is_empty() && suggestions.is_empty() {
    entry(style::OK, "✓", ".agents", "everything here is consistent");
  }
  summary(problems.len(), suggestions.len());

  Ok(problems.is_empty())
}

/// One finding: a marked path, then the detail indented under it.
fn entry(mark: Style, symbol: &str, path: &str, message: &str) {
  let bold = style::PATH;
  anstream::println!("  {mark}{symbol}{mark:#}  {bold}{path}{bold:#}");
  anstream::println!("     {message}");
  anstream::println!();
}

fn summary(problems: usize, suggestions: usize) {
  if problems == 0 && suggestions == 0 {
    return;
  }
  let dim = style::DIM;
  anstream::println!(
    "  {dim}{} problem{}, {} suggestion{}{dim:#}",
    problems,
    plural(problems),
    suggestions,
    plural(suggestions)
  );
  anstream::println!();
}

fn plural(count: usize) -> &'static str {
  if count == 1 { "" } else { "s" }
}

pub fn check(layout: &Layout) -> Result<Vec<Problem>> {
  let mut problems = Vec::new();
  check_tmp(layout, &mut problems)?;
  check_memory(layout, &mut problems)?;
  check_skills(layout, &mut problems)?;
  Ok(problems)
}

fn check_tmp(layout: &Layout, problems: &mut Vec<Problem>) -> Result<()> {
  let gitignore = layout.tmp_gitignore();
  let name = rel(layout, &gitignore);
  if !gitignore.exists() {
    // Only worth reporting once the directory it guards is actually in use.
    if layout.tmp_dir().is_dir() {
      problems.push(Problem::new(name, "missing; it should hold `*`"));
    }
    return Ok(());
  }
  if !fs::read_to_string(&gitignore)?
    .lines()
    .any(|line| line.trim() == "*")
  {
    problems.push(Problem::new(
      name,
      "does not ignore the directory contents; it should hold `*`",
    ));
  }
  Ok(())
}

fn check_memory(layout: &Layout, problems: &mut Vec<Problem>) -> Result<()> {
  let dir = layout.memory_dir();
  let memories = entries::memories(layout)?;
  let names: BTreeSet<&str> = memories
    .iter()
    .filter_map(|entry| entry.name.strip_suffix(".md"))
    .collect();

  for entry in &memories {
    if entry.description.is_none() {
      problems.push(Problem::new(
        rel(layout, &entry.path),
        "no `description` in its frontmatter",
      ));
    }
    // Wiki links can point anywhere in the body, so this needs the whole file.
    let text = markdown::strip_comments(&fs::read_to_string(&entry.path)?);
    for link in markdown::wiki_links(&text) {
      if !names.contains(link.as_str()) {
        problems.push(Problem::new(
          rel(layout, &entry.path),
          format!("links to `[[{link}]]`, which is not a memory here"),
        ));
      }
    }
  }

  for path in strays(&dir)? {
    let message = if path.is_dir() {
      // Nothing reads these, so they would sit there unnoticed.
      "memories should be flat; move the files up into `.agents/memory/`"
    } else {
      "memories should be Markdown; give it a `.md` name or move it out"
    };
    problems.push(Problem::new(rel(layout, &path), message));
  }
  Ok(())
}

fn check_skills(layout: &Layout, problems: &mut Vec<Problem>) -> Result<()> {
  for entry in entries::skills(layout)? {
    if !entry.path.is_file() {
      problems.push(Problem::new(
        rel(layout, &entry.path),
        "missing; every skill directory needs one",
      ));
      continue;
    }
    if entry.description.is_none() {
      problems.push(Problem::new(
        rel(layout, &entry.path),
        "no `description` in its frontmatter",
      ));
    }
  }

  let dir = layout.skills_dir();
  if dir.is_dir() {
    for path in fs::read_dir_sorted(&dir)? {
      let path = path.path();
      if path.is_file() {
        problems.push(Problem::new(
          rel(layout, &path),
          "not a skill directory; each skill is a folder with a SKILL.md",
        ));
      }
    }
  }
  Ok(())
}

/// Anything in the memory directory that `recall` cannot list: subdirectories,
/// and files that are not Markdown.
fn strays(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
  let mut strays = Vec::new();
  for entry in fs::read_dir_sorted(dir)? {
    let path = entry.path();
    let listed = path.is_file() && path.extension().is_some_and(|ext| ext == "md");
    if !listed {
      strays.push(path);
    }
  }
  Ok(strays)
}

fn rel(layout: &Layout, path: &Path) -> String {
  display_relative(path, layout.root())
}
