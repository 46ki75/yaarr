# Updating Skills

Use this guide when changing a Skill in this repository. A Skill update is not
only a local documentation edit: routing, related references, examples,
evaluations, source provenance, and release metadata may all form part of the
same contract.

## Establish scope and ownership

Before editing, classify each proposed rule:

- Cross-cutting policy belongs in one general reference.
- Ecosystem policy belongs in the relevant language or runtime reference.
- Tool-specific configuration belongs in that tool's reference.
- Repository-specific implementation remains in the target repository unless
  there is evidence that it is an org standard.

Give each policy one authoritative owner. Other references may link to it and
state the local integration requirement, but should not repeat its scripts,
versions, CI topology, or detailed configuration. For example, an ESLint
reference can define lint coverage and the `eslint` command while a global
quality-gate reference owns how lint composes with every merge requirement.

When importing a practice from another repository, distinguish observed
practice from settled policy. Check sibling repositories and existing
references before presenting one implementation as an org-wide standard.

## Audit before editing

Search the whole Skill for the affected concepts, not only the intended target
file. Include:

- `SKILL.md` routing and trigger metadata;
- detailed and summary references;
- executable examples, scripts, and CI snippets;
- eval prompts, expected outputs, and assertions;
- source metadata such as `.sources.json` when guidance is derived upstream.

Review workspace, monorepo, and polyglot cases when the policy can cross package
or language boundaries. A locally correct example may still conflict with the
repository-level gate or leave newly added packages undiscovered.

## Preserve progressive disclosure

Keep `SKILL.md` as a concise router. Put detailed guidance in a reference and
add a routing entry that says when to read it. Split a topic into its own
reference when it has independent setup, policy, exceptions, or evaluation
needs; do not make every task load unrelated detail.

Cross-references are preferable to duplicated policy. A short integration
invariant is useful when needed for correct behavior, but the linked owner
remains authoritative.

## Keep examples executable and composable

Treat examples as code that users and agents may apply literally:

- Ensure related snippets can coexist without duplicate exports, imports, or
  competing script names.
- Keep tool-specific leaf commands separate from repository-level gate
  composition.
- Make summary examples agree with detailed coverage requirements.
- Show complete composition when separate configuration objects must be used
  together.
- Preserve the distinction between fast local feedback and the authoritative
  merge gate.

Do not repair a contradiction by copying the preferred policy into another
reference. Fix the authoritative owner, replace duplicates with links, and then
audit remaining examples for stale assumptions.

## Update evaluations with behavior

Add or revise evals when an update changes what the Skill should produce or
detect. Use realistic prompts that exercise the policy in context rather than
checking whether new wording is repeated.

Cover both setup and audit behavior when useful. Assertions should detect
integration failures such as incomplete workspace coverage, a specialized
reference redefining global policy, invalid configuration composition, or an
audit request modifying files.

Keep eval JSON structurally consistent with the existing suite. A prompt file
is design evidence, not regression protection unless the repository has a
repeatable runner and grading path for it.

## Track provenance and release metadata

When a Skill derives documentation from an upstream project, update its source
record with the exact revision and relevant paths. Do not silently combine
guidance from different upstream versions.

Update `metadata.version` when the changed Skill is intended for release,
following the repository's release convention. Keep related edits in one
unreleased version rather than incrementing the version repeatedly while the
same change set is still under review.

## Validate the complete update

Set `SKILL_PATH` to the Skill being updated, then run the checks that apply to
the repository, including:

```sh
SKILL_PATH=skills/development-standards
git diff --check
pnpm fmt.check
cargo run -p skill-cli -- check
python .agents/skills/skill-creator/scripts/quick_validate.py "$SKILL_PATH"
```

Also parse changed eval JSON and inspect the complete diff, including untracked
references. Validation should confirm both document shape and policy coherence;
a passing formatter check cannot detect contradictory examples or competing
sources of truth.

Before finishing, confirm:

1. The entry point routes the new topic without absorbing its details.
2. One reference clearly owns each policy.
3. Related examples are executable, composable, and mutually consistent.
4. Global gates still discover every maintained package and artifact.
5. Evals cover new behavior and important near-miss cases.
6. Provenance and release metadata reflect the update where applicable.
