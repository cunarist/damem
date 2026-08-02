# damem

`damem` tells agents how to manage your project memory and skills files in the
code repository without branding them.

Memory of your agents such as Claude Code or Codex lives on one machine in one
tool's format, so a new computer, a teammate's clone, or a different agent
starts from nothing. `.agents/` is committed with the code, and whoever shows up
next reads it.

This tiny CLI tool aims to be platform-agnostic and agent-agnostic.

## Quickstart

Add the line below to your `AGENTS.md`

```markdown
Run [`damem recall`](https://github.com/cunarist/damem/releases) in the terminal
before starting any session. Run `damem --help` for details.
```

## What it does

`.agents/` holds plain Markdown that any coding agent can read: what the project
knows, what it can do, and where to put scratch files.

`damem` prints the rules for keeping it, and reports when the directory drifts.
It never owns the files, and never writes one. Delete `damem` and everything
there still works.

```
.agents/
  memory/
    db-choice.md   one fact per file
  skills/
    release-staging/
      SKILL.md     when to use the skill and how
      upload.py    helper files live with the skill
  tmp/
    .gitignore     scratch space, already ignored
```

There is no index to keep in sync. Every file describes itself, and
`damem recall` collects those descriptions, so one command shows the agent
everything the project has.

## Notes

- `damem recall` tells agents that `.agents/` is theirs to edit and that
  everything outside it needs your approval, so a session that learns something
  writes it down instead of rewriting your code.
- Skill scripts are Python 3.12 or newer so they run on any operating system.
  That is a convention `damem recall` states, not something `damem` enforces.
- Output is colored for a terminal and plain everywhere else, so
  `damem recall > context.md` and piping into an agent both give clean Markdown.
- Reading is sequential and stays out of the way: 500 memories take about 110
  ms, 2000 take about 300 ms.
