# Routing Learnings by Scope

Where a captured learning belongs, and what never belongs in memory at all. This
is the decision the original CLAUDE.md-only command could not make.

## Decision tree

1. **Is it a multi-step procedure / checklist rather than a standing fact?**
   → Not memory. Suggest a skill.
2. **Is it personal preference that applies across all your projects?**
   → `~/.claude/CLAUDE.md` or a `~/.claude/rules/` file. Requires explicit user
   approval — it changes every project on the machine.
3. **Is it personal and project-specific, not for the team?**
   → `./CLAUDE.local.md` (gitignored).
4. **Does it only apply to some files (one language, directory, or layer)?**
   → `.claude/rules/<topic>.md` with a `paths:` glob.
5. **Otherwise — a project-wide fact needed often?**
   → `./CLAUDE.md` (or `./.claude/CLAUDE.md`).

## Quick reference

| Learning                                             | Destination                             |
| ---------------------------------------------------- | --------------------------------------- |
| Repo-wide build/test command, top-level convention   | `./CLAUDE.md`                           |
| Rule that only matters for `.tsx` / `src/api` / etc. | `.claude/rules/<topic>.md` + `paths:`   |
| Bulky project-wide topic best kept out of CLAUDE.md  | `.claude/rules/<topic>.md`, no `paths:` |
| Your sandbox URL, your preferred test data           | `./CLAUDE.local.md`                     |
| "I always prefer X" across every project             | `~/.claude/` (with approval)            |
| A repeatable multi-step workflow                     | A skill, not memory                     |

## Choosing the `paths:` glob

When routing to a path-scoped rule, derive the glob from the files the learning
governs, then verify it matches something real:

| Learning is about... | Glob                      |
| -------------------- | ------------------------- |
| React components     | `src/components/**/*.tsx` |
| API handlers         | `src/api/**/*.ts`         |
| Anything TypeScript  | `**/*.{ts,tsx}`           |
| Tests                | `tests/**/*.ts`           |

A glob that matches nothing means the rule will never load; a glob that matches
everything (`**/*`) should usually just be an unconditional rule or a CLAUDE.md
line instead.

## What NOT to capture

- **Obvious-from-code facts** — "the `UserService` handles users." The name says it.
- **Generic best practice** — "write tests", "use clear names." Not project-specific.
- **One-off fixes** — "fixed the bug in commit abc123." Will not recur.
- **Verbose explanations** — compress to the actionable core. Prefer
  "Auth: JWT HS256, `Authorization: Bearer <token>`" over a paragraph on JWTs.
- **Duplicates** — if it already lives in another memory file, do not restate it;
  if it belongs elsewhere, move it rather than copy it.
