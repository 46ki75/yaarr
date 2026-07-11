# claude-md-management-reloaded

A rebuild of Anthropic's
[`claude-md-management`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/claude-md-management)
plugin that understands the whole modern memory surface — not just a monolithic
`CLAUDE.md`, but also path-scoped [`.claude/rules/`](https://code.claude.com/docs/en/memory#organize-rules-with-claude%2Frules%2F)
files, `CLAUDE.local.md`, and user-scope memory. It also adds a hook, a component
type the original did not use.

## What's new vs. the original

| Area               | Original                         | Reloaded                                                        |
| ------------------ | -------------------------------- | --------------------------------------------------------------- |
| Memory surface     | `CLAUDE.md` only                 | `CLAUDE.md` + `.claude/rules/` + `CLAUDE.local.md` + user scope |
| Bloat handling     | Flags a long file                | Splits it into path-scoped rules with `paths:` globs            |
| Session capture    | `/revise-claude-md` command      | A skill that **routes** each learning to the right scope        |
| Scaffolding        | Monolithic `CLAUDE.md` (`/init`) | Hybrid lean `CLAUDE.md` + path-scoped rules (`init-claude-md`)  |
| Components         | 1 skill + 1 command              | 3 skills + 1 hook                                               |
| Oversize detection | Manual audit only                | A `PostToolUse` hook that warns automatically on edit           |

## Components

- **`init-claude-md`** (skill) — scaffold a repo's memory from scratch as a
  HYBRID layout: a lean root `CLAUDE.md` (repo-wide essentials) plus path-scoped
  `.claude/rules/*.md` for area-specific conventions. The rules-aware analog of
  Claude Code's built-in `/init` — it analyzes build/test/lint commands and
  big-picture architecture, decides what stays in `CLAUDE.md` versus what becomes
  a rule, verifies each `paths:` glob, and writes only with your approval. If
  memory already exists it suggests improvements instead of clobbering. Invoke
  with `/claude-md-management-reloaded:init-claude-md`.
- **`claude-md-improver`** (skill) — audit and improve memory across every scope.
  Scores each file, then proposes targeted fixes, splits an oversized `CLAUDE.md`
  into `.claude/rules/*.md`, and fixes rule scope/globs. Invoke with
  `/claude-md-management-reloaded:claude-md-improver`, or let Claude run it when
  you ask to audit or reorganize project memory.
- **`revise-claude-md`** (skill) — capture this session's learnings and route each
  to a project `CLAUDE.md`, a path-scoped rule, `CLAUDE.local.md`, or user scope.
  Successor to the original `/revise-claude-md` command (commands are now skills).
  Invoke with `/claude-md-management-reloaded:revise-claude-md`.
- **Memory-size hook** (`PostToolUse`) — after any `Edit`/`Write` to a
  `CLAUDE.md` or `CLAUDE.local.md`, warns when the file passes the recommended
  ~200-line budget and nudges toward splitting into rules. Threshold overridable
  with `CLAUDE_MD_MAX_LINES`.

## Install

```bash
/plugin marketplace add 46ki75/claude-plugins
/plugin install claude-md-management-reloaded@46ki75-plugins
```

## Notes

- The hook only ever **warns**; it never edits files. Splitting is always done by
  a skill, with your approval.
- User-scope memory (`~/.claude/CLAUDE.md`, `~/.claude/rules/`) is touched only
  when you explicitly ask — never as a side effect of project work.
- This plugin manages author-written memory. *Auto memory* (the notes Claude
  writes itself under `~/.claude/projects/.../memory/`) is a separate system and
  is left alone.
