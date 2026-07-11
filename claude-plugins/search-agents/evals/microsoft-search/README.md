# microsoft-search eval set

Eval cases for the `microsoft-search` subagent, run with the
`prompt-evaluation-claude-code` skill (subagents as eval runners — no SDK, no
API key). `eval-set-v1.jsonl` holds 7 cases.

Iteration run artifacts (candidate outputs, judge verdicts) are ephemeral and
kept in the session scratchpad, not committed; only the versioned eval set lives
here.

## Provenance

All seven cases are hand-crafted for this agent (`"source": "new"`) — the
Microsoft/Azure domain was not part of the generalist information-retrieval
policy set the `web-search` and `aws-search` sets were ported from, so there was
nothing to port. They mirror the coverage shape of the `aws-search` set:

1. `ms-01` — Stable concept (C# `struct` vs. `class`): answer from knowledge, no
   lookup, no freshness marker.
2. `ms-02` — Software version (latest .NET LTS): fluid; verify via the Microsoft
   Learn MCP server and mark freshness.
3. `ms-03` — Regional availability (Azure OpenAI in Japan East): fluid; verify.
4. `ms-04` — Quota (storage accounts per subscription per region): fluid; verify
   via the Azure service-limits docs.
5. `ms-05` — Renamed product / "stale-positive" (Azure AD → Microsoft Entra ID):
   the model knows the old name and portal flow from training; a passing answer
   treats current naming and setup steps as fluid and verifies rather than
   confidently writing a guide under stale branding. This is the parallel to
   `aws-search`'s `aws-08`.
6. `ms-06` — Code sample (upload to Blob Storage in Python): exercises the
   broader scope of this agent — it should prefer `microsoft_code_sample_search`
   for an official, current sample over SDK code recalled from memory, and cite
   the source page.
7. `ms-07` — High-stakes security (Storage private endpoint vs. public access):
   ground in primary Microsoft docs; the correct substance is that a private
   endpoint alone does not disable the public endpoint — public network access
   must also be disabled/restricted.

## Record format

JSONL, one case per line. `id`, `input`, and `criterion` are required; the rest
are optional.

```json
{
  "id": "ms-04",
  "source": "new",
  "input": "What's the default maximum number of storage accounts per Azure subscription per region?",
  "criterion": "one-sentence pass condition for the binary judge",
  "must_do": ["detailed rubric checklist ..."],
  "must_not_do": ["detailed rubric checklist ..."],
  "tags": ["fluid", "azure", "quota", "mcp-preferred"]
}
```

The candidate subagent runs tool-less, so these grade the agent's **policy and
reasoning** — does it classify stable vs. fluid, prefer the Microsoft Learn MCP,
mark freshness, treat security as high-stakes, prefer official code samples over
memory — not live retrieval.

## Versioning

Bump the filename (`eval-set-v2.jsonl`) when cases change; never mutate a set
that has already been measured against. Stamp every iteration's results with the
eval-set version used. See `prompt-evaluation-claude-code` →
`references/eval_set.md` for the full discipline.
