---
title: "Crosslink integration surfaces beyond the command line"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
---


## Design Specification

### 1. mcp servers

Crosslink ships **three MCP servers**, each a single-file Python script (PEP-723 inline metadata, run under `uv`, stdio JSON-RPC, MCP protocol `2024-11-05`, hand-rolled loop — no SDK dependency). The scripts are **embedded in the crosslink binary** via `include_str!` (`crosslink/src/commands/init/mod.rs:41-47`) from `crosslink/resources/claude/mcp/`, and `crosslink init` deploys them to the project's `.claude/mcp/` and registers them in `.mcp.json` via a **preserving merge** (`write_mcp_json_merged`: embedded keys are managed, custom server entries survive `init --force`). The embedded registration template is `crosslink/resources/mcp.json` — it lists exactly these three; there are **no shipped-but-unregistered servers**.

| Server | Tool(s) | Resources | Backend |
|---|---|---|---|
| `crosslink-safe-fetch` | `safe_fetch` | — | direct `httpx` |
| `crosslink-knowledge` | `search_knowledge` | `crosslink://knowledge/<slug>` | shells to `crosslink` CLI |
| `crosslink-agent-prompt` | `agent_prompt` | — | shells to `crosslink agent prompt` |

### crosslink-safe-fetch (`resources/claude/mcp/safe-fetch-server.py`, 286 lines)
- **Tool `safe_fetch`** — schema: `url` (string, required), `prompt` (string, optional, default "Extract the main content"; advisory only — the server does not use it for extraction, it returns the full sanitized body).
- Behavior: validates scheme (http/https only, host required); `httpx` GET with redirects, 30 s timeout, UA `Mozilla/5.0 (compatible; CrosslinkSafeFetch/1.0)`; then applies regex sanitization patterns loaded from **`.crosslink/rules/sanitize-patterns.txt`** (`pattern|||replacement` lines; found by walking up ≤10 dirs for `.crosslink/`) plus one hardcoded always-on pattern redacting `ANTHROPIC_MAGIC_STRING_TRIGGER_REFUSAL_*` → `[REDACTED_TRIGGER]`. Prepends a note with the sanitization count when >0.
- **Enforcement linkage** (the behavioral-guard mandate): `rules/global.md` line 80 — "Use `mcp__crosslink-safe-fetch__safe_fetch` for all web requests. Never use raw `WebFetch`" — and `rules/web.md` ("prefer safe_fetch over WebFetch when available") are injected by the UserPromptSubmit guard; separately the PreToolUse hook on `WebFetch|WebSearch` (`pre-web-check.py`) injects the RFIP external-content framing (from `web.md`, `rules.local` override honored) rather than denying the call, and blocks only fail-closed on unparseable stdin. Enforcement grade, honestly named: **injected convention/friction, not a block** — raw `WebFetch` still executes.

### crosslink-knowledge (`resources/claude/mcp/knowledge-server.py`, 302 lines)
- **Tool `search_knowledge`** — schema: `query` (string, required; case-insensitive substring), `tag` (string, optional), `since` (string, optional, YYYY-MM-DD). Runs `crosslink knowledge search <query> --json [--tag] [--since]`.
- **MCP resources** (the only server exposing them): `resources/list` maps `crosslink knowledge list --json` to `crosslink://knowledge/<slug>` URIs (`text/markdown`); `resources/read` serves `crosslink knowledge show <slug>`.
- Pure CLI adapter — 10 s subprocess timeout, no direct data access; the knowledge data model belongs to the sibling inventory.

### crosslink-agent-prompt (`resources/claude/mcp/agent-prompt-server.py`, 220 lines)
- **Tool `agent_prompt`** — schema: `session` (string, required; agent slug or tmux session name), `prompt` (string, required, multiline/any length), `submit` (boolean, default true). Wraps `crosslink agent prompt <session> <prompt> [--no-submit]` (tmux `load-buffer` + `paste-buffer` — no newline mangling, no length limits), 10 s timeout. This is the agent-to-agent prompt-delivery surface for tmux kickoff sessions.

**vsdd-cli binding**: all three are live — deployed at `.claude/mcp/`, registered in `.mcp.json`, and visible as `mcp__crosslink-*` tools in sessions. `sanitize-patterns.txt` is deployed under `.crosslink/rules/`. vsdd's contract consumes these as-is; nothing vsdd-specific extends them.

---

### 2. file-protocol surfaces (the kickoff/worktree contract)

The autonomous-execution loop communicates through **files in the agent worktree** (`<repo-root>/.worktrees/<slug>`, created by `kickoff run`; `mission control` and the swarm/status readers scan this directory). All nine kickoff files are added to the worktree's git exclude (`KICKOFF_EXCLUDE_PATTERNS`, `src/commands/kickoff/helpers.rs:400-410`). Documented upstream in `docs_src/reference/state-files.qmd` and `docs_src/reference/kickoff-report.qmd`.

### `.kickoff-status` — the sentinel state machine
- **Launcher writes** (`src/commands/kickoff/launch.rs:636-653`): `LAUNCHING` written **before the launch act**, then `RUNNING` on successful spawn or `FAILED` on spawn failure.
- **Agent writes** (instructed by the generated prompt): `DONE` as the very last step (after `crosslink sync` + `session end`); `CI_FAILED` after 5 failed CI fix-and-retry cycles (`prompt.rs:67`).
- **Readers**: `kickoff status`/`monitor`, `swarm status` (returns the raw sentinel string, else probes tmux/container liveness), the pipeline reconciler (`pipeline.rs::worktree_probe` + `reconcile_runs` — case-insensitive substring match: contains `done` → completed, `fail`/`error` → failed, worktree gone + no live agent → aborted, else left running), and the HTTP server's `agents`/`orchestrator poll_agents` handlers.
- **The missing-TIMEOUT gap = upstream #60** (verified open at basis time: "timeout kill never writes the TIMEOUT sentinel the harvest checks — killed agents remain classified RUNNING"). Timeout is *detected* live — `is_timed_out()` (`types.rs:379`) compares wall clock against `.kickoff-metadata.json` — but the kill path never persists a terminal sentinel, so a killed agent's worktree still says `RUNNING`.
- **vsdd contract binding**: the ratified never-started/stalled/dispatch-failed classification (`.design/agent-first-vsdd-toolkit.md`, the attended/autonomous amendment) binds *exactly* here — "the launch-status record written before the launch act" is the `LAUNCHING` pre-write; record present with no session activity = never started; record absent = launcher died pre-write; heartbeat staleness = stalled. #60 is why the classification cannot trust the sentinel alone for timed-out.

### The other kickoff files
| File | Written by | Carries |
|---|---|---|
| `KICKOFF.md` | `kickoff run` (`run.rs:117`) | The full generated agent prompt: issue/branch context, the rendered `## Design Specification` block from `--doc` (via `design_doc.rs:360`), verify-level instructions, final-steps protocol (sync → session end → `DONE`). A consumable artifact: `crosslink container start` executes `claude … "$(cat KICKOFF.md)"`. |
| `PLAN_KICKOFF.md` | `kickoff plan` (`plan.rs:187`) | The plan-mode (read-only gap-analysis) prompt; plan launches read it instead of KICKOFF.md. |
| `.kickoff-plan.json` | the plan agent | The structured gap report (the prompt dictates exact JSON shape); agent also copies it beside the design doc for discoverability; harvested by `plan.rs:331`. |
| `.kickoff-metadata.json` | `kickoff run` (`run.rs:140`) | `{started_at: ISO-8601, timeout_secs}` — the timeout budget (`KickoffMetadata`, `types.rs:52`). |
| `.kickoff-doc.json` | `kickoff run --doc` (`run.rs:312`) | `{rel_path, doc_hash: "sha256:<hex>"}` — the frozen-design-doc breadcrumb (GH#580); `monitor report/status` re-hashes and warns loudly if the agent rewrote its read-only input (`DocIntegrity`). |
| `.kickoff-criteria.json` | launch (from `--doc` acceptance criteria) | `{source_doc, extracted_at, criteria: [{id, type, …}]}` — machine-readable acceptance criteria (`CriteriaFile`). |
| `.kickoff-report.json` | the agent, second-to-last step before `DONE` | Structured completion report: `schema_version: 1`, `agent_id`, `issue_id`, `status: completed|failed|partial`, per-phase `PhaseTiming` metrics (duration, files read/modified, lines, tests, criteria), per-criterion verdicts with evidence, summary counts, `unresolved_questions`, `commits`, `files_changed`. Required: `validated_at`, `criteria`, `summary`. Harvested by `monitor.rs:824`; reference page `docs_src/reference/kickoff-report.qmd`. |
| `.kickoff-slug` | `kickoff run`/`plan` | The compact agent name (worktree ↔ agent-id join key). |

### Heartbeats, session records, `.crosslink/` state mirrors
- **Heartbeats**: the PostToolUse hook `heartbeat.py` fires on every tool call but invokes `crosslink heartbeat` at most every **120 s**, and only when `.crosslink/agent.json` exists (agent context). The heartbeat lands as `heartbeat.json` **at the root of the agent's own hub ref** (`refs/heads/crosslink/agents/<agent-id>`, hub v3, `hub_v3.rs:803`) and in the local `.crosslink/heartbeats/` cache; the HTTP server's fs watcher (`server/watcher.rs`) diffs that directory and broadcasts `heartbeat`/`agent_status` WebSocket events. Heartbeat staleness is the contract's "started-then-stalled" instrument.
- **`session.json`**: mirror of the current session row, rewritten by the daemon every 30 s so external tooling (statusline scripts, IDE plugins, the TUI) can read session state without opening SQLite. `{session_id, started_at, active_issue_id}`. Read-only by convention — the daemon overwrites it.
- **`locks.json`**: lives on the shared hub, cached at `.crosslink/.hub-cache/locks.json`; per-issue `{agent_id, branch, claimed_at, signed_by}` + `settings.stale_lock_timeout_minutes` (default 60) — the stale-lock/steal protocol's data surface.
- Smaller mirrors in vsdd-cli's live `.crosslink/`: `.active-issue`, `.last-hydrated-ref`, `.promoted-uuids`, `promotion-log.json`, `repo-id` (single-line opaque id, `E00r` here, used in compact agent IDs), `last_test_run`.
- The full state-file inventory is documented at `docs_src/reference/state-files.qmd`.

---

### 3. the rules-injection surface

`.crosslink/rules/` is the policy-as-context store; `crosslink init` deploys ~30 files from `resources/crosslink/rules/`: `global.md`, `project.md`, `knowledge.md`, `quality.md`, `rigor.md`, `web.md`, `tracking-{strict,normal,relaxed}.md`, `sanitize-patterns.txt`, and per-language files (c, cpp, csharp, elixir(+phoenix), go, java, javascript(+react), kotlin, odin, php, python, ruby, rust, scala, shell, swift, typescript(+react), zig).

- **`rules.local/`**: same-named files override `rules/` (checked first in every loader); survives `init --force`; the per-machine customization channel.
- **Assembly/injection** (`prompt-guard.py`, UserPromptSubmit): first prompt (or marker older than 4 h — marker at `.crosslink/.cache/guard-full-sent`) gets the **full `<crosslink-behavioral-guard>` block**: global + project + knowledge + quality rules, language sections selected by manifest detection, a project tree (depth 3, ≤50 entries), dependency list (≤30), and the active `tracking-<mode>.md`. Subsequent prompts get a short condensed reminder every **`reminder_drift_threshold`** prompts (hook-config key, default 3; 0 = every prompt) — this is the `reminder_drift` mechanism. A context-budget estimator can force full-guard **reinjection** plus a compression directive. Agent contexts (agent.json present) always get condensed-only.
- **Cross-surface links**: `sanitize-patterns.txt` feeds the safe-fetch MCP server's sanitizer; `web.md` is the RFIP text `pre-web-check.py` injects before raw WebFetch/WebSearch.
- **Configurable**: every file is plain hand-editable Markdown (or via `crosslink workflow`); tracking mode and drift threshold via `hook-config.json`. Reference: `docs_src/reference/rules.qmd`, `docs_src/guides/tracking-modes.qmd`.
- **vsdd binding**: this is the "rules" seam the vsdd contract names as an install target ("installed into crosslink's seams — rules, skills, hook config, house style"). vsdd-cli live state: the full default rule set is deployed; `rules.local/` exists but is **empty** — vsdd has not yet placed methodology context in this seam.

---

### 4. daemon + http/websocket server

Two distinct long-running components:

- **`crosslink daemon`** (`src/daemon.rs`): a detached background process (PID/log at `.crosslink/daemon.pid`/`daemon.log`), **not a network listener**. Every 30 s it hydrates the SQLite `issues.db` from the hub-v3 ref namespace (reduce checkpoint → hydrate) and rewrites the `session.json` mirror. It is the freshness engine behind every file-mirror consumer.
- **The web server** (`src/server/`, started by `crosslink dashboard serve`; the deprecated `crosslink serve` is the no-dashboard variant): axum on **`127.0.0.1:<port>` only**, bearer-token auth on all `/api/` routes (token printed/rotated at startup; `/api/v1/health` and `/ws` exempt), 10 MB body cap, CORS for the Vite dev server (:5173). Serves the React dashboard from an embedded `rust-embed` bundle (or `--dashboard-dir` for development).

**REST API** (`src/server/routes.rs`, all under `/api/v1`): agents (`/agents`, `/agents/{id}`, `/agents/{id}/status` — these read `.kickoff-status` from worktrees), locks (+ `/locks/stale`, `/locks/notify`), full issue CRUD with comments/labels/blockers and ready/blocked views, sessions (current/start/end/work), milestones, knowledge (list/get/create/search), unified `/search`, sync (status/fetch/push), config (GET/PATCH — validates `signing_enforcement` against `off|audit|warn|enforce`), token usage, and the **orchestrator handler**: `/orchestrator/plans`, `/decompose` (LLM-assisted document decomposition via the `claude` CLI), `/execute`, `/pause`, `/resume`, `/snapshot`, `/status`, `/agents/poll`, and per-stage `retry|skip|running|done|failed` transitions (`src/orchestrator/` = plan/DAG/executor with kickoff integration). Nested dashboard-only routers add project aggregation, a GitHub API bridge, export, webhooks, and a **PTY API** (REST + WebSocket) for embedded terminals.

**WebSocket hub** (`/ws`, `src/server/ws.rs`): broadcast events `Heartbeat`, `AgentStatus`, `IssueUpdated`, `LockChanged`, `ExecutionProgress`, `DashboardProjectUpdated`, `DashboardAlertsChanged`; fed by the heartbeat fs watcher and the handlers.

**Consumers today**: the bundled React dashboard (multi-project mission control) and its PTY terminals (spawned with `CROSSLINK_DASHBOARD=1` in the env). The VS Code extension (`vscode-extension/`, activates on `workspaceContains:.crosslink`) manages the daemon and shells the CLI rather than speaking HTTP. `crosslink mission control` is tmux-based, not a server consumer. **Is it consumer-facing?** Technically yes — localhost + bearer token, stable `/api/v1` JSON — but nothing in vsdd-cli consumes it: no `daemon.pid`, no `session.json` present in the live `.crosslink/`; vsdd's chosen integration is the subprocess CLI client (the un-designed #14 work). This whole class is an **unwired surface** for vsdd.

Related autonomous component: the **sentinel loop** (`src/commands/sentinel/`, PID/log `.crosslink/sentinel.pid|log`) — a poller that sources work (default source: GitHub issues labeled `agent-todo: replicate|fix`), dispatches kickoff agents (propagating `GH_TOKEN` for CI-verify dispatches), tracks a seen-set, and escalates failed attempts to a stronger model with cooldown/attempt caps. Fully configured via the `sentinel` block of `hook-config.json`; **disabled in vsdd-cli** (`sentinel.enabled: false`).

---

### 5. signing / trust material

The substrate the #815 corroboration keystone would build on:

- **`agent.json`** (`.crosslink/`, gitignored, per machine/worktree): agent identity — `agent_id` (`driver--<name>` or `<parent>--<slug>` for kickoff children), `machine_id`, `role` (`driver` owns a key; `agent` inherits the driver's), `ssh_key_path`, `ssh_fingerprint`, `ssh_public_key`. Written by `agent init` / `kickoff run`. vsdd-cli live: driver agent `xqjG` plus per-kickoff keypairs.
- **`keys/`**: generated Ed25519 SSH keypairs (`generate_agent_key`, `src/signing.rs:88`); worktree agents' keys are stored in the *host* repo's `.crosslink` (`host_crosslink_dir`). vsdd-cli live: the driver key plus four kickoff/plan agent keypairs (the Slice-3/Slice-4 dispatch evidence).
- **`driver-key.pub`**: cached driver public key for trust lookups.
- **The trust store**: `trust/allowed_signers` in the hub cache (SSH allowed-signers format). `trust approve` publishes an agent's public key and adds its entry (commit + push to the hub); `trust revoke` removes by principal; `trust pending` diffs `trust/keys/` against `allowed_signers`. `AllowedSigners::is_trusted(principal)` / `contains_key` are the verification primitives.
- **Signing paths**: git SSH commit signing configured per repo/worktree (`configure_git_ssh_signing`, worktree-config aware); detached content signing with SSH namespaces (`sign_content`/`verify_content`, `canonicalize_for_signing` for field-stable payloads). Lock claims carry `signed_by` fingerprints.
- **`signing_enforcement` modes**: `off | audit | warn | enforce` (validated by the server config handler; default `audit` in the shipped hook-config; vsdd-cli live: `audit`).
- **Per-agent hub refs**: every agent writes exclusively to `refs/heads/crosslink/agents/<agent-id>` (hub v3 — plain branches, browsable on any git host, always-fast-forward plumbing writes; legacy `refs/crosslink/*` namespace retained as constants for migration). The ref is simultaneously the identity anchor, the event log, and the heartbeat home.
- **What's verifiable by a consumer today**: commit signatures on per-agent refs against `allowed_signers` (grade: audit — recorded, not blocking), and key membership/fingerprint checks. Honestly noted: `SignatureVerification` now carries only the discriminant (`Valid|Unsigned|Invalid|NoCommits`) and its sole consumer is the **dashboard signature badge** — the richer v2 signing-enforcement report was retired with the v2 write path (#754, comment at `src/signing.rs:36`). A corroboration consumer would need to re-grow that reporting surface.
- Distinct "trust" namesake: **`swarm.toml`** (`trust-model init`) — review-triage priors (`local-only|multi-tenant|public-api|custom`, ignore patterns, trust boundaries) consumed by `swarm review`. Absent in vsdd-cli.

---

### 6. environment variables + config knobs + non-hook init deployments

**Env vars read by the binary** (source sweep):

| Var | Effect |
|---|---|
| `CROSSLINK_LOG` | log level (clap env for `--log`, default `warn`) |
| `CROSSLINK_LOG_FORMAT` | log format (clap env) |
| `CROSSLINK_BIN` | explicit binary path override, honored by dashboard-spawned subprocesses (`dashboard/projects.rs:738`) |
| `CROSSLINK_VERSION` | build-time version override (`option_env!`) |
| `CROSSLINK_FORCE_WORKTREE_ORPHAN_FALLBACK` | git-compat escape hatch (`git_compat.rs:22`) |
| `CROSSLINK_DASHBOARD=1` | set *by* the dashboard in PTY child envs — detectable marker |
| `GH_TOKEN` | propagated into sentinel CI-verify dispatches (resolved via `gh auth token` fallback); GitHub API bridge |
| `TMUX` | mission-control nesting detection |
| `CLAUDE_CODE` / `CLAUDECODE` | inside-Claude-Code detection (`design_cmd.rs`); kickoff *unsets* `CLAUDECODE` for child `claude` runs and sets `CLAUDE_CONFIG_DIR` |
| `HOSTNAME`/`COMPUTERNAME`/`USER`/… | `machine_id` defaulting |

**Config knobs** (`.crosslink/hook-config.json` + `hook-config.local.json` shallow-merge overlay, the latter gitignored and `init --force`-proof): `tracking_mode`, `intervention_tracking`, `cpitd_auto_install`, `comment_discipline`, `kickoff_verification`, `signing_enforcement`, `auto_steal_stale_locks`, `tracker_remote`, `blocked_git_commands`/`gated_git_commands`/`allowed_bash_prefixes`, `reminder_drift_threshold`, `agent_overrides` (relaxed tracking + narrower git blocks for agents), the `sentinel` block (interval, concurrency, sources, default agent, escalation ladder), `house_style` (style-sync source config, `src/commands/style.rs`; unset in vsdd-cli). Reference: `docs_src/reference/hook-config.qmd`.

**What `crosslink init` deploys that is not a hook/skill/command**: the three MCP servers + merged `.mcp.json`; merged `.claude/settings.json`; the shared hook library `.claude/hooks/crosslink_config.py`; the full `rules/` tree + `rules.local/`; `hook-config.json`; `.crosslink/.gitignore` plus a **managed marker section in the root `.gitignore`**; `repo-id`; and **`init-manifest.json`** — per-file `{sha256, written_by_version}` for every managed artifact, the drift-detection basis `init --force` uses to avoid clobbering user-modified files (and the direct ancestor of vsdd's installed-artifact-integrity discipline). Kickoff separately appends its nine file patterns to each worktree's git exclude.

---

### 7. container surface (one line, by design)

Agent containers run the GHCR image `ghcr.io/dollspace-gay/crosslink-agent:latest` (`src/commands/container.rs:43`) with a root entrypoint (`resources/container/entrypoint.sh`) that remaps the agent user to `HOST_UID`/`HOST_GID`, resolves Claude auth (Keychain-less macOS token handoff), and gosu-drops to the agent user to execute `claude` over the bind-mounted worktree's `KICKOFF.md` — full details live in the knowledge page **`attended-design-autonomous-execution`** and `docs_src/guides/container-agents.qmd`.

---

### upstream doc index (docs_src/)

Guides: `hooks`, `kickoff`, `knowledge`, `multi-agent`, `swarm`, `session-workflow`, `tracking-modes`, `tui`, `web-dashboard`, `container-agents`, `design-workflow`, `maintenance`. Reference: `commands`, `hook-config`, `kickoff-report`, `rules`, `state-files`.

### the most design-relevant unwired surfaces (vsdd-cli, at basis)

1. **The localhost REST/WS server + orchestrator API** — stage-lifecycle transitions, `agents/poll`, decompose/execute, WebSocket progress events; nothing in vsdd consumes it (the daemon isn't even running here), yet it is the only machine-readable *push* channel for exactly the run-state vsdd's Status/gate designs re-derive from files.
2. **The sentinel loop** (`sentinel.enabled: false`) — issue-sourced autonomous dispatch with an escalation ladder; the closest existing mechanism to the phases-dispatched keystone (#840) and never evaluated for it.
3. **`swarm.toml` trust-model config** (absent) — the triage-prior surface `swarm review` consumes; Slice 6 binds `crosslink swarm review` as the phase-3 exit act, so its priors file is a direct, currently-unauthored vsdd input. (Runner-up: the `house_style` seam, named in the vsdd contract as an install target, unset.)

Known gaps carried into vsdd designs: upstream **#60** (timeout kill never writes a terminal sentinel — RUNNING lies) and **#61** (swarm launch surface lacks the effort dial).

