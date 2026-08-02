//! Scans `.agents/memory` and `.agents/skills` and reads their frontmatter.

use crate::error::{Result, fs};
use crate::frontmatter;
use crate::layout::Layout;
use std::path::{Path, PathBuf};

/// Frontmatter lives at the top of a file, so this much of it is always enough.
const HEAD_BYTES: usize = 8 * 1024;

/// One memory file, or one skill directory.
#[derive(Debug)]
pub struct Entry {
  /// `db-choice.md` for a memory, `lint` for a skill.
  pub name: String,
  /// The memory file, or the skill's `SKILL.md`.
  pub path: PathBuf,
  /// The `description` field, when the file has one.
  pub description: Option<String>,
}

pub fn memories(layout: &Layout) -> Result<Vec<Entry>> {
  let mut entries = Vec::new();
  for path in markdown_files(&layout.memory_dir())? {
    let Some(name) = file_name(&path) else {
      continue;
    };
    let description = describe(&path)?;
    entries.push(Entry {
      name,
      path,
      description,
    });
  }
  Ok(entries)
}

pub fn skills(layout: &Layout) -> Result<Vec<Entry>> {
  let mut entries = Vec::new();
  for entry in fs::read_dir_sorted(&layout.skills_dir())? {
    let dir = entry.path();
    if !dir.is_dir() {
      continue;
    }
    let Some(name) = file_name(&dir) else {
      continue;
    };
    let path = dir.join("SKILL.md");
    let description = if path.is_file() {
      describe(&path)?
    } else {
      None
    };
    entries.push(Entry {
      name,
      path,
      description,
    });
  }
  Ok(entries)
}

/// Markdown files directly inside `dir`, sorted by name.
pub fn markdown_files(dir: &Path) -> Result<Vec<PathBuf>> {
  let mut files = Vec::new();
  for entry in fs::read_dir_sorted(dir)? {
    let path = entry.path();
    if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
      files.push(path);
    }
  }
  Ok(files)
}

fn describe(path: &Path) -> Result<Option<String>> {
  let head = fs::read_head(path, HEAD_BYTES)?;
  Ok(frontmatter::parse(&head).and_then(|front| front.get("description").map(str::to_owned)))
}

fn file_name(path: &Path) -> Option<String> {
  path
    .file_name()
    .map(|name| name.to_string_lossy().into_owned())
}
