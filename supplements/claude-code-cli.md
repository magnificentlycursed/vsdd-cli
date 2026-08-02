---
schema_class: supplement
supplement_slug: claude-code-cli
languages_or_interfaces: [Claude Code CLI, Claude Agent SDK]
domains_in_scope: [ai-engineer, software-engineer, platform-engineer]
extensions: []
---

# Claude Code CLI Supplement

Per-domain extensions for projects that integrate with the [Claude Code CLI](https://code.claude.com/docs/en/) and/or the [Claude Agent SDK](https://code.claude.com/docs/en/agent-sdk/overview). Per the methodology's runtime-harness composition, vsdd-cli composes against this runtime harness as primitive.

## Activation

**Tier — runtime-determined always-on.** This supplement composes into the dispatch whenever the runtime is Claude Code — determined by the runtime the agent executes in, not by the task surface or the project's languages (the three-tier supplement-activation model). It is not task-gated; a dispatch under the Claude Code runtime that omits it is a conformance gap.

The current supplement frontmatter gates only by `languages_or_interfaces`; there is no runtime-determined-always-on field yet, so this note documents the model rather than enforcing it. Once the frontmatter field lands, the composition function (Slice 2) computes this supplement into the SHOULD unconditionally under the Claude Code runtime and the verifier gates on it. The frontmatter field, the mdatron schema change, and the composition-function wiring are the named Slice-2 cross-repo follow-on — out of scope for this prose edit.

## AI Engineer extensions

- **Harness run record as the measured ground truth.** Each dispatched agent writes an append-only transcript `agent-<id>.jsonl` recording per-message `usage` (`input_tokens` / `output_tokens` / `cache_creation_input_tokens` / `cache_read_input_tokens`), `tool_use` inputs, `model` (recorded per-agent), per-entry `durationMs` (the wall-clock / operator-time signal), and timestamps. `effort`'s presence is **dispatch-primitive-dependent**: it IS recorded in an Agent-tool sub-agent transcript (`"effort":"xhigh"` observed) but is absent from a crosslink-kickoff sub-agent record — so the reliable capture-source is the **dispatch manifest**, with the transcript a **sometimes-available backstop** (could-not-check only when the record lacks it, never a blanket absent). This record — which the checked agent cannot author — is the source for every cost/efficiency figure. There is no telemetry export, no exporter env var, and no external collector.
- **Delegation chain from the transcript turn tree.** The transcript's own `uuid` / `parentUuid` linkage — together with `sourceToolAssistantUUID`, `logicalParentUuid`, and the `isSidechain` marker — plus the `subagents/agent-<id>.jsonl` directory structure is the delegation-chain record: parent-to-sub-agent nesting reconstructs directly from the records. Delegation **depth** is not a recorded field; it is **derived by walking that graph** (a measured figure), not read off a `spawnDepth` key — no such field exists in the record. No trace-context header injection and no external trace backend.
- **Sub-agent delegation via Task tool.** Spawning sub-agents via the `Task` tool nests the child under the parent in the transcript turn tree (`parentUuid` / `sourceToolAssistantUUID`, with the child written to `subagents/agent-<id>.jsonl`); the cost-aware delegation pattern is judged from that record.
- **Skill invocation captured in `attributionSkill`.** A skill *invocation* is recorded in the transcript's `attributionSkill` field, distinct from a plain file `Read` (which appears only as a `Read` `tool_use` over the skill file). This is the record-backed capture-source for the skill-invocation audit: an invocation is the strong activation signal, a recorded `Read` of the same skill file is the weaker signal — the invoke-over-Read activation-strength distinction is visible in the record via `attributionSkill`.
- **Prompt-cache discipline.** 5-minute default TTL (API key); 1-hour TTL via `ENABLE_PROMPT_CACHING_1H=1` (paid more per write; longer cache reuse). Plan auth auto-enables 1-hour TTL.
- **Cost is a projection under subscription auth.** `total_cost_usd` + `modelUsage` from the SDK is a client-side estimate from a bundled price table, not authoritative billing; under subscription (Max/Pro) auth, dollars are a **projection** — the binding constraints are usage windows and operator time. The records carry usage tokens; the static price table (Slice 2) computes the projected bill. No Usage-API reconciliation.
- **Provenance tag.** Every cost-relevant figure carries a provenance tag — one of **recorded** (read verbatim from the run-record usage/tool events), **measured** (computed deterministically over recorded values or an actual file), **judgment** (a labeled right-sizing assessment), or **could-not-check** (the figure's source is not in the available record — tagged honestly, never fabricated). A figure with no provenance tag is rejected. (Naming: the tag is **provenance**, unified with the AI Engineer domain's term; *capture-source* refers only to which record field a datum was captured from.)

## Software Engineer extensions

- **`.claude/` runtime-harness directory.** Hooks at `.claude/hooks/`; slash commands at `.claude/commands/`; MCP servers at the repo-root `.mcp.json`; settings at `.claude/settings.json`.
- **Hook architecture.** Pure-Python hooks at `.claude/hooks/*.py`; vsdd-cli's Rust mirror subprocess from Python wrapper. One source; two enforcement surfaces.
- **Slash command discipline.** `.claude/commands/<name>.md` defines slash commands. VSDD-prefix discipline (`/vsdd-phase-3`, `/vsdd-domain-quality-engineer`) ensures no collision with crosslink's 14 commands.
- **MCP server integration.** The repo-root `.mcp.json` registers MCP servers — crosslink's knowledge server (`search_knowledge`) among them. vsdd ships **no** docs-server: `vsdd mcp-serve` is **cut** (superseded). Knowledge-surfacing rides crosslink's knowledge surface plus the agent's own `WebFetch` / `WebSearch` and the sibling tools' own docs surfaces (Claude Code docs, the Anthropic API docs), not a vsdd-hosted tool set.

## Platform Engineer extensions

- **Auth method per context.** Plan (Max/Pro) for operator-local skill mode; API key for CI/automation per Anthropic's guidance. Per the methodology's `auth_method.operator_local` + `auth_method.ci` separation.
- **`.claude/settings.json` discipline.** UNION-merge for `permissions.allow` / `permissions.ask` / `permissions.deny` (the deprecated `allowedTools` key is not taught — permission entries live under the `permissions` key); managed-section pattern for hooks. vsdd init composes with crosslink's existing entries.
- **Plan auth × CI rejection.** Per the methodology's cross-field validation, `auth_method.ci: plan` is structurally rejected (Plan requires operator-interactive session CI cannot provide).
- **Cron triggers + Notifications.** `CronCreate` for scheduled drift sweeps + session-close reminders; `PushNotification` + `RemoteTrigger` for budget breach + rate-limit headroom alerts.
- **Background tasks via `Bash run_in_background`.** CI-side compositions + long-running aggregations + cold-session dispatch primitives.
- **Permission modes.** Hook-bypass-marker enforcement at PR-time via Claude Code's permission modes.
