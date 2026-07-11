#!/usr/bin/env bash
# Claude Code hook (PostToolUse): warn when a CLAUDE.md memory file grows past
# the recommended line budget, and nudge toward splitting it into path-scoped
# .claude/rules/ files.
#
# Reads the PostToolUse payload on stdin, inspects `.tool_input.file_path`, and
# acts only on CLAUDE.md / CLAUDE.local.md files. Every other path is a clean
# no-op, as is a missing `jq`. Emits `systemMessage` (shown to the user) and
# `additionalContext` (a nudge for Claude) only when the file is oversized.
#
# Override the threshold with CLAUDE_MD_MAX_LINES (default 200).

set -u

max_lines="${CLAUDE_MD_MAX_LINES:-200}"

command -v jq >/dev/null 2>&1 || exit 0

input="$(cat)"
file_path="$(jq -r '.tool_input.file_path // empty' <<<"$input")"
[[ -n "$file_path" ]] || exit 0

case "$(basename "$file_path")" in
  CLAUDE.md | CLAUDE.local.md) ;;
  *) exit 0 ;;
esac

[[ -f "$file_path" ]] || exit 0

lines="$(grep -c '' "$file_path" 2>/dev/null || echo 0)"
[[ "$lines" =~ ^[0-9]+$ ]] || exit 0
((lines > max_lines)) || exit 0

base="$(basename "$file_path")"
warn="${base} is now ${lines} lines (recommended <= ${max_lines}). Large memory files dilute instruction adherence — consider extracting topic- or path-specific sections into .claude/rules/*.md."
ctx="The memory file just edited (${file_path}) is ${lines} lines, over the recommended ${max_lines}-line budget. When it is a natural moment, offer to split topic- or path-specific sections into path-scoped .claude/rules/<topic>.md files (each with a paths: glob), or to run the claude-md-improver skill. Do not refactor memory without the user's go-ahead."

jq -n --arg warn "$warn" --arg ctx "$ctx" '{
  systemMessage: $warn,
  suppressOutput: true,
  hookSpecificOutput: {
    hookEventName: "PostToolUse",
    additionalContext: $ctx
  }
}'
exit 0
