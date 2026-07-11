---
name: acp-knowledge
description: >
  Expert guidance for the Agent Client Protocol (ACP) — the JSON-RPC 2.0
  standard connecting code editors/IDEs (Clients) with AI coding agents
  (Agents), the way LSP connected editors with language servers.
  Covers v1 (stable) and v2 (draft): initialization and capability
  negotiation; sessions (new/load/resume/list/delete/close); the prompt
  turn (`session/prompt`, `session/update`, stop reasons); content blocks;
  tool calls and permission requests; agent plans; client `fs/*` and
  `terminal/*` methods; session modes and config options; slash commands;
  cancellation; MCP server passthrough; extensibility (`_meta`); the stdio
  transport; the v1→v2 migration; and the official SDKs (Rust, TypeScript
  `@agentclientprotocol/sdk`, Python, Kotlin, Java).
  Always invoke for ACP, Agent Client Protocol, editor↔agent integration,
  Zed external agents, building an ACP Agent or Client, or any
  `session/*`, `fs/*`, or `terminal/*` protocol method.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.0.0"
---

# ACP Skill

You are an expert in the **Agent Client Protocol (ACP)** — an open standard for
communication between code editors/IDEs (Clients) and AI coding agents (Agents).
ACP does for coding agents what the Language Server Protocol did for language
servers: an agent that implements ACP works in any compatible editor, and an
editor that speaks ACP gains the whole ecosystem of ACP agents.

## What ACP Is

ACP is the **editor↔agent** layer of the AI ecosystem. The user sits in their
editor and reaches out to an agent for a task; the agent runs as a subprocess
of the editor (local) or on remote infrastructure, and streams its work back
into the editor UI. Communication is **JSON-RPC 2.0**, bidirectional: the
Client calls Agent methods (`initialize`, `session/new`, `session/prompt`),
and the Agent calls back into the Client (`session/request_permission`,
`fs/read_text_file`, `terminal/create`) and streams `session/update`
notifications in real time.

Design principles:

- **MCP-friendly** — reuses MCP's `ContentBlock` JSON representation so MCP
  tool output can be forwarded without transformation, and the editor passes
  its user-configured MCP servers to the agent at session setup.
- **UX-first** — purpose-built types for agentic coding UX: streamed message
  chunks, tool-call status, diffs, embedded terminals, plans.
- **Trusted** — the editor grants a trusted agent access to local files and
  MCP servers, while keeping the user in control via permission requests.

The default transport is **stdio** with newline-delimited JSON-RPC; the agent
must write nothing but ACP messages to stdout (logs go to stderr). The default
format for user-readable text is Markdown. All file paths are absolute; line
numbers are 1-based. ACP property keys use `camelCase`; discriminator string
values use `snake_case`.

## Versions

The protocol version is a **single integer**, bumped only for breaking
changes; non-breaking features arrive as **capabilities** that are negotiated
during `initialize` (omitted capability = unsupported).

| Version | Status                                                 | Guidance                                                                   |
| :------ | :----------------------------------------------------- | :------------------------------------------------------------------------- |
| **1**   | Stable, in production (Zed, other clients/agents)      | Default for all current work.                                              |
| **2**   | Published but **labeled draft**; consolidation release | Add behind feature flags; negotiate per connection; keep serving v1 peers. |

v2 in five points: (1) the `session/prompt` response no longer ends the turn —
it acknowledges acceptance, and progress/completion arrive as `state_update`
notifications; (2) updates are **upserts** patched by ID (omitted = unchanged,
`null` = cleared, chunks append); (3) the `fs/*`, `terminal/*`, and session
modes APIs are **removed** in favor of client-provided MCP servers and config
options; (4) capabilities are reorganized under role-agnostic `capabilities` +
required `info`, with a required baseline of session methods; (5) everything is
extensible — enums accept unknown values, `_`-prefixed values are free for
custom use. `authenticate`/`logout` became `auth/login`/`auth/logout`, and
`session/load` was replaced by `session/resume` with `replayFrom`.
Full details: `references/v2/migration.md`.

## Core Concepts

- **Client** — the editor/IDE (or other UI). Manages the environment, user
  interaction, and resource access. Baseline method: `session/request_permission`.
  Optional (v1): `fs/read_text_file`, `fs/write_text_file`, `terminal/*`.
- **Agent** — the AI coding program, typically a Client subprocess. Baseline
  methods (v1): `initialize`, `authenticate`, `session/new`, `session/prompt`.
- **Initialization** — the first exchange: version negotiation plus capability
  and `authMethods` discovery, with `clientInfo`/`agentInfo` metadata.
- **Session** — one conversation/thread, created with an absolute `cwd` and MCP
  server configs; identified by a `sessionId`. May be listed, loaded (full
  history replay), resumed (no replay), closed, and deleted, each behind an
  agent capability.
- **Prompt turn** — one interaction cycle: `session/prompt` → streamed
  `session/update` notifications (message/thought chunks, plans, tool calls,
  usage) → nested tool loops → response with a `stopReason` (`end_turn`,
  `max_tokens`, `max_turn_requests`, `refusal`, `cancelled`).
- **Content block** — MCP-shaped content: `text`, `image`, `audio`,
  `resource` (embedded context, the preferred @-mention form), `resource_link`.
  Support for the richer types is gated by prompt capabilities.
- **Tool call** — reported via `session/update` (`tool_call`/`tool_call_update`)
  with `kind`, `status` (`pending` → `in_progress` → `completed`/`failed`),
  content (including diffs and embedded terminals), locations for
  follow-along, and raw I/O.
- **Permission request** — `session/request_permission` presents options
  (`allow_once`, `allow_always`, `reject_once`, `reject_always`); must resolve
  as `cancelled` when the turn is cancelled.
- **Plan** — `session/update` with plan entries (`content`, `priority`,
  `status`); each update replaces the whole plan.
- **Session config options** — typed selectors (`select`, `boolean`) for
  model/mode/thought level; supersede the older session modes API.
- **Cancellation** — `session/cancel` notification ends the turn with the
  `cancelled` stop reason (never an error); `$/cancel_request` cancels
  individual JSON-RPC requests with error `-32800`.
- **Extensibility** — `_meta` fields on every type, `_`-prefixed custom
  methods/notifications, custom capabilities advertised at init.

## Reference Files

Each file under `references/` is a verbatim copy of the upstream Mintlify docs
at `submodules/agent-client-protocol/docs/` (`.mdx` renamed to `.md`).

### Getting started

| File                                     | Content                                                               |
| :--------------------------------------- | :-------------------------------------------------------------------- |
| `references/get-started/introduction.md` | Why ACP exists; local vs remote agents; LSP analogy.                  |
| `references/get-started/architecture.md` | Design principles; subprocess setup; MCP passthrough/proxy diagrams.  |
| `references/get-started/agents.md`       | Known ACP-compatible agents.                                          |
| `references/get-started/clients.md`      | Known clients, frameworks, connectors, and tools.                     |
| `references/get-started/registry.md`     | The ACP Registry: discovering and installing agents; manifest format. |

### Protocol v1 (stable — default)

| File                                      | Content                                                                                                                                  |        |               |      |                              |
| :---------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- | ------ | ------------- | ---- | ---------------------------- |
| `references/v1/overview.md`               | Communication model; full method/notification catalog for both sides.                                                                    |        |               |      |                              |
| `references/v1/initialization.md`         | Version negotiation; client/agent capabilities; implementation info.                                                                     |        |               |      |                              |
| `references/v1/authentication.md`         | `authMethods`, `authenticate`, `logout`.                                                                                                 |        |               |      |                              |
| `references/v1/session-setup.md`          | `session/new`, `session/load`, `session/resume`, `session/close`; `cwd`; additional directories; MCP server transports (stdio/HTTP/SSE). |        |               |      |                              |
| `references/v1/session-list.md`           | `session/list` pagination; `session_info_update` metadata pushes.                                                                        |        |               |      |                              |
| `references/v1/session-delete.md`         | `session/delete` semantics.                                                                                                              |        |               |      |                              |
| `references/v1/prompt-turn.md`            | The turn lifecycle end to end; message IDs; usage updates; stop reasons; cancellation flow.                                              |        |               |      |                              |
| `references/v1/content.md`                | The five `ContentBlock` types and their prompt-capability gates.                                                                         |        |               |      |                              |
| `references/v1/tool-calls.md`             | Reporting tool calls; statuses; kinds; diffs; embedded terminals; permission options.                                                    |        |               |      |                              |
| `references/v1/file-system.md`            | `fs/read_text_file`, `fs/write_text_file`.                                                                                               |        |               |      |                              |
| `references/v1/terminals.md`              | `terminal/create                                                                                                                         | output | wait_for_exit | kill | release`; building timeouts. |
| `references/v1/agent-plan.md`             | Plan entries and full-replacement updates.                                                                                               |        |               |      |                              |
| `references/v1/session-modes.md`          | Legacy modes API (superseded by config options).                                                                                         |        |               |      |                              |
| `references/v1/session-config-options.md` | Config selectors; categories; `session/set_config_option`; boolean options.                                                              |        |               |      |                              |
| `references/v1/slash-commands.md`         | `available_commands_update`; running commands via prompts.                                                                               |        |               |      |                              |
| `references/v1/cancellation.md`           | `$/cancel_request`; error `-32800`; cascading cancellation.                                                                              |        |               |      |                              |
| `references/v1/extensibility.md`          | `_meta`; `_`-prefixed methods; custom capabilities.                                                                                      |        |               |      |                              |
| `references/v1/transports.md`             | stdio framing rules; custom transports.                                                                                                  |        |               |      |                              |
| `references/v1/schema.md`                 | Generated schema reference for every v1 type (~3,000 lines — jump via anchors).                                                          |        |               |      |                              |

### Protocol v2 (draft)

Same topics restructured for v2, minus the removed surfaces (`fs/*`,
`terminal/*`, session modes) — see `overview.md`, `initialization.md`,
`authentication.md`, `session-setup.md`, `session-list.md`,
`session-delete.md`, `content.md`, `tool-calls.md`, `agent-plan.md`,
`session-config-options.md`, `slash-commands.md`, `cancellation.md`,
`extensibility.md`, `transports.md`, `schema.md` under `references/v2/`. Plus:

| File                                | Content                                                                                                                                  |
| :---------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `references/v2/migration.md`        | **The v1→v2 guide**: method/update-variant change tables; new prompt lifecycle; upsert semantics; supporting both versions side by side. |
| `references/v2/prompt-lifecycle.md` | v2 replacement for the prompt turn: accepted prompts, `state_update`, upserts.                                                           |

### Libraries

| File                                 | Content                                                                               |
| :----------------------------------- | :------------------------------------------------------------------------------------ |
| `references/libraries/rust.md`       | `agent-client-protocol` crate (crates.io).                                            |
| `references/libraries/typescript.md` | `@agentclientprotocol/sdk` npm package; `AgentSideConnection`/`ClientSideConnection`. |
| `references/libraries/python.md`     | `agent-client-protocol` on PyPI; Pydantic models; async base classes.                 |
| `references/libraries/kotlin.md`     | `com.agentclientprotocol:acp` (JVM).                                                  |
| `references/libraries/java.md`       | java-sdk; Spring AI integration examples.                                             |
| `references/libraries/community.md`  | Community-managed libraries (Go, and others).                                         |

### Canonical schema and RFDs (in the submodule)

The machine-readable JSON Schemas live in the submodule, not under
`references/`: `submodules/agent-client-protocol/schema/v1/schema.json` and
`schema/v2/schema.json` (with `schema.unstable.json` variants for opt-in draft
features, and `meta.json` listing method names). Protocol change proposals
(RFDs) live at `submodules/agent-client-protocol/docs/rfds/`.

## When to Read Which Files

Use this matrix as the first stop. Default to `v1/` unless the user is
explicitly targeting v2. Paths are relative to `references/`.

| User is asking about...                                        | Read                                                                         |
| :------------------------------------------------------------- | :--------------------------------------------------------------------------- |
| What ACP is, why it exists, ACP vs LSP/MCP                     | `get-started/introduction.md` then `get-started/architecture.md`             |
| Which agents/editors support ACP; installing agents            | `get-started/agents.md`, `get-started/clients.md`, `get-started/registry.md` |
| Method catalog / who calls what                                | `v1/overview.md`                                                             |
| `initialize`, capability negotiation, protocol version         | `v1/initialization.md`                                                       |
| Login flows, `authMethods`, logout                             | `v1/authentication.md`                                                       |
| Creating/loading/resuming/closing sessions; `cwd`; MCP configs | `v1/session-setup.md`                                                        |
| Session history UI (`session/list`, titles, pagination)        | `v1/session-list.md` then `v1/session-delete.md`                             |
| The prompt turn, streaming updates, stop reasons               | `v1/prompt-turn.md`                                                          |
| Text/image/audio/resource content; @-mentions                  | `v1/content.md`                                                              |
| Reporting tool calls, statuses, diffs; permission prompts      | `v1/tool-calls.md`                                                           |
| Reading/writing files in the editor (unsaved buffers)          | `v1/file-system.md`                                                          |
| Running shell commands, live output, timeouts                  | `v1/terminals.md`                                                            |
| Plans / todo lists in the UI                                   | `v1/agent-plan.md`                                                           |
| Model/mode/reasoning selectors                                 | `v1/session-config-options.md` (legacy: `v1/session-modes.md`)               |
| Slash commands                                                 | `v1/slash-commands.md`                                                       |
| Cancelling turns or individual requests                        | `v1/prompt-turn.md` §Cancellation then `v1/cancellation.md`                  |
| `_meta`, custom methods, custom capabilities                   | `v1/extensibility.md`                                                        |
| stdio framing, remote transports                               | `v1/transports.md`                                                           |
| Exact shape of any type/method                                 | `v1/schema.md` (or `schema/v1/schema.json` in the submodule)                 |
| What's new in v2 / migrating / supporting both versions        | `v2/migration.md`                                                            |
| v2 prompt lifecycle, `state_update`, upserts                   | `v2/prompt-lifecycle.md`                                                     |
| Any v2 surface in depth                                        | the matching file under `v2/`                                                |
| Building an Agent or Client in a specific language             | `libraries/<language>.md`                                                    |
| Proposals / upcoming features (elicitation, HTTP transport, …) | `submodules/agent-client-protocol/docs/rfds/`                                |

## Language Libraries

| Language                | Package                                       | Repository                                                |
| :---------------------- | :-------------------------------------------- | :-------------------------------------------------------- |
| Rust                    | `agent-client-protocol` (crates.io)           | <https://github.com/zed-industries/agent-client-protocol> |
| JavaScript / TypeScript | `@agentclientprotocol/sdk` (npm)              | <https://github.com/agentclientprotocol/typescript-sdk>   |
| Python                  | `agent-client-protocol` (PyPI)                | <https://github.com/agentclientprotocol/python-sdk>       |
| Kotlin (JVM)            | `com.agentclientprotocol:acp` (Maven Central) | <https://github.com/agentclientprotocol/kotlin-sdk>       |
| Java                    | java-sdk (see repo README)                    | <https://github.com/agentclientprotocol/java-sdk>         |

Every SDK exposes both sides of the protocol (agent-side and client-side
connections), so the same package serves editor authors and agent authors.
Community libraries for other languages are indexed in
`references/libraries/community.md`.

## Relationship with MCP and A2A

These three protocols are complementary layers of an agent stack:

- **MCP** — agent reaches **down** to tools/resources. ACP reuses MCP's
  `ContentBlock`, and the Client hands its MCP server configs to the Agent at
  session setup (the editor can even expose itself as an MCP server via a
  small stdio proxy).
- **ACP** — the **UI layer**: editor reaches the agent on the user's behalf.
- **A2A** — agent reaches **across** to other agents as opaque peers.

The companion skills `mcp-knowledge` and `a2a-knowledge` in this repo cover
those protocols in equivalent depth.
