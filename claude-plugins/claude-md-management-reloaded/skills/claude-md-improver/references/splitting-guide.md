# Splitting CLAUDE.md into Path-Scoped Rules

The headline capability the original CLAUDE.md-only tooling lacked: when a
`CLAUDE.md` grows large or mixes whole-project facts with section-specific
detail, move the section-specific parts into `.claude/rules/*.md` so they load
only when relevant.

## When to split

Split when **any** of these hold:

- The file exceeds ~200 lines.
- A section only applies to part of the codebase (one language, one directory,
  one layer) yet loads every session.
- The file mixes unrelated topics (testing + deploy + frontend styling) that
  different contributors edit independently.
- Two contributors keep colliding on the same monolithic file.

Do **not** split a fact that is genuinely needed in every session (build command,
top-level architecture, repo-wide conventions) — those stay in `CLAUDE.md`.

## What goes where

| Content                                            | Destination                              |
| -------------------------------------------------- | ---------------------------------------- |
| Repo-wide facts needed every session               | Stay in `CLAUDE.md`                      |
| Rules for one file type / directory / layer        | `.claude/rules/<topic>.md` with `paths:` |
| Project-wide but bulky topic (e.g. commit style)   | `.claude/rules/<topic>.md`, no `paths:`  |
| A multi-step procedure rather than a standing fact | A skill (out of scope for memory)        |

## Procedure

1. **Identify a cohesive section** — usually a single `##` heading whose content
   is specific to one area (e.g. "Frontend", "API handlers", "Database").
2. **Derive the glob** — map the section to the files it governs. "Frontend"
   editing `.tsx` under `src/components/` → `src/components/**/*.tsx`. Verify the
   glob matches real files before committing to it.
3. **Create the rule** — write `.claude/rules/<topic>.md`. Add `paths:` if the
   section is path-specific; omit it only if the content is truly project-wide.
4. **Remove the migrated section from `CLAUDE.md`** in the same change. Leave at
   most a one-line pointer if discoverability matters; never keep the full text
   in both places.
5. **Re-check the size budget** — repeat until `CLAUDE.md` is back under ~200
   lines and holds only repo-wide essentials.

## Worked example

Before — an oversized `./CLAUDE.md` (excerpt):

```markdown
## Frontend
- Components live in `src/components/`, one folder per component.
- Use CSS modules, never inline styles.
- All interactive elements must have an aria-label.

## API
- Handlers live in `src/api/handlers/`.
- Every endpoint validates input with the shared `zod` schemas.
- Return the standard error envelope from `src/api/errors.ts`.
```

After — `CLAUDE.md` drops both sections, and two path-scoped rules appear:

`.claude/rules/frontend.md`

```markdown
---
paths:
  - "src/components/**/*.tsx"
---

# Frontend
- Components live in `src/components/`, one folder per component.
- Use CSS modules, never inline styles.
- All interactive elements must have an aria-label.
```

`.claude/rules/api.md`

```markdown
---
paths:
  - "src/api/**/*.ts"
---

# API
- Handlers live in `src/api/handlers/`.
- Every endpoint validates input with the shared `zod` schemas.
- Return the standard error envelope from `src/api/errors.ts`.
```

Result: the frontend rules load only when Claude touches `.tsx` components, the
API rules only when it touches `src/api/`, and `CLAUDE.md` shrinks to the facts
every session actually needs.
