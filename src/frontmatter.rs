//! The `---` block at the top of a memory file or a `SKILL.md`.
//!
//! Flat `key: value` lines only. Anything nested belongs in the body.

/// Fields of one frontmatter block, in the order they were written.
#[derive(Debug, Default)]
pub struct Frontmatter {
  fields: Vec<(String, String)>,
}

impl Frontmatter {
  pub fn get(&self, key: &str) -> Option<&str> {
    self
      .fields
      .iter()
      .find(|(name, _)| name == key)
      .map(|(_, value)| value.as_str())
  }
}

/// Reads the leading `---` block. Returns `None` when the file has none.
pub fn parse(text: &str) -> Option<Frontmatter> {
  // Windows editors put a byte order mark before the opening `---`.
  let text = text.strip_prefix('\u{feff}').unwrap_or(text);
  let mut lines = text.lines();
  if lines.next()?.trim_end() != "---" {
    return None;
  }

  let mut fields = Vec::new();
  for line in lines {
    let trimmed = line.trim();
    if trimmed == "---" {
      return Some(Frontmatter { fields });
    }
    if trimmed.is_empty() || trimmed.starts_with('#') {
      continue;
    }
    let Some((key, value)) = trimmed.split_once(':') else {
      continue;
    };
    let value = value.trim().trim_matches(['"', '\'']).trim();
    if !value.is_empty() {
      fields.push((key.trim().to_owned(), value.to_owned()));
    }
  }
  // No closing `---`, so this was never a frontmatter block.
  None
}

#[cfg(test)]
mod tests {
  #![allow(clippy::expect_used)]

  use super::parse;

  #[test]
  fn reads_fields() {
    let text = "---\nname: lint\ndescription: Run ruff\n---\n\nBody.\n";
    let front = parse(text).expect("frontmatter");
    assert_eq!(front.get("name"), Some("lint"));
    assert_eq!(front.get("description"), Some("Run ruff"));
    assert_eq!(front.get("missing"), None);
  }

  #[test]
  fn strips_quotes_and_keeps_inner_colons() {
    let front = parse("---\ndescription: \"Use 5432: the default\"\n---\n").expect("frontmatter");
    assert_eq!(front.get("description"), Some("Use 5432: the default"));
  }

  #[test]
  fn ignores_a_byte_order_mark() {
    let front = parse("\u{feff}---\ndescription: written on Windows\n---\n").expect("frontmatter");
    assert_eq!(front.get("description"), Some("written on Windows"));
  }

  #[test]
  fn rejects_files_without_a_block() {
    assert!(parse("# Title\n\nText.\n").is_none());
    assert!(parse("---\ndescription: unterminated\n").is_none());
  }
}
