---
name: capture-eval-case
description: >-
  Capture an agent's wrong, stale, unsupported, or mis-routed answer from the
  current conversation into a standalone eval case in the project's JSONL eval
  set. Run manually with /capture-eval-case right after you spot a bad answer
  worth regression-testing; it reads the failing exchange from the live
  conversation, so it runs in the main session, not a subagent.
license: MIT
argument-hint: "[agent-name | eval-set path | what was wrong]"
disable-model-invocation: true
metadata:
  author: "Ikuma Yamashita"
  version: "1.0.0"
---

# Capture eval case

Turn a failure you just witnessed into a regression test. A wrong answer is the
most valuable eval input there is — it is a *real* failure, already stratified by
reality, not a synthetic guess. This skill captures it before the context
scrolls away.

## Why this is a skill, not a subagent

The evidence lives in the **current conversation**: the question that was asked,
the answer that came back, and the turn where you (or the user) noticed it was
wrong. A subagent starts with a fresh context window and cannot see any of that,
so this runs in the main session. (You may still *spawn* a subagent for one
narrow job — confirming ground truth; see Step 3.)

## When to invoke

Manually, with `/capture-eval-case`, right after an answer turns out to be:

- **Factually wrong or stale** — wrong number, outdated product name, a feature
  that no longer behaves that way.
- **Unsupported** — a confident claim with no source, or a fabricated citation.
- **Mis-routed** — the wrong agent answered (an AWS question handled by
  `web-search`), or an agent answered from memory when it should have verified.
- **Policy-violating** — skipped a required lookup, dropped the freshness
  marker, treated a high-stakes security/billing question casually.

Optional argument: name the agent, the target eval-set path, or one line on what
was wrong, to skip the questions in Step 1.

## Cases must be standalone (they ship)

The eval sets are committed and distributed with the repo, and each case is later
replayed in a **fresh** context where the runner sees only the case's `input`. So
a case cannot lean on anything outside itself — not the conversation it came from,
not your local files, not project-specific names. Both the `input` (the prompt)
and the `criterion` (the expected behavior) must be self-contained and
reproducible by anyone. Decontextualize as you capture: keep the part that
triggers the defect, drop the part that is merely *your* situation.

## Workflow

### Step 1 — Pin down the failure

From the conversation, identify and state back to the user:

1. **The input** — the prompt that triggered the bad answer, captured as a
   **self-contained** `input` (see "Cases must be standalone" above). If the
   failing message was already standalone, take it verbatim. If it leaned on
   earlier turns or project context (a file you had shown, an internal name, "the
   bug above"), rewrite it into a standalone prompt that still triggers the same
   defect, and strip anything private to the project. Then confirm the defect
   still reproduces from the rewritten prompt alone — if it doesn't, you have
   changed the test, not captured it.
2. **The agent under test** — which subagent produced it (e.g.
   `microsoft-search`), or main-thread Claude. This decides which eval set the
   case belongs to.
3. **What went wrong** — the specific defect, in one sentence. "Asserted the
   storage-account limit as 250 from memory with no source" is usable; "the
   answer was bad" is not.

If any of the three is ambiguous — several recent failures, or the skill was
invoked generically — ask the user rather than guessing. The whole value of the
case is that it targets a *real, specific* defect.

### Step 2 — Locate (or create) the eval set

The agent under test usually *has* a canonical eval set — but it may not be one
you can write to. Find it, then branch on **writability**:

- **Writable** — you are in the repo that owns the set (developing the plugin
  itself, or the set lives in the current project). Read the latest
  `eval-set-vN.jsonl` and any `README.md` beside it to match its schema, ID
  sequence, and tag vocabulary, so the new case fits its neighbors. The case
  lands here (Step 5). In this marketplace repo, the search-agent sets live at
  `plugins/search-agents/evals/<agent>/` — one example layout, not a requirement.
- **Read-only** — the agent's set ships inside an *installed* plugin (under
  `~/.claude/plugins/…`), which is immutable. Do **not** try to write there.
  Capture into an eval set in the user's own project instead, and tell them that
  folding the case into the canonical upstream set means opening a PR against the
  plugin's source repository.
- **None yet** — ask the user where to create one (a sensible default is
  `evals/<agent>/eval-set-v1.jsonl` under the current working directory).

Either way, the case must land somewhere the user can actually commit — never
inside the read-only plugin cache.

### Step 3 — Establish ground truth (don't fabricate it)

A good `criterion` must encode the *correct* behavior or fact:

- **Policy / behavior failures** — the search agents are graded tool-less on
  their reasoning, so the criterion is about behavior: classified fluid vs.
  stable, preferred the bundled MCP, marked freshness, routed correctly, treated
  a high-stakes topic as high-stakes. You usually know the right behavior from
  the agent's own policy; no external lookup needed.
- **Factual failures** — you need the right answer. Confirm it: ask the user if
  they know it, or **spin up the relevant research subagent** (or use the bundled
  MCP / web tools) to verify the current fact before writing the criterion. The
  verified fact lands in the `golden_answer` field (Step 4). Never invent the
  "correct" answer to grade against — a case with a wrong golden answer is worse
  than no case.

### Step 4 — Draft the case(s)

Write JSONL matching the set's schema. Required: `id`, `input`, `criterion`. Use
the optional fields the existing set uses (`source`, `golden_answer`, `must_do`,
`must_not_do`, `tags`, `notes`):

- `id` — next stable ID in sequence (e.g. `ms-08`); never reuse a retired ID.
  For a brand-new set, pick a short agent prefix and start at `-01` (e.g.
  `ms-01`).
- `source` — provenance, e.g. `"observed-failure"`.
- `input` — the standalone prompt you settled on in Step 1 (verbatim if it was
  already self-contained; otherwise your decontextualized rewrite).
- `criterion` — one sentence, an **observable** pass condition *that the
  witnessed defect would fail*. This is the load-bearing field: if the bad answer
  would still pass your criterion, the case does not discriminate and is
  worthless. Avoid "good" / "high quality". State it against general, public
  ground truth — never against project-private facts (e.g. "matches our config"),
  which a shipped case cannot check.
- `golden_answer` — the verified correct answer for factual cases (enables
  reference-guided judging; see `eval_set.md`). Omit for behavior/policy cases.
- `must_do` / `must_not_do` — short rubric checklist; put the specific defect in
  `must_not_do`.
- `tags` — reuse the set's vocabulary; add one that names the failure mode.
- `notes` — human-only: what the agent actually said, and the date, so the next
  reader knows the case's origin.

One defect per case. If the answer was wrong in two independent ways, write two
cases.

### Step 5 — Add under bump-don't-mutate, then lint

If the target set **has already been measured**, do not edit it in place — that
breaks cross-iteration comparison. Copy it forward to `eval-set-v(N+1).jsonl`,
append the new case(s) with stable IDs, and — if the set has a provenance
`README.md` — add an entry for the new case. If the set is brand-new and
unmeasured, appending in place is fine. See
`../prompt-evaluation-claude-code/references/eval_set.md` (bundled in this
plugin) for the full versioning discipline: bump-don't-mutate, stable IDs, stamp
results with the set version.

Show the user the drafted case(s) before writing. After editing, lint the files
you touched with whatever the project uses — `just check` in this marketplace
repo, otherwise `markdownlint` or the project's configured tooling — and fix any
findings.

## Example

**Observed failure.** You asked `microsoft-search` a quota question and it
answered from memory:

> **Q:** What's the maximum number of storage accounts per Azure subscription per region?
>
> **A (microsoft-search):** 250 per region.

No source, no freshness marker, no sign it treated the limit as fluid — a policy
violation. The captured case (one line in the `.jsonl`; pretty-printed here):

```json
{
  "id": "ms-08",
  "source": "observed-failure",
  "input": "What's the maximum number of storage accounts per Azure subscription per region?",
  "criterion": "Treats the limit as fluid and verifies via the Microsoft Learn MCP server, marking freshness; does not assert a bare number from memory with no source.",
  "must_do": ["Treat the limit as fluid", "Prefer the Microsoft Learn MCP / Azure service-limits docs", "Mark freshness or state verification is needed"],
  "must_not_do": ["Assert a number from memory as current with no caveat or source"],
  "tags": ["fluid", "azure", "quota", "mcp-preferred", "from-memory"],
  "notes": "2026-06-26: microsoft-search answered '250 per region' with no source or freshness marker."
}
```

The `criterion` is written so the bad answer *fails* it: "250 per region" with no
source does not "verify … marking freshness," so the case discriminates. This is
a behavior failure, so `golden_answer` is omitted — the test is about
*verifying*, not the specific number.

## Output

- The new or updated eval-set JSONL **in the user's project** (with a bumped
  filename if the set had already been measured).
- A provenance entry for the new case, if the set keeps a `README.md`.
- A one-line summary to the user: which case ID, against which agent, capturing
  which defect.
