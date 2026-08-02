//! End-to-end runs of the binary against a throwaway project directory.
//!
//! Tests may panic; that is how a failure is reported.
#![allow(clippy::expect_used, clippy::panic)]

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

const DB_CHOICE: &str =
  "---\ndescription: Postgres over SQLite, for concurrent writes\n---\n\nSee [[api-style]].\n";
const API_STYLE: &str = "---\ndescription: Errors are typed, never strings\n---\n";
const LINT_SKILL: &str = "---\nname: lint\ndescription: Run ruff before every commit\n---\n\nRun `python lint.py --fix`.\n";

struct Project {
  dir: PathBuf,
}

impl Project {
  /// A project with `.agents/` laid out the way the guidance describes.
  fn new(label: &str) -> Self {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("damem-test-{}-{label}-{id}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    // A `.git` directory makes the project root unambiguous.
    for sub in [".git", ".agents/memory", ".agents/skills", ".agents/tmp"] {
      fs::create_dir_all(dir.join(sub)).expect("create test project");
    }
    let project = Self { dir };
    project.write(".agents/tmp/.gitignore", "*\n!.gitignore\n");
    project
  }

  /// A project that has never heard of damem.
  fn bare(label: &str) -> Self {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("damem-test-{}-{label}-{id}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join(".git")).expect("create test project");
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
}

impl Drop for Project {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.dir);
  }
}

fn stdout(output: &Output) -> String {
  String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn doctor_passes_on_an_empty_layout() {
  let project = Project::new("clean");
  let output = project.run("doctor");
  assert!(output.status.success(), "{}", stdout(&output));
  assert!(stdout(&output).contains("consistent"));
}

#[test]
fn doctor_accepts_frontmatter_and_wiki_links() {
  let project = Project::new("linked");
  project.write(".agents/memory/db-choice.md", DB_CHOICE);
  project.write(".agents/memory/api-style.md", API_STYLE);
  project.write(".agents/skills/lint/SKILL.md", LINT_SKILL);

  let output = project.run("doctor");
  assert!(output.status.success(), "{}", stdout(&output));
}

#[test]
fn doctor_reports_missing_descriptions_and_dangling_links() {
  let project = Project::new("dirty");
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
fn doctor_wants_the_tmp_directory_ignored() {
  let project = Project::new("tmp");
  project.write(".agents/tmp/.gitignore", "# nothing ignored\n");

  let output = project.run("doctor");
  assert!(!output.status.success());
  assert!(
    stdout(&output).contains("should hold `*`"),
    "{}",
    stdout(&output)
  );
}

#[test]
fn recall_lists_what_each_file_describes() {
  let project = Project::new("recall");
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
  let output = project.run("recall");
  assert!(output.status.success());
  assert_eq!(stdout(&output).matches("Empty so far.").count(), 2);
}

#[test]
fn commands_work_before_anything_exists() {
  let project = Project::bare("bare");
  let recall = project.run("recall");
  assert!(recall.status.success(), "{}", stdout(&recall));
  assert!(stdout(&recall).contains("Empty so far."));
  // Nothing was created: damem only reads.
  assert!(!project.dir.join(".agents").exists());

  let doctor = project.run("doctor");
  assert!(doctor.status.success(), "{}", stdout(&doctor));
}

#[test]
fn unknown_commands_fail() {
  let project = Project::bare("unknown");
  let output = project.run("frobnicate");
  assert!(!output.status.success());
  assert!(String::from_utf8_lossy(&output.stderr).contains("unknown command"));
}
