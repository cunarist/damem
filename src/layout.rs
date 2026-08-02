use crate::error::{Error, Result};
use std::env;
use std::path::{Path, PathBuf};

/// Where the project keeps its agent-facing files.
#[derive(Debug, Clone)]
pub struct Layout {
  root: PathBuf,
}

impl Layout {
  pub fn new(root: PathBuf) -> Self {
    Self { root }
  }

  /// The project root: the nearest ancestor holding `.agents`, else the nearest
  /// holding `.git`, else the current directory.
  pub fn discover() -> Result<Self> {
    let cwd = env::current_dir().map_err(|source| Error::NoCurrentDir { source })?;
    let by_agents = find_ancestor(&cwd, ".agents");
    let root = by_agents
      .or_else(|| find_ancestor(&cwd, ".git"))
      .unwrap_or(cwd);
    Ok(Self::new(root))
  }

  /// Like [`Self::discover`], but fails when `.agents` is missing.
  pub fn discover_initialized() -> Result<Self> {
    let layout = Self::discover()?;
    if layout.agents_dir().is_dir() {
      Ok(layout)
    } else {
      Err(Error::NotInitialized {
        root: layout.root.clone(),
      })
    }
  }

  pub fn root(&self) -> &Path {
    &self.root
  }

  pub fn agents_dir(&self) -> PathBuf {
    self.root.join(".agents")
  }

  pub fn memory_dir(&self) -> PathBuf {
    self.agents_dir().join("memory")
  }

  pub fn skills_dir(&self) -> PathBuf {
    self.agents_dir().join("skills")
  }

  pub fn tmp_dir(&self) -> PathBuf {
    self.agents_dir().join("tmp")
  }

  pub fn memory_index(&self) -> PathBuf {
    self.memory_dir().join("MEMORY.md")
  }

  pub fn skills_index(&self) -> PathBuf {
    self.skills_dir().join("SKILLS.md")
  }

  pub fn tmp_gitignore(&self) -> PathBuf {
    self.tmp_dir().join(".gitignore")
  }
}

fn find_ancestor(start: &Path, marker: &str) -> Option<PathBuf> {
  start
    .ancestors()
    .find(|dir| dir.join(marker).is_dir())
    .map(Path::to_path_buf)
}
