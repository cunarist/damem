# damem

damem tells agents to manage your project memory and skills without branding them.

## What it does

`.agents/` holds plain Markdown that any coding agent can read: what the project
knows, what it can do, and where to put scratch files. damem creates that
directory, tells agents how to keep it, and reports when it drifts. It never
owns the files. Delete damem and every file still works.

```
.agents/
  memory/
    MEMORY.md      index, one line per memory
    db-choice.md   one fact per file
  skills/
    SKILLS.md      index, one line per skill
    lint/
      SKILL.md     when to use it and how
  tmp/
    .gitignore     scratch space, already ignored
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
damem recall    # print how to manage .agents/, and what is in it now
damem doctor    # report anything inconsistent
```

Then add one line to `AGENTS.md` so agents pick it up on their own:

```markdown
Run `damem recall` before you start working, and `damem doctor` before you finish.
```

`doctor` reports what an agent can fix without help:

```
✗ .agents/memory/MEMORY.md: links to `api-style.md`, which does not exist
✗ .agents/memory/db-choice.md: not listed in .agents/memory/MEMORY.md
✗ .agents/skills/lint: missing SKILL.md
✗ .agents/tmp/.gitignore: missing; run `damem init`
```

## Notes

The files are yours. damem writes only during `init`, and even then it never
overwrites a file that already exists. Everything after that is written by the
agent, in Markdown, reviewable in a normal diff.

Skill scripts are Python 3.12 or newer so they run on any operating system.
That is a convention `recall` states, not something damem enforces.

`recall` is not magic. It prints instructions and the two index files. The value
is that every agent working in the repository gets the same instructions and the
same context, instead of each one keeping its own private store.
