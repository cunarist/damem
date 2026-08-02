use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

/// Every way damem can fail.
#[derive(Debug)]
pub enum Error {
  /// A filesystem operation failed, with the path it was working on.
  Io { path: PathBuf, source: io::Error },
  /// The current directory could not be read.
  NoCurrentDir { source: io::Error },
  /// `.agents/` does not exist anywhere up the directory tree.
  NotInitialized { root: PathBuf },
  /// The command line did not parse.
  Usage { message: String },
}

impl Error {
  pub fn io(path: impl Into<PathBuf>, source: io::Error) -> Self {
    Self::Io {
      path: path.into(),
      source,
    }
  }

  pub fn usage(message: impl Into<String>) -> Self {
    Self::Usage {
      message: message.into(),
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Io { path, source } => write!(f, "{}: {source}", path.display()),
      Self::NoCurrentDir { source } => write!(f, "cannot read the current directory: {source}"),
      Self::NotInitialized { root } => write!(
        f,
        "no `.agents` directory in {} or its parents; run `damem init` first",
        root.display()
      ),
      Self::Usage { message } => write!(f, "{message}"),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::Io { source, .. } | Self::NoCurrentDir { source } => Some(source),
      Self::NotInitialized { .. } | Self::Usage { .. } => None,
    }
  }
}

/// `std::fs` wrappers that keep the path in the error.
pub mod fs {
  use super::{Error, Result};
  use std::fs;
  use std::path::Path;

  pub fn create_dir_all(path: &Path) -> Result<()> {
    fs::create_dir_all(path).map_err(|source| Error::io(path, source))
  }

  pub fn read_to_string(path: &Path) -> Result<String> {
    let text = fs::read_to_string(path).map_err(|source| Error::io(path, source))?;
    Ok(without_bom(text))
  }

  /// Windows editors put a byte order mark at the start of a UTF-8 file. Every
  /// line-by-line check here would trip over it, so it goes early.
  fn without_bom(text: String) -> String {
    match text.strip_prefix('\u{feff}') {
      Some(rest) => rest.to_owned(),
      None => text,
    }
  }

  /// Writes `contents` only when `path` is absent. Returns whether it wrote.
  pub fn write_if_absent(path: &Path, contents: &str) -> Result<bool> {
    if path.exists() {
      return Ok(false);
    }
    fs::write(path, contents).map_err(|source| Error::io(path, source))?;
    Ok(true)
  }

  /// Reads at most `limit` bytes from the start of `path`.
  ///
  /// Frontmatter sits at the top of the file, so there is no reason to read the
  /// rest of it. Invalid UTF-8, including a multi-byte character cut off at the
  /// limit, becomes a replacement character.
  pub fn read_head(path: &Path, limit: usize) -> Result<String> {
    use std::io::Read;

    let file = fs::File::open(path).map_err(|source| Error::io(path, source))?;
    let mut buffer = Vec::new();
    file
      .take(u64::try_from(limit).unwrap_or(u64::MAX))
      .read_to_end(&mut buffer)
      .map_err(|source| Error::io(path, source))?;
    Ok(without_bom(String::from_utf8_lossy(&buffer).into_owned()))
  }

  /// Entries of `path`, sorted by file name. An absent directory reads as empty.
  pub fn read_dir_sorted(path: &Path) -> Result<Vec<fs::DirEntry>> {
    let reader = match fs::read_dir(path) {
      Ok(reader) => reader,
      Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
      Err(source) => return Err(Error::io(path, source)),
    };
    let mut entries = Vec::new();
    for entry in reader {
      entries.push(entry.map_err(|source| Error::io(path, source))?);
    }
    entries.sort_by_key(std::fs::DirEntry::file_name);
    Ok(entries)
  }
}

/// Path relative to `root`, for messages. Falls back to the full path.
pub fn display_relative(path: &Path, root: &Path) -> String {
  path
    .strip_prefix(root)
    .unwrap_or(path)
    .display()
    .to_string()
    .replace('\\', "/")
}
