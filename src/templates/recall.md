# Project memory and skills

This project keeps durable context in `.agents/`. The files there are plain
Markdown owned by the project, not by any one agent. Read them and edit them
directly, the same way you edit source files.

There is no index to maintain. Every file describes itself in its frontmatter,
and `damem recall` collects those descriptions into the listing below.

## `.agents/memory/` — what the project knows

One fact per file, named after the fact:

    ---
    description: Postgres over SQLite, for concurrent writes
    ---

    Early load tests deadlocked SQLite at eight writers. Postgres held.
    Migrations live in `db/migrations`.

The description is one line, and it is what other agents see before they open
the file. Make it say the fact, not the topic: "Postgres over SQLite, for
concurrent writes", never "notes about the database".

Write a memory when something is durable and not derivable from the repository:
a decision and the reason behind it, a constraint, a preference the user stated,
a fact about an external system. Do not write down what the code, the tests, or
the git history already say.

Before adding a file, look through the listing for one that already covers the
topic and update that instead. Delete memories that turn out to be wrong. Link
related memories with `[[db-choice]]`, using the file name without `.md`.

## `.agents/skills/` — what the project can do

One skill per subdirectory, each with a `SKILL.md`:

    ---
    name: lint
    description: Run ruff and fix what it reports, before every commit
    ---

    Run `python lint.py --fix`. It exits non-zero when something is left.

The description says when to reach for the skill; the body says how to run it.
Keep helper files in the same directory as the skill that uses them.

Write skill scripts in Python 3.12 or newer so they run on any operating system.
Annotate every function with modern type hints (`list[str]`, `str | None`,
`type Row = dict[str, int]`), and comment anything the code does not make
obvious.

## `.agents/tmp/` — scratch space

Anything not worth committing: intermediate output, downloaded data, drafts.
It is already git-ignored. Never keep memories or skills here.

## Before you finish

Run `damem doctor` and fix whatever it reports.
