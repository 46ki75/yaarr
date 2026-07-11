# Claude Code Memory Architecture

The model behind every recommendation this skill makes. Two author-written
mechanisms carry instructions across sessions: **CLAUDE.md files** and
**`.claude/rules/`**. (A third mechanism, *auto memory*, is written by Claude
itself and is out of scope — do not edit it here.)

## CLAUDE.md files

Markdown files loaded into context at the start of every session. They are
loaded **in full regardless of length**, so size directly costs context.

### Scopes and load order

Listed broadest to most specific; later entries load after (and thus take
priority over) earlier ones.

| Scope   | Location                                   | Committed?     | Use for                             |
| ------- | ------------------------------------------ | -------------- | ----------------------------------- |
| Managed | OS-specific managed-policy path            | No (IT)        | Org-wide policy; cannot be excluded |
| User    | `~/.claude/CLAUDE.md`                      | No             | Personal prefs across all projects  |
| Project | `./CLAUDE.md` **or** `./.claude/CLAUDE.md` | Yes            | Team-shared project context         |
| Local   | `./CLAUDE.local.md`                        | No (gitignore) | Personal per-project notes          |

Claude walks up the directory tree from the working directory, concatenating
every `CLAUDE.md` and `CLAUDE.local.md` it finds (root → cwd order). `CLAUDE.md`
in **subdirectories** is not loaded at launch; it loads on demand when Claude
reads a file in that subdirectory — a natural form of path-scoping.

### Imports

A CLAUDE.md can pull in other files with `@path/to/file` syntax (relative to the
importing file, max depth 4). Imports are expanded into context **at launch**,
so they help *organization* but do **not** reduce context cost. To reduce
context, use path-scoped rules instead.

### Size budget

Target **under 200 lines per CLAUDE.md file**. Longer files consume more context
and measurably reduce instruction adherence. When a file grows past this,
splitting into path-scoped rules is the fix — see `splitting-guide.md`.

## `.claude/rules/`

A directory of topic markdown files, discovered **recursively** (`*.md`,
including subdirectories like `frontend/`, `backend/`). One topic per file with a
descriptive name (`testing.md`, `api-design.md`).

### Unconditional vs path-scoped

The only frontmatter field a rule supports is **`paths:`** (a list of globs).

- **No `paths:`** → loaded unconditionally at launch, same priority as
  `.claude/CLAUDE.md`. Use only for genuinely project-wide instructions.
- **With `paths:`** → loaded only when Claude works with a file matching one of
  the globs. This is the context-saving mechanism: instructions appear exactly
  when relevant and cost nothing otherwise.

```markdown
---
paths:
  - "src/api/**/*.ts"
  - "src/**/*.{ts,tsx}"
---

# API rules
- All endpoints validate input.
- Use the standard error envelope.
```

### Glob reference

| Pattern                | Matches                                 |
| ---------------------- | --------------------------------------- |
| `**/*.ts`              | All TypeScript files, any directory     |
| `src/**/*`             | Everything under `src/`                 |
| `*.md`                 | Markdown at the project root            |
| `src/components/*.tsx` | Components in one directory             |
| `src/**/*.{ts,tsx}`    | Brace expansion for multiple extensions |

### User-level rules

`~/.claude/rules/*.md` apply to every project and load before project rules
(project rules win on conflict). Only touch these when the user explicitly asks
about cross-project personal memory.

## Decision: where does an instruction belong?

| The instruction is...                                 | Put it in                                   |
| ----------------------------------------------------- | ------------------------------------------- |
| A project-wide fact needed every session              | `./CLAUDE.md`                               |
| Specific to one file type or directory                | `.claude/rules/<topic>.md` with `paths:`    |
| A multi-step procedure / checklist rather than a fact | A skill (not memory at all)                 |
| Personal, project-specific, not for the team          | `./CLAUDE.local.md` (gitignored)            |
| Personal preference across all your projects          | `~/.claude/CLAUDE.md` or `~/.claude/rules/` |

If two files give conflicting guidance, Claude may pick one arbitrarily — so
audits should also hunt for contradictions across CLAUDE.md, nested CLAUDE.md,
and rules.
