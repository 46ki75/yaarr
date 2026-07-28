---
name: mcp-knowledge
description: >
  Expert guidance for the Model Context Protocol (MCP), the JSON-RPC 2.0
  protocol connecting LLM apps to tools and data. Covers official versions
  2024-11-05 through 2026-07-28, including modern stateless request metadata,
  `server/discover`, MRTR, subscriptions, extension negotiation, transports,
  Resources, Prompts, Tools, and Elicitation, plus legacy initialization,
  sessions, Sampling, Roots, and Tasks. Also covers the Rust SDK `rmcp` 2.2
  and the TypeScript SDK `@modelcontextprotocol/sdk`, including the
  `pkce-challenge` bundler failure. Use when building or debugging MCP clients,
  servers, transports, authorization, or SDK integrations. Always invoke for
  questions mentioning MCP, modelcontextprotocol, `rmcp`, tools/list,
  tools/call, server/discover, subscriptions/listen, MRTR, resultType,
  Streamable HTTP, Mcp-Session-Id, or `@modelcontextprotocol/sdk`.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.3.0"
---

# MCP Skill

You are an expert in the Model Context Protocol (MCP) — an open, JSON-RPC 2.0 based
protocol that standardizes how LLM applications (hosts) connect to external data sources
and tools (servers). MCP revisions through 2025-11-25 use stateful sessions established
by `initialize`; 2026-07-28 removes protocol sessions and makes each request
self-contained. Always distinguish these eras before giving implementation guidance.

## What MCP Is

MCP follows a client-host-server architecture. A **host** (e.g., an IDE or chat app)
runs clients that communicate with **servers** exposing Resources, Prompts, and Tools.
In 2026-07-28, clients send protocol version and capabilities on every request; servers
return client-input needs through Multi Round-Trip Requests (MRTR), rather than initiating
JSON-RPC requests. The protocol supports stdio, Streamable HTTP, and custom transports.

## Versions

There are five public versions:

| Version        | Status        | Key additions                                                                                                                                                                                                              |
| :------------- | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **2024-11-05** | First release | stdio + HTTP/SSE transports; Resources, Prompts, Tools, Sampling, Roots; Pagination, Logging, Cancellation, Ping, Progress, Completion                                                                                     |
| **2025-03-26** | Stable        | OAuth 2.1 authorization; Streamable HTTP transport; JSON-RPC batching; tool annotations; audio content type; completions capability                                                                                        |
| **2025-06-18** | Stable        | Elicitation; structured tool output; resource links in tool results; OAuth Resource Server classification; RFC 8707 Resource Indicators; `MCP-Protocol-Version` header; removed batching; `title` field; `_meta` expansion |
| **2025-11-25** | Stable        | Tasks utility; OpenID Connect discovery; tool/resource/prompt icons; incremental scope consent; URL-mode elicitation; tool calling in sampling; OAuth Client ID Metadata Documents                                         |
| **2026-07-28** | Latest stable | Stateless requests; discovery; MRTR; subscriptions; caching; extensions; deprecations                                                                                                                                      |

Use **2026-07-28** for new protocol designs, but verify that the selected SDK and peer
implementations support it. Use **2025-11-25** when interoperability requires the legacy
initialization/session model. Never mix wire semantics from the two eras.

## Core Concepts

- **Host** — the LLM application; manages client lifecycle and user consent
- **Client** — created by the host; sends requests to a server
- **Server** — exposes Resources, Prompts, or Tools; may request Sampling
- **Capability declaration** — sent during `initialize` through 2025-11-25; sent in
  `_meta.io.modelcontextprotocol/clientCapabilities` on every 2026-07-28 request
- **Discovery** — `server/discover` advertises modern server versions, capabilities,
  and identity; clients may also negotiate by handling `UnsupportedProtocolVersionError`
- **Resources** — application-driven context: files, DB schemas, live data (URI-addressed)
- **Prompts** — user-triggered prompt templates with optional arguments
- **Tools** — model-controlled functions that call external systems
- **MRTR** — 2026-07-28 server input requests are embedded in `InputRequiredResult`, then
  answered by retrying the original request with `inputResponses` and new JSON-RPC ID
- **Subscriptions** — `subscriptions/listen` carries opted-in change notifications
- **Sampling** — client-provided LLM completions; deprecated in 2026-07-28
- **Roots** — client-declared filesystem boundaries; deprecated in 2026-07-28
- **Elicitation** — server requests structured input from the user via the client (2025-06-18+)
- **Tasks** — experimental core feature in 2025-11-25; redesigned as the negotiated
  `io.modelcontextprotocol/tasks` extension in 2026-07-28

## Reference Files

Each reference file is the full official MDX document for that topic, converted from
`submodules/modelcontextprotocol/docs/specification/` or the official extensions docs.

### 2024-11-05

| File                                    | Content                                                                                   |
| :-------------------------------------- | :---------------------------------------------------------------------------------------- |
| `references/2024-11-05/architecture.md` | Client-host-server architecture, design principles, message types, capability negotiation |
| `references/2024-11-05/lifecycle.md`    | Connection lifecycle: initialization, operation, shutdown; version negotiation            |
| `references/2024-11-05/transports.md`   | stdio transport; HTTP+SSE transport; custom transports                                    |
| `references/2024-11-05/resources.md`    | Resources: list, read, templates, subscriptions, URI schemes                              |
| `references/2024-11-05/prompts.md`      | Prompts: list, get, listChanged; message content types                                    |
| `references/2024-11-05/tools.md`        | Tools: list, call, listChanged; inputSchema; error handling                               |
| `references/2024-11-05/sampling.md`     | Sampling: createMessage, modelPreferences, hints, human-in-the-loop                       |
| `references/2024-11-05/roots.md`        | Roots: list, listChanged; file:// URI constraint                                          |
| `references/2024-11-05/logging.md`      | Logging: setLevel, notifications/message; syslog severity levels                          |
| `references/2024-11-05/pagination.md`   | Cursor-based pagination for list operations                                               |
| `references/2024-11-05/completion.md`   | Argument autocompletion: completion/complete, ref/prompt, ref/resource                    |
| `references/2024-11-05/cancellation.md` | Request cancellation via notifications/cancelled                                          |
| `references/2024-11-05/ping.md`         | Ping/pong for connection health                                                           |
| `references/2024-11-05/progress.md`     | Progress tracking via progressToken and notifications/progress                            |

### 2025-03-26

| File                                     | Content                                                                                               |
| :--------------------------------------- | :---------------------------------------------------------------------------------------------------- |
| `references/2025-03-26/changelog.md`     | Delta from 2024-11-05: OAuth 2.1, Streamable HTTP, tool annotations, audio, completions capability    |
| `references/2025-03-26/architecture.md`  | Architecture (unchanged)                                                                              |
| `references/2025-03-26/lifecycle.md`     | Lifecycle (unchanged)                                                                                 |
| `references/2025-03-26/transports.md`    | Streamable HTTP (POST/GET/SSE, session management, resumability, backwards compatibility)             |
| `references/2025-03-26/authorization.md` | OAuth 2.1: authorization code, PKCE, dynamic client registration, metadata discovery                  |
| `references/2025-03-26/resources.md`     | Resources (unchanged)                                                                                 |
| `references/2025-03-26/prompts.md`       | Prompts (unchanged)                                                                                   |
| `references/2025-03-26/tools.md`         | Tools: adds annotations (readOnlyHint, destructiveHint, idempotentHint, openWorldHint); audio content |
| `references/2025-03-26/sampling.md`      | Sampling: adds audio content type                                                                     |
| `references/2025-03-26/roots.md`         | Roots (unchanged)                                                                                     |
| `references/2025-03-26/logging.md`       | Logging (unchanged)                                                                                   |
| `references/2025-03-26/pagination.md`    | Pagination (unchanged)                                                                                |
| `references/2025-03-26/completion.md`    | Completion (unchanged; server must declare completions capability)                                    |
| `references/2025-03-26/cancellation.md`  | Cancellation (unchanged)                                                                              |
| `references/2025-03-26/ping.md`          | Ping (unchanged)                                                                                      |
| `references/2025-03-26/progress.md`      | Progress: adds optional message field                                                                 |

### 2025-06-18

| File                                     | Content                                                                                                                           |
| :--------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| `references/2025-06-18/changelog.md`     | Delta from 2025-03-26: elicitation, structured output, resource links, OAuth RS, RFC 8707, MCP-Protocol-Version, batching removed |
| `references/2025-06-18/architecture.md`  | Architecture (unchanged)                                                                                                          |
| `references/2025-06-18/lifecycle.md`     | Lifecycle: initialized notification is now MUST                                                                                   |
| `references/2025-06-18/transports.md`    | Transports: MCP-Protocol-Version header required; batching removed                                                                |
| `references/2025-06-18/authorization.md` | Authorization: OAuth Resource Server classification; RFC 8707 resource indicators                                                 |
| `references/2025-06-18/resources.md`     | Resources: adds title field                                                                                                       |
| `references/2025-06-18/prompts.md`       | Prompts: adds title field                                                                                                         |
| `references/2025-06-18/tools.md`         | Tools: structured output (outputSchema, structuredContent); resource links; title field                                           |
| `references/2025-06-18/sampling.md`      | Sampling (unchanged)                                                                                                              |
| `references/2025-06-18/roots.md`         | Roots (unchanged)                                                                                                                 |
| `references/2025-06-18/elicitation.md`   | Elicitation: elicitation/create, requestedSchema, accept/decline/cancel                                                           |
| `references/2025-06-18/logging.md`       | Logging (unchanged)                                                                                                               |
| `references/2025-06-18/pagination.md`    | Pagination (unchanged)                                                                                                            |
| `references/2025-06-18/completion.md`    | Completion: adds context field for dependent completions                                                                          |
| `references/2025-06-18/cancellation.md`  | Cancellation (unchanged)                                                                                                          |
| `references/2025-06-18/ping.md`          | Ping (unchanged)                                                                                                                  |
| `references/2025-06-18/progress.md`      | Progress (unchanged)                                                                                                              |

### 2025-11-25

| File                                     | Content                                                                                                                      |
| :--------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `references/2025-11-25/changelog.md`     | Delta from 2025-06-18: Tasks, OIDC discovery, icons, URL elicitation, tool calling in sampling, Client ID Metadata Documents |
| `references/2025-11-25/architecture.md`  | Architecture (unchanged)                                                                                                     |
| `references/2025-11-25/lifecycle.md`     | Lifecycle (unchanged)                                                                                                        |
| `references/2025-11-25/transports.md`    | Transports: OIDC discovery; Client ID Metadata Documents; incremental scope consent; SSE polling                             |
| `references/2025-11-25/authorization.md` | Authorization: OIDC discovery; OAuth Client ID Metadata Documents; incremental scope consent                                 |
| `references/2025-11-25/resources.md`     | Resources: adds icon field                                                                                                   |
| `references/2025-11-25/prompts.md`       | Prompts: adds icon field                                                                                                     |
| `references/2025-11-25/tools.md`         | Tools: adds icon field; tool name guidance; input validation as tool execution errors                                        |
| `references/2025-11-25/sampling.md`      | Sampling: tool calling in sampling (tools, toolChoice); sampling.tools capability                                            |
| `references/2025-11-25/roots.md`         | Roots (unchanged)                                                                                                            |
| `references/2025-11-25/elicitation.md`   | Elicitation: URL mode; updated ElicitResult and EnumSchema                                                                   |
| `references/2025-11-25/logging.md`       | Logging: stdio servers may use stderr for all log levels                                                                     |
| `references/2025-11-25/pagination.md`    | Pagination (unchanged)                                                                                                       |
| `references/2025-11-25/completion.md`    | Completion (unchanged)                                                                                                       |
| `references/2025-11-25/cancellation.md`  | Cancellation (unchanged)                                                                                                     |
| `references/2025-11-25/ping.md`          | Ping (unchanged)                                                                                                             |
| `references/2025-11-25/progress.md`      | Progress (unchanged)                                                                                                         |
| `references/2025-11-25/tasks.md`         | Tasks (experimental): durable state machines, polling, deferred results, tasks/get, tasks/cancel                             |

### 2026-07-28

This revision is a breaking transition from legacy session semantics to modern,
stateless request semantics. Start with the changelog and versioning references before
reading an individual feature file.

- Start with `changelog.md`, `protocol.md`, and `versioning.md` for migration,
  stateless request metadata, result types, extension negotiation, and dual-era fallback.
- Read `discovery.md` for `server/discover`; `architecture.md` for modern request
  flows; and `deprecated.md` for feature status and migration paths.
- Read `transports.md`, `stdio.md`, or `streamable-http.md` for transport rules.
- Read `mrtr.md` for `InputRequiredResult` and secure `requestState` handling;
  read `subscriptions.md` for opted-in change notification streams.
- Read `authorization.md`, `authorization-discovery.md`,
  `client-registration.md`, and `authorization-security.md` for OAuth.
- Read `tools.md`, `resources.md`, `prompts.md`, or `elicitation.md` for the
  corresponding core feature and its MRTR behavior.
- Read `caching.md` for `ttlMs` and `cacheScope`; read `tasks-extension.md` for
  the redesigned `io.modelcontextprotocol/tasks` extension.
- Read `extensions.md` for extension identifiers, negotiation, lifecycle, and
  graceful degradation. Read `schema.ts` for authoritative wire types.
- Read `sampling.md`, `roots.md`, or `logging.md` only for retained deprecated
  behavior. The remaining utilities are in `completion.md`, `pagination.md`,
  `cancellation.md`, and `progress.md`.

The release's Tasks overview preserves references to its experimental incubator, but
also links the current official repository: `https://github.com/modelcontextprotocol/ext-tasks`.
Prefer the official repository for current extension implementation details.

## When to Read Which Files

For 2026-07-28 questions, use the modern routing list above. In particular, read
`protocol.md` plus `versioning.md` before advising on sessions or initialization;
`mrtr.md` before advising on elicitation, sampling, or roots; and
`tasks-extension.md` instead of the legacy core `tasks.md`.

| User is asking about...                    | Read                                                    |
| :----------------------------------------- | :------------------------------------------------------ |
| Migrating protocol versions                | `references/{target-version}/changelog.md`              |
| MCP architecture / design principles       | `references/{version}/architecture.md`                  |
| Legacy initialization / sessions           | `references/2025-11-25/lifecycle.md` (or earlier)       |
| stdio transport                            | `references/{version}/transports.md`                    |
| HTTP+SSE transport                         | `references/2024-11-05/transports.md`                   |
| Legacy Streamable HTTP / `Mcp-Session-Id`  | `references/2025-11-25/transports.md` (or earlier)      |
| OAuth 2.1 authorization                    | `references/2025-03-26/authorization.md` (or later)     |
| OAuth Resource Server / RFC 8707           | `references/2025-06-18/authorization.md` (or later)     |
| Legacy OIDC / Client ID Metadata           | `references/2025-11-25/authorization.md`                |
| Resources (list/read/subscribe)            | `references/{version}/resources.md`                     |
| Prompts (list/get)                         | `references/{version}/prompts.md`                       |
| Tools (list/call/annotations)              | `references/{version}/tools.md`                         |
| Structured tool output                     | `references/2025-06-18/tools.md` (or later)             |
| Resource links in tool results             | `references/2025-06-18/tools.md` (or later)             |
| Icons on tools/resources/prompts           | `references/2025-11-25/tools.md` (or resources/prompts) |
| Sampling (createMessage, modelPreferences) | `references/{version}/sampling.md`                      |
| Legacy tool calling inside sampling        | `references/2025-11-25/sampling.md`                     |
| Roots (filesystem boundaries)              | `references/{version}/roots.md`                         |
| Elicitation (server→user input)            | `references/2025-06-18/elicitation.md` (or later)       |
| Legacy URL-mode elicitation                | `references/2025-11-25/elicitation.md`                  |
| Logging                                    | `references/{version}/logging.md`                       |
| Pagination                                 | `references/{version}/pagination.md`                    |
| Argument autocompletion                    | `references/{version}/completion.md`                    |
| Cancellation                               | `references/{version}/cancellation.md`                  |
| Ping / connection health                   | `references/{legacy-version}/ping.md`                   |
| Progress notifications                     | `references/{version}/progress.md`                      |
| Legacy core Tasks                          | `references/2025-11-25/tasks.md`                        |
| Rust SDK (`rmcp`) — anything Rust-specific | `references/rust-sdk/overview.md` (indexes the rest)    |
| Upgrading `rmcp` 2.0 through 2.2           | `references/rust-sdk/migration-2.2.md`                  |
| TypeScript SDK build/bundler errors        | `references/typescript-sdk/pkce-challenge.md`           |

## Language SDK Guides

Beyond the spec itself, this skill ships reference material for the official
MCP SDKs. The depth differs by language: the Rust SDK has a full user guide;
the TypeScript SDK currently only documents one well-known build-time quirk.
These materials are language-specific — Rust guidance does not apply to
TypeScript and vice versa. They are pinned to their documented SDK versions and legacy
protocol behavior; do not infer 2026-07-28 support from the SDK guides. Check the SDK's
declared protocol support before translating the latest specification into SDK code.

| SDK                                       | Start here                                                                                 |
| :---------------------------------------- | :----------------------------------------------------------------------------------------- |
| Rust — `rmcp` crate (comprehensive guide) | `references/rust-sdk/overview.md`, then drill via `references/rust-sdk/doc-index.md`       |
| TypeScript — `@modelcontextprotocol/sdk`  | `references/typescript-sdk/pkce-challenge.md` (Vite/bundler `pkce-challenge` resolver fix) |

### Rust SDK (`rmcp`) at a glance

`references/rust-sdk/overview.md` covers workspace orientation, version /
stability notes (Tier 2 conformance — some 2025-11-25 features are still in
motion), Cargo feature flags, and the smallest viable server and client.
From there, `references/rust-sdk/doc-index.md` indexes every per-feature
file: server primitives (tools, prompts, resources, tasks, sampling,
elicitation, roots, transports), client features (handler, requests,
sampling, elicitation, roots, transports, testing), and the shared Cargo
`feature-flags.md`. The canonical local example is `crates/mcp-server/`;
the pinned upstream source of truth is `submodules/mcp-rust-sdk/`.

### TypeScript SDK (`@modelcontextprotocol/sdk`)

Currently scoped to one entry — the Vite/Rollup/Webpack resolver error for
`pkce-challenge` in browser/SSR builds, with a stub-alias workaround. See
`references/typescript-sdk/pkce-challenge.md`. New TypeScript-SDK quirks
should be added as sibling files under `references/typescript-sdk/`.
