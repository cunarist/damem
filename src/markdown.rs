//! Just enough Markdown parsing to check that an index matches the files.

/// Removes `<!-- ... -->` blocks so examples inside comments are not checked.
pub fn strip_comments(text: &str) -> String {
  let mut out = String::with_capacity(text.len());
  let mut rest = text;
  while let Some(start) = rest.find("<!--") {
    out.push_str(rest.get(..start).unwrap_or_default());
    let after = rest.get(start + 4..).unwrap_or_default();
    match after.find("-->") {
      Some(end) => rest = after.get(end + 3..).unwrap_or_default(),
      None => return out,
    }
  }
  out.push_str(rest);
  out
}

/// Targets of inline links: `[text](target)` yields `target`.
pub fn link_targets(text: &str) -> Vec<String> {
  let mut targets = Vec::new();
  let mut rest = text;
  while let Some(open) = rest.find("](") {
    let after = rest.get(open + 2..).unwrap_or_default();
    let Some(close) = after.find(')') else { break };
    if let Some(target) = after.get(..close) {
      let target = target.split_whitespace().next().unwrap_or_default();
      if !target.is_empty() {
        targets.push(target.to_owned());
      }
    }
    rest = after.get(close + 1..).unwrap_or_default();
  }
  targets
}

/// Names inside `[[wiki links]]`.
pub fn wiki_links(text: &str) -> Vec<String> {
  let mut names = Vec::new();
  let mut rest = text;
  while let Some(open) = rest.find("[[") {
    let after = rest.get(open + 2..).unwrap_or_default();
    let Some(close) = after.find("]]") else { break };
    if let Some(name) = after.get(..close) {
      let name = name.trim();
      if !name.is_empty() && !name.contains('\n') {
        names.push(name.to_owned());
      }
    }
    rest = after.get(close + 2..).unwrap_or_default();
  }
  names
}

/// Whether a link points outside the repository.
pub fn is_external(target: &str) -> bool {
  target.contains("://") || target.starts_with('#') || target.starts_with("mailto:")
}

#[cfg(test)]
mod tests {
  use super::{is_external, link_targets, strip_comments, wiki_links};

  #[test]
  fn reads_link_targets() {
    let text = "- [Postgres](db-choice.md) — why\n- [Docs](https://x.dev)";
    assert_eq!(link_targets(text), ["db-choice.md", "https://x.dev"]);
  }

  #[test]
  fn ignores_link_titles() {
    assert_eq!(link_targets("[a](b.md \"title\")"), ["b.md"]);
  }

  #[test]
  fn survives_unclosed_links() {
    assert!(link_targets("[a](b.md").is_empty());
    assert!(wiki_links("[[a").is_empty());
  }

  #[test]
  fn reads_wiki_links() {
    assert_eq!(
      wiki_links("see [[db-choice]] and [[ api ]]"),
      ["db-choice", "api"]
    );
  }

  #[test]
  fn drops_commented_out_examples() {
    let text = "keep\n<!-- - [x](missing.md) -->\ntail";
    assert_eq!(strip_comments(text), "keep\n\ntail");
    assert_eq!(strip_comments("open <!-- forever"), "open ");
  }

  #[test]
  fn classifies_external_targets() {
    assert!(is_external("https://example.com"));
    assert!(is_external("#section"));
    assert!(!is_external("db-choice.md"));
  }
}
