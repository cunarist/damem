//! End-to-end runs of the binary against a throwaway project directory.
//!
//! Tests may panic; that is how a failure is reported.
#![allow(clippy::expect_used, clippy::panic)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

struct Project {
  dir: PathBuf,
}

impl Project {
  fn new(label: &str) -> Self {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("damem-test-{}-{label}-{id}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create test project");
    // A `.git` directory makes the project root unambiguous.
    fs::create_dir_all(dir.join(".git")).expect("create .git");
    Self { dir }
  }

  fn run(&self, command: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_damem"))
      .arg(command)
      .current_dir(&self.dir)
      .output()
      .expect("run damem")
  }

  fn write(&self, relative: &str, contents: &str) {
    let path = self.dir.join(relative);
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent).expect("create parent");
    }
    fs::write(path, contents).expect("write file");
  }

  fn path(&self, relative: &str) -> PathBuf {
    self.dir.join(relative)
  }
}

impl Drop for Project {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.dir);
  }
}

fn stdout(output: &Output) -> String {
  String::from_utf8_lossy(&output.stdout).into_owned()
}

fn exists(path: &Path) -> bool {
  path.exists()
}

const DB_CHOICE: &str =
  "---\ndescription: Postgres over SQLite, for concurrent writes\n---\n\nSee [[api-style]].\n";
const API_STYLE: &str = "---\ndescription: Errors are typed, never strings\n---\n";
const LINT_SKILL: &str = "---\nname: lint\ndescription: Run ruff before every commit\n---\n\nRun `python lint.py --fix`.\n";

#[test]
fn init_creates_the_layout_and_is_idempotent() {
  let project = Project::new("init");
  assert!(project.run("init").status.success());
  assert!(exists(&project.path(".agents/memory")));
  assert!(exists(&project.path(".agents/skills")));
  assert!(exists(&project.path(".agents/tmp/.gitignore")));
  // No index files: every entry describes itself in its frontmatter.
  assert!(!exists(&project.path(".agents/memory/MEMORY.md")));
  assert!(!exists(&project.path(".agents/skills/SKILLS.md")));

  project.write(".agents/tmp/.gitignore", "*\n!keep.txt\n");
  let second = project.run("init");
  assert!(second.status.success());
  assert!(stdout(&second).contains("kept"));
  let kept = fs::read_to_string(project.path(".agents/tmp/.gitignore")).expect("read gitignore");
  assert!(kept.contains("!keep.txt"));
}

#[test]
fn doctor_passes_on_a_fresh_project() {
  let project = Project::new("clean");
  assert!(project.run("init").status.success());
  let output = project.run("doctor");
  assert!(output.status.success(), "{}", stdout(&output));
  assert!(stdout(&output).contains("consistent"));
}

#[test]
fn doctor_accepts_frontmatter_and_wiki_links() {
  let project = Project::new("linked");
  assert!(project.run("init").status.success());
  project.write(".agents/memory/db-choice.md", DB_CHOICE);
  project.write(".agents/memory/api-style.md", API_STYLE);
  project.write(".agents/skills/lint/SKILL.md", LINT_SKILL);

  let output = project.run("doctor");
  assert!(output.status.success(), "{}", stdout(&output));
}

#[test]
fn doctor_reports_missing_descriptions_and_dangling_links() {
  let project = Project::new("dirty");
  assert!(project.run("init").status.success());
  project.write(
    ".agents/memory/db-choice.md",
    "Postgres. See [[api-style]].\n",
  );
  project.write(".agents/skills/lint/notes.md", "no SKILL.md here\n");

  let output = project.run("doctor");
  assert!(!output.status.success());
  let report = stdout(&output);
  assert!(report.contains("no `description`"), "{report}");
  assert!(report.contains("[[api-style]]"), "{report}");
  assert!(report.contains("skills/lint/SKILL.md"), "{report}");
}

#[test]
fn recall_lists_what_each_file_describes() {
  let project = Project::new("recall");
  assert!(project.run("init").status.success());
  project.write(".agents/memory/db-choice.md", DB_CHOICE);
  project.write(".agents/memory/api-style.md", API_STYLE);
  project.write(".agents/skills/lint/SKILL.md", LINT_SKILL);

  let output = project.run("recall");
  assert!(output.status.success());
  let report = stdout(&output);
  assert!(
    report.contains("`db-choice.md` — Postgres over SQLite"),
    "{report}"
  );
  assert!(
    report.contains("`lint/` — Run ruff before every commit"),
    "{report}"
  );
}

#[test]
fn recall_says_when_nothing_is_stored_yet() {
  let project = Project::new("empty");
  assert!(project.run("init").status.success());
  let output = project.run("recall");
  assert!(output.status.success());
  assert_eq!(stdout(&output).matches("Empty so far.").count(), 2);
}

#[test]
fn commands_need_an_initialized_project() {
  let project = Project::new("bare");
  let output = project.run("recall");
  assert!(!output.status.success());
  assert!(String::from_utf8_lossy(&output.stderr).contains("damem init"));
}

#[test]
fn unknown_commands_fail() {
  let project = Project::new("unknown");
  let output = project.run("frobnicate");
  assert!(!output.status.success());
  assert!(String::from_utf8_lossy(&output.stderr).contains("unknown command"));
}
