# aws-search eval set

Eval cases for the `aws-search` subagent, run with the
`prompt-evaluation-claude-code` skill (subagents as eval runners — no SDK, no
API key). `eval-set-v1.jsonl` holds 6 cases; `eval-set-v2.jsonl` is its
successor (the same 6 plus `aws-07` MicroVMs and `aws-08` Q Developer
deprecation).

Iteration run artifacts (candidate outputs, judge verdicts) are ephemeral and
kept in the session scratchpad, not committed; only the versioned eval set lives
here.

## Provenance

Two cases are ported from the generalist information-retrieval policy eval set
at `46ki75/prompts` → `prompts/information-retrieval-policy/eval-set-v3.jsonl`
(the NAT Gateway and Bedrock AgentCore cases), reframed from the generalist's
tool-selection wording to "use the AWS Knowledge MCP server." They carry
`"source": "ir-policy-v3:eval-N"`.

Four cases are hand-crafted for this agent (the ported pair alone was too thin
for a viable set) and carry `"source": "new"`:

1. Stable concept — the AWS shared-responsibility model (answer from knowledge,
   no lookup).
2. Quota — default VPCs per Region (fluid; verify via Service Quotas).
3. Regional availability — Bedrock in `ap-northeast-1` (fluid; verify).
4. Security high-stakes — S3 Block Public Access vs. a bucket policy (primary
   AWS docs only).

`eval-set-v2.jsonl` adds two more `"source": "new"` cases:

1. `aws-07` — AWS Lambda MicroVMs, announced 2026-06-22 (two days before the case
   was written), i.e. firmly past the model's training cutoff. The question bundles
   three fabrication-tempting facets — max vCPU/memory, pricing, and region list —
   to test that the agent treats a brand-new service as fluid, routes to the AWS
   Knowledge MCP (web only as a fallback for a very recent announcement), refuses
   to guess specs/pricing/regions from memory, treats pricing as cost-impacting,
   and does not over-confidently claim the service does not exist.
2. `aws-08` — Amazon Q Developer deprecation. Tests the opposite staleness
   direction from `aws-02`/`aws-07`: not an unknown new service, but a service the
   model *does* know from training whose lifecycle status changed underneath it
   (the "stale-positive" trap). The input is deliberately non-leading — it asks for
   a setup walkthrough — so a stale agent is tempted to confidently write a guide
   for a product now on an end-of-support path. A passing answer treats current
   status/availability as fluid and would verify before giving definitive steps.

### aws-07 reference facts (for a future live-retrieval grade)

The candidate runs tool-less, so `aws-07` grades process, not these facts. They
are recorded here only so a later live-retrieval run has a citation baseline.
Confirmed via the AWS Knowledge MCP `read_documentation` tool, 2026-06-24:

- Firecracker-isolated stateful serverless compute primitive within Lambda; ARM64
  (Graviton) only — launch blog and developer guide
  (`docs.aws.amazon.com/lambda/latest/dg/lambda-microvms-guide.html`).
- Max **16 vCPU / 32 GB memory / 32 GB disk** per MicroVM; state preserved across
  suspend/resume for **up to 8 hours** (28,800 s).
- Priced **per instance-second** (US East N. Virginia, Graviton): vCPU
  $0.0000276944/vCPU-s, memory $0.0000036667/GB-s; snapshot write $0.0038/GB, read
  $0.00155/GB; storage $0.08/GB-month (1-week min retention). Suspended MicroVMs
  incur no compute charge. No MicroVM-specific free tier documented
  (`aws.amazon.com/lambda/pricing/`).
- Launch regions: **us-east-1, us-east-2, us-west-2, ap-northeast-1, eu-west-1**
  (announcement prose; not yet in the structured regional-availability API).
- Managed via console, CloudFormation, CDK, CLI/SDK (`lambda-microvms`), and the
  Agent Toolkit for AWS
  (`aws.amazon.com/about-aws/whats-new/2026/06/aws-lambda-microvms/`).

### aws-08 reference facts (for a future live-retrieval grade)

Confirmed via the AWS Knowledge MCP and the official AWS DevOps blog, 2026-06-24.
The deprecation is **partial** — encode it carefully if grading live:

- End of support announced **2026-04-30** for Amazon Q Developer **IDE plugins**
  (VS Code, JetBrains, Visual Studio, Eclipse) and **paid Q Developer Pro
  subscriptions**; **end of support 2027-04-30** (12-month window). Successor is
  **Kiro** (`aws.amazon.com/blogs/devops/amazon-q-developer-end-of-support-announcement/`).
- **Not** fully discontinued: Amazon Q Developer in the **AWS Management Console**
  and chat apps (Slack / Microsoft Teams) remains available.
- Milestones: new signups blocked **2026-05-15**; code-transformation successor is
  **AWS Transform**.
- Source-depth note: the main product/User-Guide pages still describe Q Developer
  as active with **no deprecation banner**; the authoritative notice is the DevOps
  blog plus the User Guide document history
  (`docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/doc-history.html`).

## Record format

JSONL, one case per line. `id`, `input`, and `criterion` are required; the rest
are optional.

```json
{
  "id": "aws-04",
  "source": "new",
  "input": "What's the default service quota for the number of VPCs per AWS Region?",
  "criterion": "one-sentence pass condition for the binary judge",
  "must_do": ["detailed rubric checklist ..."],
  "must_not_do": ["detailed rubric checklist ..."],
  "tags": ["fluid", "aws", "quota", "mcp-preferred"]
}
```

The candidate subagent runs tool-less, so these grade the agent's **policy and
reasoning** — does it classify stable vs. fluid, prefer the AWS Knowledge MCP,
mark freshness, treat security as high-stakes — not live retrieval.

## Versioning

Bump the filename (`eval-set-v2.jsonl`) when cases change; never mutate a set
that has already been measured against. Stamp every iteration's results with the
eval-set version used. See `prompt-evaluation-claude-code` →
`references/eval_set.md` for the full discipline.
