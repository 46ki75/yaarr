# web-search eval set

Eval cases for the `web-search` subagent, run with the
`prompt-evaluation-claude-code` skill (subagents as eval runners — no SDK, no
API key). `eval-set-v1.jsonl` holds 20 cases.

Iteration run artifacts (candidate outputs, judge verdicts) are ephemeral and
kept in the session scratchpad, not committed; only the versioned eval set lives
here.

## Provenance

Derived from the generalist information-retrieval policy eval set at
`46ki75/prompts` → `prompts/information-retrieval-policy/eval-set-v3.jsonl`
(22 cases). When that agent was split into `web-search` and `aws-search`, the
non-AWS cases moved here and the criteria were adapted:

1. The generalist tested **tool selection** ("name context7 as the tool to
   consult"). `web-search` has a fixed toolset (`WebSearch`/`WebFetch`), so
   those clauses were dropped.
2. Software-docs cases (Next.js, Tailwind, Toasty) now expect a web search
   rather than a dedicated docs tool.

Ported cases carry `"source": "ir-policy-v3:eval-N"`.

## Record format

JSONL, one case per line. `id`, `input`, and `criterion` are required; the rest
are optional.

```json
{
  "id": "ws-02",
  "source": "ir-policy-v3:eval-2",
  "input": "What's the latest stable version of Next.js?",
  "criterion": "one-sentence pass condition for the binary judge",
  "golden_answer": "only when a single reference answer exists",
  "must_do": ["detailed rubric checklist ..."],
  "must_not_do": ["detailed rubric checklist ..."],
  "tags": ["fluid", "software-version", "web-search"]
}
```

The candidate subagent runs tool-less, so these grade the agent's **policy and
reasoning** — classify stable vs. fluid, mark freshness, refuse correctly — not
live retrieval.

## Versioning

Bump the filename (`eval-set-v2.jsonl`) when cases change; never mutate a set
that has already been measured against. Stamp every iteration's results with the
eval-set version used. See `prompt-evaluation-claude-code` →
`references/eval_set.md` for the full discipline.
