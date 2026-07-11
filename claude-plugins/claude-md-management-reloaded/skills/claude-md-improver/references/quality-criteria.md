# Memory Quality Criteria

Score each memory file out of 100. The first five criteria judge content; the
sixth — **Scope & rules** — is what makes this skill memory-architecture aware
rather than CLAUDE.md-only.

## Rubric (100 points)

| Criterion          | Pts | What full marks looks like                                          |
| ------------------ | --- | ------------------------------------------------------------------- |
| Commands/workflows | 20  | Build, test, lint, deploy commands present with just enough context |
| Architecture       | 20  | Key dirs, module relationships, and entry points are clear          |
| Non-obvious        | 15  | Gotchas, quirks, and "why we do it this way" are captured           |
| Conciseness        | 15  | Dense and valuable; no filler, no restating the code                |
| Currency           | 15  | Commands run, paths resolve, versions match the real codebase       |
| Scope & rules      | 15  | Right content in the right file; rule globs correct and minimal     |

## Grades

| Grade | Range  | Meaning                          |
| ----- | ------ | -------------------------------- |
| A     | 90–100 | Comprehensive, current, scoped   |
| B     | 70–89  | Good, minor gaps                 |
| C     | 50–69  | Basic; missing sections or scope |
| D     | 30–49  | Sparse or partly outdated        |
| F     | 0–29   | Missing or severely outdated     |

## Scoring the "Scope & rules" criterion

This criterion is graded differently per file type.

For a **CLAUDE.md** (15 pts):

- **−** if it exceeds ~200 lines (it should be split into rules).
- **−** if it contains path-specific procedure that should be a path-scoped rule.
- **−** if it duplicates content that already lives in a rule.
- **−** if personal/uncommitted notes sit in a committed file (move to
  `CLAUDE.local.md`).

For a **rule file** (`.claude/rules/*.md`) (15 pts):

- **Full** if it covers exactly one topic and either has a correct `paths:` glob
  (for path-specific guidance) or is genuinely project-wide (no `paths:`).
- **−** if it is unconditional but only relevant to some files (add `paths:`).
- **−** if its `paths:` globs match nothing real, or overlap heavily with another
  rule (consolidate).
- **−** if it restates something already in `CLAUDE.md`.

## Assessment process

1. Read the file completely.
2. Cross-reference the codebase: run (or mentally run) documented commands, check
   referenced paths exist, and verify each `paths:` glob matches real files.
3. Score the six criteria; total and assign a grade.
4. List concrete issues, then concrete fixes (content, split, or restructure).

## Red flags

- Commands that would fail; references to deleted files.
- A CLAUDE.md well over 200 lines.
- The same instruction in two places (CLAUDE.md + rule, or two CLAUDE.md files).
- `paths:` globs that are empty, too broad (`**/*`), or copied without edits.
- Generic advice not specific to this project; never-completed "TODO" lines.
