# damem

`damem` tells agents to manage your project memory and skills files without branding them.

This tiny CLI tool aims to be platform-agnostic and agent-agnostic.

## Quickstart

Add below to your `AGENTS.md`

```markdown
Run [`damem recall`](https://github.com/cunarist/damem/releases) in the terminal before starting any session. Run `damem --help` for details.
```

## What it does

`.agents/` holds plain Markdown that any coding agent can read: what the project
knows, what it can do, and where to put scratch files. `damem` tells agents how
to keep that directory, and reports when it drifts. It never owns the files, and
never writes one. Delete `damem` and everything there still works.

```
.agents/
  memory/
    db-choice.md   one fact per file
  skills/
    lint/
      SKILL.md     when to use the skill and how
      lint.py      helper files live with the skill
  tmp/
    .gitignore     scratch space, already ignored
```

There is no index to keep in sync. Every file describes itself, and `recall`
collects those descriptions:

```markdown
---
description: Postgres over SQLite, for concurrent writes
---

Early load tests deadlocked SQLite at eight writers. Postgres held.
```

## Use

```sh
damem recall    # print how to manage .agents/, and what every file describes
damem doctor    # report anything inconsistent
```

Two commands, both read-only. There is nothing to set up: `recall` tells the
agent to create `.agents/` and everything under it as the work calls for it.

`recall` ends with the current listing, so an agent knows what exists before it
opens anything:

```
## Memory — `.agents/memory/`

- `db-choice.md` — Postgres over SQLite, for concurrent writes
- `api-style.md` — Errors are typed, never strings

## Skills — `.agents/skills/`

- `lint/` — Run ruff and fix what it reports, before every commit
```

`doctor` reports what an agent can fix without help:

```
  ✗  .agents/memory/db-choice.md
     no `description` in its frontmatter

  ✗  .agents/skills/lint/SKILL.md
     missing; every skill directory needs one

  →  CLAUDE.md
     missing; Claude Code does not read AGENTS.md. One line is enough: `@AGENTS.md`

  2 problems, 1 suggestion
```

Most agents read `AGENTS.md` directly. Claude Code and Gemini CLI look for their
own file first, so `doctor` points that out when either is installed here. A
suggestion, marked `→`, never fails the check.

## Notes

The files are yours. `damem` never writes anything, anywhere. Every file under
`.agents/` was written by an agent, in Markdown, reviewable in a normal diff.

`recall` tells agents that `.agents/` is theirs to edit and that everything
outside it needs your approval, so a session that learns something writes it
down instead of rewriting your code.

Skill scripts are Python 3.12 or newer so they run on any operating system.
That is a convention `recall` states, not something `damem` enforces.

Output is colored for a terminal and plain everywhere else, so
`damem recall > context.md` and piping into an agent both give clean Markdown.

Reading is sequential and stays out of the way: 500 memories take about 110 ms,
2000 take about 300 ms.

`recall` is not magic. It prints instructions and a listing. The value is that
every agent working in the repository gets the same instructions and the same
context, instead of each one keeping its own private store.
