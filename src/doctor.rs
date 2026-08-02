use crate::error::{Result, display_relative, fs};
use crate::layout::Layout;
use crate::markdown;
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

/// Prints every problem. Returns whether `.agents/` is consistent.
pub fn run(layout: &Layout) -> Result<bool> {
  let problems = check(layout)?;
  for problem in &problems {
    println!("✗ {}: {}", problem.path, problem.message);
  }
  if problems.is_empty() {
    println!("✓ .agents is consistent");
  }
  Ok(problems.is_empty())
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
    problems.push(Problem::new(name, "missing; run `damem init`"));
    return Ok(());
  }
  if !fs::read_to_string(&gitignore)?
    .lines()
    .any(|line| line.trim() == "*")
  {
    problems.push(Problem::new(name, "does not ignore the directory contents"));
  }
  Ok(())
}

fn check_memory(layout: &Layout, problems: &mut Vec<Problem>) -> Result<()> {
  let dir = layout.memory_dir();
  let index = layout.memory_index();
  let index_name = rel(layout, &index);
  if !index.exists() {
    problems.push(Problem::new(index_name, "missing; run `damem init`"));
    return Ok(());
  }

  let files = markdown_files(&dir, "MEMORY.md")?;
  let body = markdown::strip_comments(&fs::read_to_string(&index)?);
  let listed = listed_targets(&body);

  for target in &listed {
    if !dir.join(target).exists() {
      problems.push(Problem::new(
        &index_name,
        format!("links to `{target}`, which does not exist"),
      ));
    }
  }
  for file in &files {
    if !listed.contains(file) {
      problems.push(Problem::new(
        rel(layout, &dir.join(file)),
        format!("not listed in {index_name}"),
      ));
    }
  }

  for file in &files {
    let path = dir.join(file);
    let text = markdown::strip_comments(&fs::read_to_string(&path)?);
    for name in markdown::wiki_links(&text) {
      if !dir.join(format!("{name}.md")).exists() {
        problems.push(Problem::new(
          rel(layout, &path),
          format!("links to `[[{name}]]`, which does not exist"),
        ));
      }
    }
  }
  Ok(())
}

fn check_skills(layout: &Layout, problems: &mut Vec<Problem>) -> Result<()> {
  let dir = layout.skills_dir();
  let index = layout.skills_index();
  let index_name = rel(layout, &index);
  if !index.exists() {
    problems.push(Problem::new(index_name, "missing; run `damem init`"));
    return Ok(());
  }

  let mut skills = BTreeSet::new();
  for entry in fs::read_dir_sorted(&dir)? {
    let path = entry.path();
    if !path.is_dir() {
      continue;
    }
    let Some(name) = file_name(&path) else {
      continue;
    };
    if !path.join("SKILL.md").exists() {
      problems.push(Problem::new(rel(layout, &path), "missing SKILL.md"));
    }
    skills.insert(name);
  }

  let body = markdown::strip_comments(&fs::read_to_string(&index)?);
  let listed: BTreeSet<String> = listed_targets(&body)
    .iter()
    .filter_map(|target| target.split('/').next().map(str::to_owned))
    .collect();

  for name in &listed {
    if !skills.contains(name) {
      problems.push(Problem::new(
        &index_name,
        format!("links to `{name}`, which is not a skill directory"),
      ));
    }
  }
  for name in &skills {
    if !listed.contains(name) {
      problems.push(Problem::new(
        rel(layout, &dir.join(name)),
        format!("not listed in {index_name}"),
      ));
    }
  }
  Ok(())
}

/// Link targets inside the project, with `./` and backslashes normalized away.
fn listed_targets(body: &str) -> BTreeSet<String> {
  markdown::link_targets(body)
    .iter()
    .filter(|target| !markdown::is_external(target))
    .map(|target| {
      target
        .replace('\\', "/")
        .trim_start_matches("./")
        .to_owned()
    })
    .collect()
}

fn markdown_files(dir: &Path, skip: &str) -> Result<BTreeSet<String>> {
  let mut files = BTreeSet::new();
  for entry in fs::read_dir_sorted(dir)? {
    let path = entry.path();
    if path.is_dir() || path.extension().is_none_or(|ext| ext != "md") {
      continue;
    }
    match file_name(&path) {
      Some(name) if name != skip => {
        files.insert(name);
      }
      _ => {}
    }
  }
  Ok(files)
}

fn file_name(path: &Path) -> Option<String> {
  path
    .file_name()
    .map(|name| name.to_string_lossy().into_owned())
}

fn rel(layout: &Layout, path: &Path) -> String {
  display_relative(path, layout.root())
}
