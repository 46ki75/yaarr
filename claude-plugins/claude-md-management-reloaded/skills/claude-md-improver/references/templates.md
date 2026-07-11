# Memory File Templates

Canonical shapes for the files this skill writes. Use only the sections a project
actually needs; every line costs context.

## Principles

- Concise, dense, human-readable — one line per concept where possible.
- Actionable: commands are copy-paste ready; paths are real.
- Project-specific: document what is true of *this* repo, not generic advice.
- Right scope: project-wide → `CLAUDE.md`; path-specific → a rule.

## CLAUDE.md (minimal)

```markdown
# <Project Name>

<One-line description>

## Commands

| Command     | Description |
| ----------- | ----------- |
| `<command>` | <what>      |

## Architecture

<short tree or two-line map>

## Gotchas

- <non-obvious thing that bites people>
```

## CLAUDE.md (comprehensive)

```markdown
# <Project Name>

<One-line description>

## Commands

| Command     | Description |
| ----------- | ----------- |
| `<command>` | <what>      |

## Architecture

<structure with one-line purpose per key directory>

## Key Files

- `<path>` — <purpose>

## Code Style

- <convention>

## Environment

- `<VAR>` — <purpose>

## Gotchas

- <quirk or ordering dependency>
```

Keep this under ~200 lines. When it grows past that, move section-specific parts
into rules — see `splitting-guide.md`.

## Rule: path-scoped

For guidance that only applies to some files. Loads only when Claude touches a
matching file.

```markdown
---
paths:
  - "src/api/**/*.ts"
---

# <Topic>

- <rule that applies to matching files>
- <another rule>
```

## Rule: unconditional

For a project-wide topic that is bulky enough to keep out of `CLAUDE.md` but
should still always load. Omit `paths:`.

```markdown
# <Topic>

- <project-wide rule>
- <another rule>
```

## CLAUDE.local.md

Personal, per-project, gitignored. Same format as `CLAUDE.md`; use for things the
team should not inherit.

```markdown
# Local notes

- Sandbox URL: <url>
- Preferred test data: <...>
```

## Monorepo layout

A root `CLAUDE.md` plus rules and/or nested `CLAUDE.md` per package.

```markdown
# <Monorepo Name>

## Packages

| Package  | Path     | Purpose   |
| -------- | -------- | --------- |
| `<name>` | `<path>` | <purpose> |

## Commands

| Command     | Description |
| ----------- | ----------- |
| `<command>` | <what>      |
```

Per-package specifics go in that package's nested `CLAUDE.md` (loads on demand
when Claude works in the package) or in path-scoped rules with the package path
in the glob.
