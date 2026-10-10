---
description: Module map; recall and doctor both read .agents through entries.rs
---

`main.rs` parses the two subcommands with clap and finds the project root via
`Layout::discover` (`layout.rs`), which also names every path under `.agents/`.

- `recall.rs` prints `templates/recall.md` verbatim (compiled in with
  `include_str!`), then lists memories and skills.
- `doctor.rs` checks scratch, memory, and skills, and exits non-zero on any
  problem. It has no suggestions anymore, see [[no-per-agent-instruction-files]].
- `entries.rs` scans memory files and skill directories and reads only the
  first 8 KB of each for frontmatter, parsed by `frontmatter.rs` (flat
  `key: value` only).
- `markdown.rs` strips HTML comments and extracts double-bracket wiki links for doctor.
- `error.rs` holds the `Error` enum plus `fs` wrappers that keep the path in
  the error; `style.rs` holds every color.

The guidance agents read lives in `src/templates/recall.md`, so changing what
agents are told is a template edit, not a code change.
