---
name: init-claude-md
description: >-
  Scaffold a project's persistent memory the way Claude Code's built-in /init
  would, but as a HYBRID layout instead of one monolithic file: a lean root
  CLAUDE.md holding only repo-wide essentials, PLUS path-scoped
  .claude/rules/*.md for area-specific conventions. The rules-aware analog of
  /init. Run manually with /claude-md-management-reloaded:init-claude-md on a
  repo that has little or no memory yet: it analyzes build/test/lint commands
  and big-picture architecture, decides what stays in CLAUDE.md versus what
  becomes a path-scoped rule, derives and verifies paths: globs against real
  files, and writes the files only after you approve. If memory already exists
  it proposes improvements instead of overwriting. For auditing memory that
  already exists, use the companion claude-md-improver skill.
license: MIT
argument-hint: "[subdirectory to scope, or a specific area to document]"
disable-model-invocation: true
metadata:
  author: "Ikuma Yamashita"
  version: "1.0.0"
---

# Initialize CLAUDE.md + rules (hybrid /init)

Bootstrap a repository's persistent memory the way Claude Code's built-in `/init`
does — analyze the codebase, then write the memory a future session needs — but
emit a **hybrid layout** instead of one monolithic file:

- a **lean root `CLAUDE.md`** with only repo-wide essentials (the canonical
  build/test/lint commands and the big-picture architecture), and
- **path-scoped `.claude/rules/*.md`** for area-specific conventions, so each
  area's rules load only when Claude touches that area.

## Why this is a skill

- It **writes new memory files**, a side effect you should trigger deliberately —
  so it is user-only (`disable-model-invocation: true`) and never scaffolds on its
  own. Invoke it with `/claude-md-management-reloaded:init-claude-md`.
- It is the rules-aware analog of `/init`: same analysis, but the result is split
  across the modern memory surface instead of dumped into one file.
- The split and glob mechanics are non-trivial, so this skill reuses the
  `claude-md-improver` references rather than re-deriving them.

## When to invoke

- Manually, on a repo that has **little or no** memory yet.
- Optional argument: a subdirectory to scope the scan to (a monorepo package), or
  a specific area to focus on.
- If the repo already has substantial memory, this skill does **not** clobber it —
  it switches to proposing improvements or a split, and may hand off to
  `claude-md-improver` for a full audit.

## What this produces

```text
CLAUDE.md                      # lean: repo-wide commands + big-picture architecture
.claude/rules/
├── frontend.md                # paths: src/components/**/*.tsx
├── api.md                     # paths: src/api/**/*.ts
└── testing.md                 # paths: tests/**/*
```

## Read first

- [memory model + glob mechanics](../claude-md-improver/references/memory-architecture.md)
- [how to decide and perform a split](../claude-md-improver/references/splitting-guide.md)
- [canonical CLAUDE.md and rule shapes](../claude-md-improver/references/templates.md)

## Ask when the codebase can't answer

`/init` analyzes the code, but some memory-worthy facts aren't in it: which
command is canonical when several exist, where the real architectural boundaries
lie, deploy/release steps, or a gotcha only the maintainer knows. When the repo
doesn't settle such a point and guessing would risk a wrong or invented
instruction, **ask** rather than assume — but keep it light:

- Ask only what you genuinely can't determine from manifests, config, code, or
  the README. Never ask what the repo already answers.
- Batch the open questions into a single round (aim for ≤3) with `AskUserQuestion`,
  each with concrete options and a sensible default first.
- If the user skips, proceed with the default and note the assumption. Never
  block the scaffold on an answer.

## Workflow

### Step 1: Detect existing memory (improve, don't clobber)

Discover every memory and rules source before writing anything:

```bash
find . -path ./node_modules -prune -o \
  \( -name CLAUDE.md -o -name CLAUDE.local.md \) -print 2>/dev/null | sort
ls -1 .claude/rules/*.md .claude/rules/**/*.md 2>/dev/null
ls -1 .cursorrules .cursor/rules/* .github/copilot-instructions.md README.md 2>/dev/null
```

Branch on what you find:

- **`CLAUDE.md` or `.claude/rules/` already exist** → do not overwrite. Read them,
  then propose targeted improvements and/or a split of an oversized `CLAUDE.md`
  into rules — or defer to the `claude-md-improver` skill for a full audit.
- **Cursor rules (`.cursor/rules/`, `.cursorrules`) or Copilot rules
  (`.github/copilot-instructions.md`)** → fold the important parts into the new
  memory; do not copy them wholesale.
- **`README.md`** → mine it for the important parts; never duplicate what is
  trivially discoverable from the tree.
- **Nothing** → scaffold from scratch.

### Step 2: Analyze the repo

This is the literal `/init` "what to add", carried over so behavior matches the
built-in:

```text
1. Commands that will be commonly used, such as how to build, lint, and run
   tests. Include the necessary commands to develop in this codebase, such as how
   to run a single test.
2. High-level code architecture and structure so that future instances can be
   productive more quickly. Focus on the "big picture" architecture that requires
   reading multiple files to understand.
```

Gather these by reading build manifests (`package.json`, `justfile`, `Cargo.toml`,
`pyproject.toml`, `Makefile`), entry points, and the directory layout. Identify
the distinct **areas** (frontend / api / tests / infra / per-package) — these are
the split candidates for Step 3. If several plausible build/test commands exist
and the code doesn't reveal which is canonical, that's an Ask candidate.

### Step 3: Decide the split

Apply the "what goes where" guidance from
[splitting-guide.md](../claude-md-improver/references/splitting-guide.md):

- Repo-wide facts needed every session (top-level architecture, the canonical
  build/test/lint commands, repo-wide conventions) → **stay in the lean root
  `CLAUDE.md`**.
- Conventions specific to one area, language, or layer → a
  **`.claude/rules/<topic>.md`** with a `paths:` glob.
- A bulky project-wide topic (e.g. commit style) → a rule **without** `paths:`.
- A multi-step procedure rather than a standing fact → suggest a skill, not memory.

Aim to keep the root file under the ~200-line budget, pushing each area-specific
block down into a rule. Confirm any non-obvious split boundary with the user when
the code doesn't make it clear.

### Step 4: Derive and verify `paths:` globs

For each rule, map the area to the files it governs, then **verify the glob
matches real files before committing to it**:

```bash
ls src/components/**/*.tsx 2>/dev/null | head   # confirm the glob is non-empty
```

If a candidate glob matches nothing, drop the rule or widen the glob — never ship
a `paths:` that matches no files.

### Step 5: Write the lean `CLAUDE.md`

The file must begin with the exact `/init` prefix — this is the imitation
contract:

```markdown
# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
```

Then a lean body — `## Commands` (copy-paste-ready, including how to run a single
test) and `## Architecture` (the big-picture map) — per the minimal shape in
[templates.md](../claude-md-improver/references/templates.md). Rule files use the
path-scoped shape from the same templates.

### Step 6: Show the tree and diffs, apply only on approval

Present the full proposed file tree, then each new file as a diff with a one-line
"why". Apply with Edit/Write **only after the user approves**. When a split moves
content out of `CLAUDE.md`, never leave it duplicated in both the file and a rule.

## What NOT to include

Carried over from `/init` so the output stays lean:

- Do not repeat yourself, and do not include obvious instructions like "provide
  helpful error messages", "write unit tests for all new utilities", or "never
  commit secrets / API keys".
- Avoid listing every component or file that is easily discovered from the tree.
- Don't include generic development practices.
- Do not invent sections like "Common Development Tasks", "Tips for Development",
  or "Support and Documentation" unless they come from files you actually read.

One hybrid-specific rule: **do not force a split when there is nothing to split.**
A tiny single-area repo may correctly produce only a lean `CLAUDE.md` with zero
rules.

## Example

A repo with a TypeScript frontend in `src/components/` and an API in `src/api/`
scaffolds to a lean root file plus two path-scoped rules:

```text
CLAUDE.md                  # prefix + ## Commands + ## Architecture (big picture)
.claude/rules/
├── frontend.md            # paths: ["src/components/**/*.tsx"] — CSS modules, aria-labels
└── api.md                 # paths: ["src/api/**/*.ts"] — input validation, error envelope
```

The component conventions load only when Claude edits a `.tsx`, the API
conventions only under `src/api/`, and `CLAUDE.md` stays small.

## Output

- A new lean `CLAUDE.md` (with the `/init` prefix) plus one or more
  `.claude/rules/*.md`, each with a verified `paths:` glob — written **only after
  approval**.
- Or, when memory already exists, an improvement/split proposal instead of a fresh
  scaffold.
- A one-line summary: which files were created, and which areas became rules.
