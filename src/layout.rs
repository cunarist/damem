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

  pub fn scratch_dir(&self) -> PathBuf {
    self.agents_dir().join("scratch")
  }

  pub fn scratch_gitignore(&self) -> PathBuf {
    self.scratch_dir().join(".gitignore")
  }
}

fn find_ancestor(start: &Path, marker: &str) -> Option<PathBuf> {
  start
    .ancestors()
    .find(|dir| dir.join(marker).is_dir())
    .map(Path::to_path_buf)
}
