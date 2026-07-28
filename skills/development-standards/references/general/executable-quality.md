# Executable Quality Standards

Use this reference when designing or auditing quality gates, validation and
generation scripts, contract tests, generated artifacts, or evaluation systems.

## Treat Quality Requirements as Software

A recurring quality requirement is operational only when the normal development
workflow can detect a violation and return an actionable failure. Documentation
explains intent, rationale, and exceptions; an executable mechanism determines
whether the repository currently complies.

Choose the most direct reliable mechanism available:

1. Type, schema, compiler, or package-manager constraint.
2. Formatter, linter, or static analyzer.
3. Focused behavior or contract test.
4. Validation, generation, or drift-detection script.
5. Explicit review criterion when automation would be brittle or misleading.

Do not automate subjective judgment merely to eliminate prose. A noisy check
that developers learn to ignore is weaker than a clearly identified review
criterion. When a requirement cannot be checked reliably, document why and keep
its review scope narrow.

## Provide One Canonical Quality Gate

Expose one repository-level command, normally `just check` or `just ci`, that
composes every check required for a merge. Include all applicable mechanisms:

- formatting and linting;
- compilation, type checking, and package or lockfile consistency;
- hermetic tests;
- schema, manifest, and repository-policy validation;
- generated-artifact and source-provenance drift checks;
- artifact build or packaging verification.

CI invokes this command instead of duplicating its implementation in workflow
YAML. Editors, agent hooks, and pre-commit hooks may run fast subsets for earlier
feedback, but they do not replace the authoritative CI gate. A documented check
that CI does not run is advisory, not enforced.

When adding a package, generator, artifact type, or policy, extend the canonical
gate in the same change. Validators should report all related defects in one run
where practical so humans and agents can repair them in one feedback cycle.

## Make Automation Safe to Run

Treat repository scripts and generators as production code rather than informal
snippets.

- Validate before mutating. Discover, parse, and validate every input before
  deleting, overwriting, publishing, or deploying anything. Preserve the last
  known-good output when validation fails.
- Treat unexpectedly finding no inputs as an error unless empty input is an
  explicitly supported state.
- Make repeated execution deterministic and idempotent. Publication and
  deployment should reconcile current state, skip completed work, and recover
  from partial failure instead of assuming they run exactly once.
- Provide a dry-run or check mode for destructive operations. A check mode
  validates access and preconditions but does not mutate state.
- Make failure unambiguous. Return non-zero when compliance cannot be determined,
  not only when a violation is positively detected.
- Emit actionable diagnostics with the affected path, violated contract, and
  repair direction. Prefer structured output when another tool consumes it.
- Test nontrivial transformations, ordering, exclusion rules, failure paths, and
  safety properties. The script that enforces quality also needs regression
  protection.

For checked-in generated files, support a drift check that regenerates into a
temporary location and compares results, or regenerates in CI and asserts that
the worktree remains clean. Never require a reviewer to recognize stale output
by inspection.

## Test Contracts, Not Checklists

Turn important behavioral claims into focused tests at the boundary where users
or other systems observe them.

### Bug fixes start with a failing regression test

For every bug fix, write an automated regression test that reproduces the defect
before changing any implementation, configuration, migration, generator, script,
manifest, or other artifact intended to correct the defect. During this red
phase, change only test code, fixtures, or test-harness code needed to establish
the reproduction. Run the test and confirm that it fails for the expected reason;
a test that starts green or fails during unrelated setup does not demonstrate the
bug.

Only after recording the expected failure may implementation of the fix begin.
Rerun the same test to prove it passes, then run the relevant wider suite and
keep the regression test to prevent recurrence.

If the defect depends on an external or unsafe system, first capture the smallest
hermetic reproduction at the affected boundary. If no automated reproduction can
be created, stop and explain the blocker rather than changing the artifact that
would fix the defect. Do not implement first and add a test afterward, because a
post-hoc test can accidentally validate the fix without proving that it detects
the original defect.

Diagnosis-only work may inspect code, collect evidence, and run existing checks
without adding a test. If a later fix is proposed, identify the failing regression
test as its first implementation step.

### General contract testing

- Prefer realistic but hermetic adapters, such as an in-memory transport or
  database, over mocks that only restate implementation details.
- Keep the default suite deterministic, isolated, and free of secrets, network
  access, cost, and persistent side effects. Put live or mutating tests behind an
  explicit command and approval boundary.
- For asynchronous behavior, poll an observable state with a bounded timeout
  instead of relying on a fixed sleep.
- Snapshot or otherwise compare generated public contracts such as schemas, API
  descriptions, CLI help, manifests, and archive contents.
- Explain briefly why a non-obvious regression assertion is load-bearing. The
  prose records the reason; the assertion enforces the behavior.

Test the quality system itself. A validator needs boundary tests for every rule;
a quality gate needs a test or audit proving that it discovers every supported
package and artifact; a release pipeline needs tests for partial-failure recovery
and validate-before-mutate behavior.

## Prevent Competing Sources of Truth

- Define one authoritative schema or validator for each artifact format. Make
  packaging and publishing wrappers call it rather than reimplementing the same
  rules in multiple languages.
- When metadata appears in several manifests, generate secondary copies from one
  source or add a consistency check. If versions are intentionally independent,
  encode or test the permitted relationship.
- Pin toolchains and package managers, commit lockfiles, and run CI with those
  pins so results do not depend on a contributor's global environment.
- Record the upstream repository, exact revision, and relevant paths for derived
  documentation, vendored schemas, or generated clients. Add a freshness check
  to the canonical gate, and fail if freshness cannot be determined.
- Scope lint suppressions and generated-file exclusions as narrowly as possible,
  with a reason where the tool supports one. Broad exclusions silently convert
  enforced standards back into prose.

## Evaluate Stochastic Behavior Empirically

For AI, ranking, search, and other stochastic or heuristic behavior, use an
executable evaluation suite in addition to deterministic unit tests.

- Build cases from real tasks and failures, including difficult near misses.
- Give cases stable identifiers and version the evaluation set. Do not change the
  measurement instrument silently while comparing implementations.
- Use machine-verifiable assertions for objective properties and retain human
  review for subjective quality.
- Compare against the previous implementation or a no-feature baseline. Repeat
  runs and report rates or variance rather than trusting one sample.
- Keep held-out cases when optimizing against an evaluation set, and select
  changes using held-out performance rather than training examples alone.
- Validate the evaluation schema and runner. A directory of example prompts is
  useful design evidence, but it is not a regression suite until a repeatable
  command executes and grades it.

Do not force inherently stochastic evaluation into a zero-tolerance merge gate
without enough evidence that the threshold is stable. Use deterministic contract
checks for hard invariants and measured evaluations for quality trends.

## Audit Questions

When reviewing a repository, ask:

1. Which quality claims still rely on someone remembering documentation?
2. What is the smallest reliable mechanism that could check each claim?
3. Does one local command run every required check, and does CI invoke it?
4. Are default commands hermetic and safe, with risky operations explicit?
5. Do generators and release scripts validate before mutation and survive reruns?
6. Are validators, generated outputs, duplicated metadata, and upstream sources
   protected against drift?
7. Are the enforcement mechanisms themselves tested?
