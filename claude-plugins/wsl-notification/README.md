# wsl-notification

A Claude Code plugin that surfaces `Notification` and `Stop` hook events as
Windows toast notifications when Claude Code is running inside **WSL2**.

The hook reads the JSON payload Claude Code sends on stdin, extracts
`.message`, and dispatches a toast via `powershell.exe`.

- **`Notification`** is scoped to the `permission_prompt` matcher, so it fires
  the moment Claude needs permission to run a tool. The toast body is the
  event's `.message`. Other notification types (`idle_prompt`, `auth_success`,
  …) are intentionally not surfaced, so there is no idle-timer toast.
- **`Stop`** fires whenever Claude finishes responding. Its payload carries no
  `.message`, so a fixed `"Claude finished responding."` body is used. This
  still works under bypass-permissions mode, where `Notification` permission
  events never fire.

## Requirements

- WSL2 on Windows. The hook detects non-WSL environments via `/proc/version`
  and exits cleanly, so it is safe to install on systems where it cannot run —
  but it will only produce notifications inside WSL2.
- `powershell.exe` reachable on `PATH` (default on WSL2).
- `jq` installed inside the WSL2 distribution.

## Install

This plugin is distributed through the `46ki75-plugins` marketplace:

```bash
/plugin marketplace add 46ki75/claude-plugins
/plugin install wsl-notification@46ki75-plugins
```

## What it does

The plugin registers a `Notification` hook (scoped to `permission_prompt`) and
a `Stop` hook:

```json
{
  "hooks": {
    "Notification": [
      {
        "matcher": "permission_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/notify.sh Notification"
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/notify.sh Stop"
          }
        ]
      }
    ]
  }
}
```

`${CLAUDE_PLUGIN_ROOT}` is expanded by Claude Code to this plugin's install
directory at runtime.

## Notification matchers

`Notification` hooks filter on the notification type. Claude Code documents six
matcher values — `permission_prompt`, `idle_prompt`, `auth_success`,
`elicitation_dialog`, `elicitation_complete`, and `elicitation_response` (see
the official hooks reference at <https://code.claude.com/docs/en/hooks.md>).

This plugin scopes to `permission_prompt` only. To also toast idle waits — the
state that backstops mid-turn `AskUserQuestion`-style prompts — add a second
matcher block alongside the existing one:

```json
{ "matcher": "idle_prompt", "hooks": [ /* same notify.sh Notification call */ ] }
```

`idle_prompt` fires once Claude has been waiting on you for the idle threshold
(default **60000 ms**), emitting the message `"Claude is waiting for your
input"`. That threshold (`messageIdleNotifThresholdMs`) is an internal default
in the Claude Code app-state config (`~/.claude.json`); it is **not** a
`settings.json` option and has no supported override.

> The field the matcher checks (`notification_type`) and the 60000 ms idle
> default are not in the public docs. Both were verified directly against the
> shipped Claude Code binary (v2.1.183) — the authoritative source when the
> reference is silent.

## Layout

```text
wsl-notification/
├── .claude-plugin/
│   └── plugin.json
├── hooks/
│   └── hooks.json
└── scripts/
    └── notify.sh
```

## License

MIT.
