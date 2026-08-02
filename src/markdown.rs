//! Just enough Markdown parsing to follow links between memories.

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

#[cfg(test)]
mod tests {
  use super::{strip_comments, wiki_links};

  #[test]
  fn reads_wiki_links() {
    assert_eq!(
      wiki_links("see [[db-choice]] and [[ api ]]"),
      ["db-choice", "api"]
    );
  }

  #[test]
  fn survives_unclosed_links() {
    assert!(wiki_links("[[a").is_empty());
  }

  #[test]
  fn drops_commented_out_examples() {
    let text = "keep\n<!-- see [[missing]] -->\ntail";
    assert_eq!(strip_comments(text), "keep\n\ntail");
    assert!(wiki_links(&strip_comments(text)).is_empty());
    assert_eq!(strip_comments("open <!-- forever"), "open ");
  }
}
