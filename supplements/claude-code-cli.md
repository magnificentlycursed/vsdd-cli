---
supplement_slug: claude-code-cli
languages_or_interfaces: [Claude Code CLI, Claude Agent SDK]
domains_in_scope: [ai-engineer, software-engineer, platform-engineer]
extensions: []
---

# Claude Code CLI Supplement

Per-domain extensions for projects that integrate with the [Claude Code CLI](https://code.claude.com/docs/en/) and/or the [Claude Agent SDK](https://code.claude.com/docs/en/agent-sdk/overview). Per the methodology's substrate composition, vsdd-cli composes against this substrate as primitive.

## AI Engineer extensions

- **Agent SDK as observability primitive.** `CLAUDE_CODE_ENABLE_TELEMETRY=1` + per-signal exporter env vars; Agent SDK emits metrics + log events + traces to any OTLP-compatible backend.
- **W3C trace context propagation.** SDK auto-injects TRACEPARENT into CLI subprocess + Bash/PowerShell tool calls; full delegation chain visible as single trace.
- **Sub-agent delegation via Task tool.** Spawning sub-agents via `Task` tool nests under parent's span tree; cost-aware delegation pattern.
- **Prompt-cache discipline.** 5-minute default TTL (API key); 1-hour TTL via `ENABLE_PROMPT_CACHING_1H=1` (paid more per write; longer cache reuse). Plan auth auto-enables 1-hour TTL.
- **SDK message stream cost.** `total_cost_usd` + `modelUsage` from SDK is client-side estimate from bundled price table — not authoritative billing. v1 ships estimate; v1+ reconciles with Anthropic Usage API.
- **Capture-source provenance.** Every cost-relevant event carries `capture_source` enum (otel-metric / otel-log-event / otel-trace-attribute / vsdd-custom-event / sdk-result-message / usage-api-reconciled / unmeasurable).

## Software Engineer extensions

- **`.claude/` substrate directory.** Hooks at `.claude/hooks/`; slash commands at `.claude/commands/`; MCP servers at `.claude/mcp.json`; settings at `.claude/settings.json`.
- **Hook architecture.** Pure-Python hooks at `.claude/hooks/*.py`; vsdd-cli's Rust mirror subprocess from Python wrapper. One source; two enforcement surfaces.
- **Slash command discipline.** `.claude/commands/<name>.md` defines slash commands. VSDD-prefix discipline (`/vsdd-phase-3`, `/vsdd-domain-quality-engineer`) ensures no collision with crosslink's 14 commands.
- **MCP server integration.** `.claude/mcp.json` registers MCP servers; vsdd-cli registers `vsdd mcp-serve` exposing 4 tools (methodology.lookup, claude_code.docs.search, crosslink.docs.search, anthropic.api.docs.search).

## Platform Engineer extensions

- **Auth method per context.** Plan (Max/Pro) for operator-local skill mode; API key for CI/automation per Anthropic's guidance. Per the methodology's `auth_method.operator_local` + `auth_method.ci` separation.
- **`.claude/settings.json` discipline.** UNION-merge for `allowedTools`; managed-section pattern for hooks. vsdd init composes with crosslink's existing entries.
- **Plan auth × CI rejection.** Per the methodology's cross-field validation, `auth_method.ci: plan` is structurally rejected (Plan requires operator-interactive session CI cannot provide).
- **Cron triggers + Notifications.** `CronCreate` for scheduled drift sweeps + cycle-close reminders; `PushNotification` + `RemoteTrigger` for budget breach + rate-limit headroom alerts.
- **Background tasks via `Bash run_in_background`.** CI-side compositions + long-running aggregations + cold-session dispatch primitives.
- **Permission modes.** Hook-bypass-marker enforcement at PR-time via Claude Code's permission modes.
