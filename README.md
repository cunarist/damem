# damem

damem tells agents to manage your project memory and skills files without branding them.

This tool aims to be platform-agnostic and agent-agnostic.

## Quickstart

Add below to your `AGENTS.md`

```markdown
Run [`damem recall`](https://github.com/cunarist/damem/releases) in the terminal before starting any session.
```

## What it does

`.agents/` holds plain Markdown that any coding agent can read: what the project
knows, what it can do, and where to put scratch files. damem creates that
directory, tells agents how to keep it, and reports when it drifts. It never
owns the files. Delete damem and every file still works.

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

## Install

Download the binary for your platform from
[Releases](https://github.com/cunarist/damem/releases) and put it on your `PATH`.

```sh
# Linux and macOS
curl -fsSL https://github.com/cunarist/damem/releases/latest/download/damem-aarch64-apple-darwin.tar.gz \
  | tar xz --strip-components=1 -C /usr/local/bin damem-aarch64-apple-darwin/damem
```

Or build it yourself with `cargo install --git https://github.com/cunarist/damem`.

## Use

```sh
damem init      # create .agents/{memory,skills,tmp}
damem recall    # print how to manage .agents/, and what every file describes
damem doctor    # report anything inconsistent
```

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
✗ .agents/memory/db-choice.md: no `description` in its frontmatter
✗ .agents/memory/db-choice.md: links to `[[api-style]]`, which is not a memory here
✗ .agents/skills/lint/SKILL.md: missing; every skill directory needs one
✗ .agents/tmp/.gitignore: missing; run `damem init`
```

## Notes

The files are yours. damem writes only during `init`, and even then it never
overwrites a file that already exists. Everything after that is written by the
agent, in Markdown, reviewable in a normal diff.

Skill scripts are Python 3.12 or newer so they run on any operating system.
That is a convention `recall` states, not something damem enforces.

damem has no dependencies, and will not get any. Reading is sequential and stays
out of the way: 500 memories take about 110 ms, 2000 take about 300 ms.

`recall` is not magic. It prints instructions and the two index files. The value
is that every agent working in the repository gets the same instructions and the
same context, instead of each one keeping its own private store.
