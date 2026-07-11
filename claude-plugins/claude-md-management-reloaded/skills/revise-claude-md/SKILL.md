---
name: revise-claude-md
description: >
  Capture learnings from the current session into project memory, routing each
  one to the right place: a project CLAUDE.md, a path-scoped .claude/rules/ file,
  a gitignored CLAUDE.local.md, or user-scope ~/.claude memory. Use at the end of
  a session, or whenever something is worth remembering for next time — a build
  command discovered, a gotcha hit, a convention confirmed, a correction the user
  made. Invoke directly with /revise-claude-md, or let it run when the user says
  "capture what we learned", "update CLAUDE.md with this", "remember this for the
  project", "add this gotcha", or "write this down for next time". For auditing or
  reorganizing memory that already exists (not capturing new learnings), use the
  companion claude-md-improver skill instead.
license: MIT
argument-hint: "[what to capture, or a target file]"
metadata:
  author: "Ikuma Yamashita"
  version: "1.0.0"
---

# Revise CLAUDE.md & Rules

Turn what this session revealed into persistent memory, and put each learning in
the file where it will actually load when it is needed. The original
`/revise-claude-md` only ever wrote to `CLAUDE.md`; this version routes by scope
across the whole memory surface.

This runs in the **main session** because the evidence — the commands run, the
gotcha hit, the correction made — lives in the current conversation. A subagent
with a fresh context window could not see it.

## Step 1: Reflect

What context, if it had been in memory at the start, would have made this session
go better? Look for:

- Build / test / lint commands that were discovered the hard way.
- Code-style or structural conventions that were followed or corrected.
- Gotchas, ordering dependencies, environment quirks.
- A correction the user made that will recur.

Skip one-off fixes, anything obvious from the code, and generic best practice.
A learning earns a place only if a future session would genuinely benefit.

## Step 2: Discover targets

List the memory files that already exist so additions land in the right one:

```bash
find . -path ./node_modules -prune -o \
  \( -name CLAUDE.md -o -name CLAUDE.local.md \) -print 2>/dev/null | sort
ls -1 .claude/rules/*.md .claude/rules/**/*.md 2>/dev/null
```

## Step 3: Route each learning

For every learning, decide its destination **before** drafting. Use the decision
tree in [references/routing-guide.md](references/routing-guide.md). In short:

- Project-wide fact, needed often → `./CLAUDE.md` (or `./.claude/CLAUDE.md`).
- Specific to one file type / directory → `.claude/rules/<topic>.md` with a
  `paths:` glob (create the file if it does not exist).
- Personal, project-specific, not for the team → `./CLAUDE.local.md` (gitignored).
- Personal preference spanning all your projects → `~/.claude/CLAUDE.md` or
  `~/.claude/rules/` — only with the user's explicit go-ahead, since these affect
  every project.

If a learning is really a multi-step procedure rather than a fact, suggest a
skill instead of adding it to memory.

## Step 4: Draft

Keep each addition to one line per concept. Memory is part of every prompt.

Format: `` `<command or pattern>` `` — `<brief reason or effect>`.

Avoid verbose explanations, restating the code, and duplicating something already
present in another memory file.

## Step 5: Show proposed changes

For each addition, show the target, a one-line why, and a diff:

```text
### Update: ./.claude/rules/testing.md  (new path-scoped rule)
Why: tests need a local Redis; this only matters under tests/.

+ ---
+ paths:
+   - "tests/**/*.ts"
+ ---
+
+ # Testing
+ - Integration tests need a local Redis on :6379 (`docker compose up redis`).
```

## Step 6: Apply with approval

Only edit files the user approves. When routing creates a new rule file, confirm
the `paths:` glob matches real files. Never write to `~/.claude/` memory as a
side effect — that scope requires an explicit yes.
