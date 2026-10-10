---
description: doctor no longer suggests CLAUDE.md or GEMINI.md; do not bring them back
---

`damem doctor` used to suggest a per-tool file that imports `AGENTS.md` when
that tool was installed. CLAUDE.md went in 0.3.0 because Claude Code reads
`AGENTS.md` on its own. The user then had GEMINI.md removed "completely" in
0.4.0, along with the whole suggestion mechanism (`agents.rs`, the blue
`SUGGESTION` style, the suggestion count in doctor's summary).

damem stays agent-agnostic: `AGENTS.md` is the one entry point. Ask the user
before adding any tool-specific file or check again.

See [[releases]] for how these removals were versioned.
