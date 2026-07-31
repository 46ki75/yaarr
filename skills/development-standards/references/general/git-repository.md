# Git Repository Standards

## Contents

- [EditorConfig](#create-editorconfig)
- [Package manager](#package-manager-pnpm)
- [Git hooks](#git-hooks-lefthook)
- [Repository formatting](#repository-formatting-with-prettier)

## Create `.editorconfig`

```ini
# editorconfig.org
root = true

[*]
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true
```

## Package manager: pnpm

Use pnpm for anything Node-based — including repos whose primary
language is not JavaScript but which carry Node tooling (e.g.
Prettier in a Rust repo). This is org policy; do not switch
to npm or yarn because a repo happens to have their artifacts lying
around.

- Declare `"packageManager": "pnpm@<exact-version>"` in `package.json`.
  Run `corepack use pnpm@<version>` rather than typing the version by hand.
- Commit `pnpm-lock.yaml`; never `package-lock.json` or `yarn.lock`.
- CI installs with `pnpm install --frozen-lockfile`.

Exception: Bun projects (see `references/bun/`), where Bun is both
runtime and package manager.

## Git hooks: lefthook

Use [lefthook](https://github.com/evilmartians/lefthook) for git hooks, not
husky. Declare it as a root `devDependency` and install it via the
standard npm lifecycle hook:

```json
{
  "scripts": {
    "prepare": "lefthook install"
  },
  "devDependencies": {
    "lefthook": "^2.1.9"
  }
}
```

The root `prepare` script installs the hook explicitly. Do not also allow a
dependency lifecycle script for lefthook; that duplicates installation and
grants unnecessary install-time execution.

### Single-package repos

A flat `pre-commit` group covering every language present is enough:

```yaml
pre-commit:
  jobs:
    - name: rustfmt
      glob: "**/*.rs"
      run: cargo fmt -- {staged_files}
      stage_fixed: true
    - name: prettier
      glob: "*.md"
      run: pnpm exec prettier --write {staged_files}
      stage_fixed: true
    - name: terraform-fmt
      glob: "**/*.tf"
      run: terraform fmt {staged_files}
```

### pnpm monorepos

Scope each job to its package with `root:`/`glob:` so a tool never has to
guess which package's config applies:

```yaml
pre-commit:
  jobs:
    - run: pnpm run --recursive check

check:
  jobs:
    - name: eslint-foo
      root: "packages/foo/"
      glob: "packages/foo/**/*.{ts,tsx}"
      run: pnpm --filter foo lint
    - name: vitest-foo
      root: "packages/foo/"
      glob: "packages/foo/**/*.{ts,tsx}"
      run: pnpm --filter foo exec vitest related --run --passWithNoTests {files}
```

Tools with no meaningful per-package config (e.g. Prettier across a whole
monorepo) run once from the repo root instead of being duplicated per
package.

### File-list template convention

Pick the template by where the file list actually comes from:

- **Real Git hooks** use the template that names their source:
  `{staged_files}` in `pre-commit`, `{push_files}` in `pre-push`. The
  template name documents the data source.
- **Custom manual-run groups** (`check`, `fmt`, `claude-check`, … —
  anything invoked via `lefthook run <group> --file <path>` or
  `--all-files`) use `{files}`. The caller supplies the list; the
  `--file`/`--all-files` flags force-substitute it into whatever template
  the job uses, so `{staged_files}` would _work_ there, but it claims a
  data source that isn't real. `{files}` also has the safer fallback: with
  no `files:` command and no `--file` flag, the list is empty and every
  job skips, whereas a bare `lefthook run` of a `{staged_files}` group
  would run against whatever happens to be staged at that moment.

### Claude Code hook integration

Wire Claude Code to the same checks as `pre-commit` so the agent fixes
lint errors at edit time instead of at commit time. Three pieces:

**1. A custom `claude-check` group in `lefthook.yml`** — same jobs as
`pre-commit`, with two deliberate differences: `{files}` instead of
`{staged_files}` (see the template convention above), and **no
`stage_fixed`** — the agent's edits must never be staged implicitly.

```yaml
# Custom hook (not a Git hook): run by the Claude Code PostToolUse hook on
# each file the agent edits — `lefthook run claude-check --file <path>`.
claude-check:
  parallel: true
  jobs:
    - name: ruff-format
      glob: "*.py"
      run: uv run ruff format {files}
    - name: ruff-check
      glob: "*.py"
      run: uv run ruff check --fix {files}
    - name: prettier
      glob: "*.md"
      run: pnpm exec prettier --write {files}
```

**2. `.claude/settings.json`** — a `PostToolUse` hook on `Edit|Write`:

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/post-edit.sh"
          }
        ]
      }
    ]
  }
}
```

**3. `.claude/hooks/post-edit.sh`** (committed, `chmod +x`):

```sh
#!/bin/sh
# Claude Code PostToolUse hook: run the lefthook `claude-check` jobs on the
# file the agent just edited. Exit 2 feeds the tool output back to the agent
# so it can fix lint errors itself.
set -u

file=$(jq -r '.tool_input.file_path // empty')
[ -z "$file" ] && exit 0

# Only check files inside this project.
case "$file" in
  "$CLAUDE_PROJECT_DIR"/*) ;;
  *) exit 0 ;;
esac

cd "$CLAUDE_PROJECT_DIR" || exit 0

out=$(NO_COLOR=1 pnpm exec lefthook run claude-check --file "$file" 2>&1) || {
  echo "$out" >&2
  exit 2
}
exit 0
```

Behavioral notes, each load-bearing:

- **Exit 2 on failure, output on stderr** — that is the PostToolUse
  contract for feeding diagnostics back to the agent; any other non-zero
  exit only shows the user.
- **Auto-fixable issues exit 0 silently** — formatters and `--fix` rules
  repair the file in place; only unfixable diagnostics (undefined names,
  real lint errors) block and round-trip to the agent.
- **`NO_COLOR=1`** — lefthook's decorated output is ANSI-heavy; without
  it the agent receives escape-code soup.
- **Skip files outside `$CLAUDE_PROJECT_DIR`** — the agent also writes to
  scratchpads and other working directories; linting those is noise.
- Hooks are snapshotted at session start: a newly added
  `.claude/settings.json` takes effect in new sessions (or after `/hooks`
  review in the current one).

## Repository formatting with Prettier

Use [Prettier](https://prettier.io/) to format repository-owned Markdown and
other file types it supports. Prettier is a formatter, not a Markdown linter;
the org does not require separate Markdown structure or style rules.

Use Prettier's default configuration. Do not add a configuration file unless
the repository has a concrete reason to deviate. In particular, the default
`proseWrap: "preserve"` avoids noisy reflow of prose that contributors or
upstream sources intentionally wrapped differently.

### `.prettierignore`

Keep the formatting target broad and exclude files that the repository does not
own. Add project-specific paths for vendored or generated files, submodules,
build output, and local scratch data. For example:

```gitignore
node_modules
submodules
target
refs
notes
**/*-workspace
```

Do not ignore a maintained documentation or source directory merely because it
contains many files. If generated or vendored files share a directory with
maintained files, use the narrowest stable patterns that separate them.

### `package.json`

Declare Prettier as a root development dependency and expose repository-wide
write and check commands:

```json
{
  "packageManager": "pnpm@11.13.0",
  "scripts": {
    "fmt": "prettier --write .",
    "fmt.check": "prettier --check ."
  },
  "devDependencies": {
    "prettier": "^3.8.3"
  }
}
```

Pin `packageManager` to the current approved pnpm release; the field requires an
exact version. Use the current approved Prettier release when creating a repo
rather than copying the example version indefinitely.

If a single-package TypeScript repository already has these scripts, widen its
Prettier target to `.` rather than defining a second Markdown-only formatter. In
a monorepo, own these repository-wide scripts at the root and run Prettier once;
do not duplicate the same formatting pass in every workspace package.

### Running and enforcement

```bash
pnpm fmt
pnpm fmt.check
```

Run `fmt` in pre-commit and edit-time hooks so routine differences are repaired
automatically. Run `fmt.check` in CI after `pnpm install --frozen-lockfile` so CI
remains authoritative without modifying the checkout. A repository that only
formats through an editor can still merge unformatted files, so editor
format-on-save is a convenience rather than the quality gate.
