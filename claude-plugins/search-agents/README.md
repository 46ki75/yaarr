# search-agents

A Claude Code plugin bundling three research subagents that classify a question
as stable vs. fluid, look information up only when needed, and return cited,
freshness-marked answers:

- **`web-search`** — factual questions best answered by a general web search
  (recent events, news, prices, current officeholders, trending terminology).
  Restricted to the built-in `WebSearch` and `WebFetch` tools, so it bundles no
  MCP server.
- **`aws-search`** — AWS questions (service behavior, features, quotas, pricing,
  regional availability, API/SDK/CloudFormation details). Prefers the bundled
  **AWS Knowledge MCP server** over a general web search, falling back to the web
  only when the MCP server cannot answer.
- **`microsoft-search`** — Microsoft and Azure questions (Azure service behavior,
  features, quotas, regional availability; .NET/C#/PowerShell and Microsoft 365;
  official docs and SDK code samples). Prefers the bundled **Microsoft Learn MCP
  server** over a general web search, falling back to the web only when the MCP
  server cannot answer.

Claude routes by domain: AWS-specific questions go to `aws-search`, Microsoft and
Azure questions go to `microsoft-search`, and everything else to `web-search`.
None cover library/framework docs better served by a dedicated documentation
tool, nor creative writing.

## Bundled MCP servers

The plugin bundles two official remote HTTP MCP servers (no auth) via
`.mcp.json`:

- **AWS Knowledge MCP server** — `https://knowledge-mcp.global.api.aws`
- **Microsoft Learn MCP server** — `https://learn.microsoft.com/api/mcp`

> Note: a plugin-bundled MCP server loads into the whole session, not just the
> agent that uses it — Claude Code does not scope bundled MCP servers per
> subagent. Enabling this plugin connects both MCP servers for the session.

## Install

```bash
/plugin marketplace add 46ki75/claude-plugins
/plugin install search-agents@46ki75-plugins
```

## Layout

```text
plugins/search-agents/
├── .claude-plugin/plugin.json
├── .mcp.json                 # AWS Knowledge + Microsoft Learn MCP servers
├── agents/
│   ├── web-search.md
│   ├── aws-search.md
│   └── microsoft-search.md
└── evals/
    ├── web-search/           # eval set for the web-search agent
    ├── aws-search/           # eval sets for the aws-search agent
    └── microsoft-search/     # eval set for the microsoft-search agent
```
