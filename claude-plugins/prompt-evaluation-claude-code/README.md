# prompt-evaluation-claude-code

A Claude Code plugin for eval-driven prompt refinement that runs entirely inside
Claude Code via the Agent/Task tooling. It bundles two skills:

- **`prompt-evaluation-claude-code`** — the full eval loop: build an eval set,
  spawn candidate and judge subagents in parallel, aggregate verdicts, and
  iterate on the prompt.
- **`capture-eval-case`** — a manually-invoked skill (`/capture-eval-case`) that
  turns an incorrect, stale, or mis-routed answer you just saw in the
  conversation into a well-formed regression eval case, under the
  bump-don't-mutate versioning discipline.

## Install

```bash
/plugin marketplace add 46ki75/claude-plugins
/plugin install prompt-evaluation-claude-code@46ki75-plugins
```
