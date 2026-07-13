//! Tool definitions for [`Server`].
//!
//! All `#[tool]`-annotated methods live in a single `#[tool_router] impl
//! Server` block so that the macro can generate one `Server::tool_router()`
//! associated function.

// `CreateMessageRequestParams` / `SamplingMessage` are SEP-2577-deprecated;
// `ask_llm` below still demonstrates sampling, so importing them is expected.
#[allow(
    deprecated,
    reason = "SEP-2577 deprecates sampling; kept as an example"
)]
use rmcp::{
    ErrorData as McpError, RoleServer,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock, CreateMessageRequestParams, SamplingMessage},
    schemars,
    service::{ElicitationError, RequestContext},
    tool, tool_router,
};
use serde::{Deserialize, Serialize};

use crate::Server;

/// Arguments for the `slow_count` task-capable tool.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SlowCountArgs {
    /// How high to count. Each tick sleeps for [`SLOW_COUNT_TICK_MS`].
    pub target: u8,
}

/// Sleep duration per `slow_count` tick. Kept small (100ms) so a synchronous
/// call from a client that does not opt in to the task path (e.g. MCP
/// Inspector) still completes within typical request timeouts.
pub const SLOW_COUNT_TICK_MS: u64 = 100;

/// Arguments for the `ask_llm` (sampling) tool.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct AskLlmArgs {
    /// The question to ask the connected client's LLM.
    pub question: String,
}

/// Shape elicited from the user by the `greet_user` tool.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct UserNameInput {
    /// The user's preferred display name.
    pub name: String,
}

rmcp::elicit_safe!(UserNameInput);

#[tool_router(vis = "pub(crate)")]
impl Server {
    /// Example tool. Replace with real tools.
    #[tool(
        description = "Health-check tool. Returns 'pong'.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    async fn ping(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![ContentBlock::text("pong")]))
    }

    /// Example task-capable tool. Sleeps `target * SLOW_COUNT_TICK_MS` and
    /// returns the final count. With `task_support = "optional"`, the client
    /// may call this either synchronously or as an async task; the task path
    /// uses the `OperationProcessor` on `self.processor`.
    #[tool(
        description = "Count up to `target` slowly (100ms per tick). Supports task-based invocation.",
        execution(task_support = "optional"),
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    async fn slow_count(
        &self,
        Parameters(args): Parameters<SlowCountArgs>,
    ) -> Result<CallToolResult, McpError> {
        for i in 1..=args.target {
            tokio::time::sleep(std::time::Duration::from_millis(SLOW_COUNT_TICK_MS)).await;
            tracing::debug!(tick = i, target = args.target, "slow_count tick");
        }
        Ok(CallToolResult::success(vec![ContentBlock::text(
            args.target.to_string(),
        )]))
    }

    /// Sampling example: ask the connected client's LLM to answer a question.
    ///
    /// Requires a client that supports `sampling/createMessage` (e.g. Claude
    /// Desktop). Inspector will respond with an error if sampling isn't
    /// available.
    ///
    /// Sampling is deprecated by SEP-2577 as of rmcp 2.0 but remains part of
    /// the protocol; this example keeps demonstrating it.
    #[allow(
        deprecated,
        reason = "SEP-2577 deprecates sampling; kept as an example"
    )]
    #[tool(
        description = "Ask the client's LLM a question via sampling.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = true,
        )
    )]
    async fn ask_llm(
        &self,
        Parameters(args): Parameters<AskLlmArgs>,
        ctx: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let response = ctx
            .peer
            .create_message(
                CreateMessageRequestParams::new(
                    vec![SamplingMessage::user_text(&args.question)],
                    512,
                )
                .with_system_prompt("You are a concise assistant.")
                .with_temperature(0.7),
            )
            .await
            .map_err(|e| McpError::internal_error(format!("sampling request failed: {e}"), None))?;

        let text = match response.message.content.iter().find_map(|c| c.as_text()) {
            Some(t) => t.text.clone(),
            None => {
                // Image/audio/tool_use/tool_result responses aren't handled
                // by this skeleton — surface a warning so anyone copying this
                // tool sees they need to handle multimodal sampling content.
                tracing::warn!(
                    model = %response.model,
                    "ask_llm: sampling response contained no text content; \
                     non-text content blocks are not handled by this skeleton",
                );
                "(no text response)".to_string()
            }
        };
        Ok(CallToolResult::success(vec![ContentBlock::text(text)]))
    }

    /// Elicitation example: ask the user (via the client) for their name,
    /// then greet them. Requires a client that supports
    /// `elicitation/create`.
    ///
    /// Decline/cancel and "client does not support elicitation" are *user
    /// actions* or *capability mismatches*, not service failures — surface
    /// them as `CallToolResult::success` with an informative message so the
    /// client can render them naturally. Only genuine protocol/parse
    /// failures escalate to `internal_error`.
    #[tool(
        description = "Ask the user for their name, then greet them.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false,
        )
    )]
    async fn greet_user(
        &self,
        ctx: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        match ctx
            .peer
            .elicit::<UserNameInput>("Please enter your name")
            .await
        {
            Ok(Some(input)) => Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                "Hello, {}!",
                input.name
            ))])),
            Ok(None) => Ok(CallToolResult::success(vec![ContentBlock::text(
                "Greeting skipped — no name was provided.",
            )])),
            Err(ElicitationError::UserDeclined) => {
                Ok(CallToolResult::success(vec![ContentBlock::text(
                    "Greeting skipped — the user declined to share their name.",
                )]))
            }
            Err(ElicitationError::UserCancelled) => {
                Ok(CallToolResult::success(vec![ContentBlock::text(
                    "Greeting cancelled — the user dismissed the prompt.",
                )]))
            }
            Err(ElicitationError::CapabilityNotSupported) => {
                Ok(CallToolResult::success(vec![ContentBlock::text(
                    "This client does not support elicitation, so the user could not be \
                     prompted for a name.",
                )]))
            }
            Err(e) => Err(McpError::internal_error(
                format!("elicitation failed: {e}"),
                None,
            )),
        }
    }

    /// Roots example: query the client for the filesystem/workspace roots
    /// it exposes, and return a summary. Requires a client that supports
    /// `roots/list` (e.g. an IDE-integrated MCP client).
    ///
    /// Roots is deprecated by SEP-2577 as of rmcp 2.0 but remains part of
    /// the protocol; this example keeps demonstrating it.
    #[allow(deprecated, reason = "SEP-2577 deprecates roots; kept as an example")]
    #[tool(
        description = "List the workspace roots the client has exposed.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    async fn list_workspace_roots(
        &self,
        ctx: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let result = ctx.peer.list_roots().await.map_err(|e| {
            McpError::internal_error(format!("roots/list request failed: {e}"), None)
        })?;

        let text = if result.roots.is_empty() {
            "Client exposed no roots.".to_string()
        } else {
            result
                .roots
                .iter()
                .map(|root| match &root.name {
                    Some(name) => format!("- {} ({})", name, root.uri),
                    None => format!("- {}", root.uri),
                })
                .collect::<Vec<_>>()
                .join("\n")
        };
        Ok(CallToolResult::success(vec![ContentBlock::text(text)]))
    }
}
