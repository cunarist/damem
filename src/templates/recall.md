# Project memory and skills

This project keeps durable context in `.agents/`. The files there are plain
Markdown owned by the project, not by any one agent. Read them and edit them
directly, the same way you edit source files.

## `.agents/memory/` — what the project knows

One fact per file. `MEMORY.md` is the index, and every memory file gets exactly
one line in it:

    - [Postgres over SQLite](db-choice.md) — chosen for concurrent writes

Write a memory when something is durable and not derivable from the repository:
a decision and the reason behind it, a constraint, a preference the user stated,
a fact about an external system. Do not write down what the code, the tests, or
the git history already say.

Before adding a file, look for one that already covers the topic and update that
instead. Delete memories that turn out to be wrong. Link related memories with
`[[db-choice]]`, using the file name without `.md`.

## `.agents/skills/` — what the project can do

One skill per subdirectory, each with a `SKILL.md` that says when to use the
skill and how to run it. Keep helper files in the same directory as the skill
that uses them. `SKILLS.md` indexes the skills the way `MEMORY.md` indexes
memories.

Write skill scripts in Python 3.12 or newer so they run on any operating system.
Annotate every function with modern type hints (`list[str]`, `str | None`,
`type Row = dict[str, int]`), and comment anything the code does not make
obvious.

## `.agents/tmp/` — scratch space

Anything not worth committing: intermediate output, downloaded data, drafts.
It is already git-ignored. Never keep memories or skills here.

## Before you finish

Run `damem doctor` and fix whatever it reports.
