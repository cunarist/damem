# Project memory and skills

This project keeps durable context in `.agents/`. The files there are plain
Markdown owned by the project, not by any one agent. Read them and edit them
directly, the same way you edit source files. Create the directories and files
below yourself when they are missing; damem only reads.

There is no index to maintain. Every file describes itself in its frontmatter,
and `damem recall` collects those descriptions into the listing below.

## What you may change on your own

Inside `.agents/`, create, edit, and delete files as the work calls for it.
Outside it, nothing. Source code, configuration, `AGENTS.md`, the rest of the
repository: ask the user before you touch any of it, even when a change would
follow from what you just learned. damem itself never writes outside `.agents/`.

## `.agents/memory/` — what the project knows

One fact per file, named after the fact:

```markdown
---
description: Postgres over SQLite, for concurrent writes
---

Early load tests deadlocked SQLite at eight writers. Postgres held.
Migrations live in `db/migrations`.
```

The description is one line, and it is what other agents see before they open
the file. Make it say the fact, not the topic: "Postgres over SQLite, for
concurrent writes", never "notes about the database".

Write a memory when something is durable and not derivable from the repository:
a decision and the reason behind it, a constraint, a preference the user stated,
a fact about an external system. Do not write down what the code, the tests, or
the git history already say.

Three moments deserve a memory even when you are not sure:

- The user is annoyed, repeats a correction, or says a thing matters. Whatever
  you just got wrong will be got wrong again next session unless it is written
  down. Record the rule, not the apology.
- The user states a preference in passing. It is cheaper to write it now than to
  rediscover it from their reaction later.
- You worked out how something fits together by reading a lot of files: the
  layout of a subsystem, what calls what, why a module is split the way it is.
  One paragraph here saves the next session that whole search.

Keep the directory flat. A memory in a subdirectory is not listed, so nobody
reads it. Before adding a file, look through the listing for one that already
covers the topic and update that instead. Delete memories that turn out to be
wrong. Link related memories with `[[db-choice]]`, using the file name without
`.md`.

## `.agents/skills/` — what the project can do

One skill per subdirectory, each with a `SKILL.md`:

```markdown
---
name: lint
description: Run ruff and fix what it reports, before every commit
---

Run `python lint.py --fix`. It exits non-zero when something is left.
```

The description says when to reach for the skill; the body says how to run it.
Keep helper files in the same directory as the skill that uses them.

Write skill scripts in Python 3.12 or newer so they run on any operating system.
Annotate every function with modern type hints (`list[str]`, `str | None`,
`type Row = dict[str, int]`), and comment anything the code does not make
obvious.

## Keep every file under 8 KB

That is the budget for one memory and for one `SKILL.md`. A file near it is
usually restating itself: cut the second telling of the same point, the
background nobody needs, the example that repeats the previous example.

If it is still too long after cutting, it holds more than one thing. Split it.
A memory becomes two memories, linked with `[[other-memory]]`. A skill keeps
`SKILL.md` short and moves the detail into a sibling file in the same directory,
mentioned by name so it gets read when it is needed.

## `.agents/tmp/` — scratch space

Anything not worth committing: intermediate output, downloaded data, drafts.
Never keep memories or skills here. The directory holds a `.gitignore` that
keeps its contents out of the repository, so create one when it is missing:

```gitignore
*
!.gitignore
```

## Before you finish

Run `damem doctor` and fix whatever it reports.
