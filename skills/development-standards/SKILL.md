---
name: development-standards
description: >
  Org-internal engineering standards. Invoke for repo scaffolding and audits,
  executable quality gates, validation or generation scripts, CI, bug
  investigation or fixes, regression tests, or configuration of `Cargo.toml`,
  `rust-toolchain.toml`, `justfile`, `.editorconfig`,
  `.prettierignore`, `tsconfig.json`, `package.json`,
  `pnpm-lock.yaml`, `bunfig.toml`, `pyproject.toml`, `uv.lock`,
  `.python-version`, `lefthook.yml`, or `*.tf`. Also for `axum`,
  `utoipa`, `async-graphql`, `uv`, `ruff`,
  `pyright`, `pytest`, `eslint`, `prettier`, or Node package-manager
  setup (pnpm is the org default). Fully documented: Rust (workspace
  inheritance, MSRV, `just`, coverage, test tiers, Axum+utoipa,
  published-crate conventions, async-graphql), Python (uv workspaces,
  `src` layout, ruff, pyright strict, pytest tiers), TypeScript/Node
  (tsconfig, multi-context configs, npm scripts), Terraform (workspaces,
  backend, naming). Bun is a stub; invoke anyway rather than improvising.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "0.9.0"
---

# Development Standards

Org-internal engineering standards. This file is a **router** — load the
reference that matches the task, not the whole tree.

Refer to [`references/general/updating-skills.md`](references/general/updating-skills.md)
before updating this or another Skill.

## Enforcement principle

Prefer executable enforcement over prose. When a standard can be expressed
through a formatter, linter, type checker, compiler setting, schema, or test,
configure that tool and enforce it in CI rather than relying on contributors
or AI agents to remember the rule.

Keep written guidance for tool selection, rationale, exceptions, and standards
that cannot be checked mechanically. Avoid restating individual tool rules in
prose; the checked-in configuration is the authoritative specification. Use
editor and pre-commit integrations for fast feedback, but keep CI authoritative.
When a recurring workflow or quality requirement still exists only as a
checklist, look for the smallest reliable script, test, or validator that can
make it executable. Read `references/general/executable-quality.md` when
designing or auditing those mechanisms, and whenever investigating or fixing a
bug.

## Routing

### Cross-cutting — `references/general/`

- `executable-quality.md`: Quality gates, validation and generation scripts,
  bug investigation and fixes, contract tests, generated-file or metadata
  drift, safe automation, and evaluation of stochastic systems.
- `git-repository.md`: New repo setup, `.editorconfig`, repository-wide
  Prettier formatting, pnpm, lefthook git hooks, and Claude Code `PostToolUse`
  integration.
- `updating-skills.md`: Updating Skills safely: policy ownership, cross-reference
  audits, composable examples, evaluations, provenance, and validation.

Commit-message conventions are not defined by this skill. Follow the current
repository's contributor instructions, or upstream Conventional Commits when
the repository explicitly adopts it.

### Rust — `references/rust/`

| File             | When to read                                                                                                                                                                                          |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `general.md`     | Any Rust project: workspace inheritance, `rust-toolchain.toml`, `just` recipes, `cargo-llvm-cov`, integration test tiers.                                                                             |
| `web-openapi.md` | HTTP API with `axum` + `utoipa`: `OpenApiRouter`, Controller/UseCase/Repository layering, `ToSchema` DTOs, error mapping, Swagger UI.                                                                 |
| `web-graphql.md` | HTTP API with `async-graphql`: schema composition, Repository/Service/Resolver layering, `ComplexObject` lazy fields. Superseded by `web-openapi.md` for new work — read the status note at the top.  |
| `library.md`     | Publishable crates: multi-crate SDK layout, proc-macro crate splits, versioning/release conventions, `readonly`/`mutable` test tiers, gaps to close (docs.rs metadata, sealed traits, semver checks). |

### Python — `references/python/`

| File         | When to read                                                                                                                                                             |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `general.md` | Any Python project: uv workspaces, `.python-version` pinning, packaged `src` layout, ruff, pyright strict, stdlib `logging`, pytest hermetic/live tiers, `just` recipes. |

### TypeScript — `references/typescript/`

- `general.md`: Any TypeScript project: `tsconfig.json` baseline, separate
  configs for multi-context packages, inline type-only imports, Prettier, and
  Stylelint.
- `eslint.md`: Any TypeScript ESLint setup or audit: flat config, typed linting,
  strict rules, migration warnings, framework presets, Vitest rules,
  exceptions, and quality-gate integration.

### Node.js — `references/nodejs/`

| File         | When to read                                                                                                                  |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| `general.md` | Any Node project: dot-namespaced `package.json` scripts, `engines` policy, OpenAPI-to-TypeScript client generation, CI shape. |

### Terraform — `references/terraform/`

| File         | When to read                                                                                                                                       |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `general.md` | Any Terraform config: flat file-per-resource layout, S3/Terraform-Cloud backend choice, workspace-based environments, naming, GitHub-as-Terraform. |

### Planned but unwritten

| Section           | Status                       |
| ----------------- | ---------------------------- |
| `references/bun/` | _Stub — not yet documented._ |

## Handling stubbed sections

The user invoked this skill expecting org conventions. If the matching
reference is a stub, **do not improvise an org standard** — that risks
laundering a one-off decision into apparent policy. Instead:

1. Tell the user the section is not yet documented.
2. Look for a de facto convention in the current repo or in sibling
   projects the user has open. If found, propose it and ask whether to
   adopt it as the standard.
3. If no convention exists, offer a recommendation based on general
   engineering judgment, label it clearly as a suggestion (not policy),
   and offer to write it up into the stub once the user decides.

## When NOT to invoke

- General programming or library tutorials — use an available language- or
  library-specific skill (for example `mcp-knowledge` or `ag-ui-knowledge`)
  or upstream docs.
- General questions about business logic that do not involve investigating or
  fixing a defect.
- Reviewing changes that do not touch tooling, project layout, or the
  architectural seams covered in `references/`.
