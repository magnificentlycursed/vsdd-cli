---
title: "Crosslink integration surfaces beyond the command line"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-10-03
---

## Design Specification

### currency (read this first; 2026-10-02)

Updated under vsdd-cli#888 against the crosslink fork tree at `ddc0cbe57` (installed binary `0.9.0-beta.1+973e395dc`), read from source and not executed. Paths in the re-verified sections are relative to the crate (`crosslink/` in the crosslink repo).

- **Re-verified and rewritten on 2026-10-02:** section 1 (MCP servers), section 3 (the rules-injection surface), and the new sections 8 (the hook payload) and 9 (what a project can ship, and what init does to it).
- **Not re-verified; still the 2026-08-02 text:** sections 2, 4, 5, 6, 7 and the two closing lists. Treat their file paths and line numbers as stale. Known-wrong statements in them:
  - Section 2: the kickoff files and the status state machine are now covered, current, on `kickoff-swarm-dispatch-pipeline`. The "missing TIMEOUT" gap (crosslink#60) is fixed: a timeout now writes `TIMEOUT`. The launch is headless, not an interactive `claude … "$(cat KICKOFF.md)"`.
  - Section 4: the daemon is no longer only a 30-second hydrator. Since the readiness model it establishes per-checkout repository readiness, and the session-start hook runs `daemon ensure --wait-ready`. The statement that no daemon runs in vsdd-cli is false since vsdd-cli PR #46. See `container-vehicle-pilot-2026-09`.
  - Section 6: the shared hook library is at `.crosslink/integrations/hooks/crosslink_config.py`, not `.claude/hooks/`. `init` no longer deploys rule content (every bundled rule file is zero bytes). The hook-config keys the hooks actually read are listed in section 8.
  - Section 7: the upstream agent image is private (crosslink#101); this estate uses the fork's mirror.
  - "The most design-relevant unwired surfaces": `rules.local/` is not a channel a project can ship (it is gitignored), and the swarm trust-model file feeds a review command that launches nothing.

---

### 1. mcp servers

**Correction 2026-10-02, evening.** Statements on this page about vsdd-cli's own rule files and hook wiring were read from a checkout that was behind main. vsdd-cli PR #50 (merged 2026-10-01, operator decision "no custom crosslink setup") returned `.claude/settings.json` to crosslink's stock wrapper (fail-open; the #658 fail-closed guard is gone) and emptied 29 of the 30 rule files, keeping only `project.md` (about 4 KB). So on main today: the only rule content crosslink's prompt hook delivers here is `project.md`; nothing in `.crosslink/rules/` carries Rust guidance; the hook's block is a few KB, not 23 KB; and a kickoff worktree's init replaces stock wiring with stock wiring. The live-test and mechanism findings stand; the vsdd-cli-specific sizes and the "fail-closed replaced by fail-open" observation do not.


Re-verified 2026-10-02.

Crosslink ships **two MCP servers**, each a single-file, standard-library-only Python script. The sources are `resources/agent/mcp/`; `crosslink init` regenerates them from the binary into the project's **`.crosslink/integrations/mcp/`** (machine-local, gitignored) and registers them in the tracked `.mcp.json`, run under `python3`. The merge preserves a project's own server entries, overwrites crosslink's two, and removes the retired `crosslink-safe-fetch` key (`src/commands/init/merge.rs:95-153`).

| Server | Tool | Resources | Backend |
|---|---|---|---|
| `crosslink-knowledge` | `search_knowledge(query, tag?, since?)` | `crosslink://knowledge/<slug>` (list and read) | shells to the `crosslink` CLI, 10 s timeout; read-only |
| `crosslink-agent-prompt` | `agent_prompt(session, prompt, submit=true)` | none | shells to `crosslink agent prompt`, which pastes text into a running tmux agent session |

- **The knowledge server delivers nothing by itself.** An agent has to call it. It is retrieval by the agent's judgment.
- **The agent-prompt server is a driver-to-agent channel** for local tmux agents. Whether a headless agent reads pasted keystrokes was not determined; kickoff now launches headless.
- **The safe-fetch server is retired upstream.** Web requests use the built-in tools; the `pre-web-check.py` hook prints a fixed provenance notice before `WebFetch` and `WebSearch` and does not block. `sanitize-patterns.txt` has no consumer.
- **vsdd-cli binding:** both servers are live and appear as `mcp__crosslink-*` tools. Nothing vsdd-specific extends them. The per-developer `.claude/settings.local.json` in the main checkout still enables the retired safe-fetch server (noted on vsdd-cli PR #51).

---

### 2. file-protocol surfaces (the kickoff/worktree contract)

> Not re-verified on 2026-10-02; see the currency note at the top.

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

Re-verified 2026-10-02 against the deployed `prompt-guard.py`, which is byte-identical to `resources/agent/hooks/prompt-guard.py`.

`.crosslink/rules/` is a tracked folder of Markdown files that crosslink's prompt hook reads and prints into the session's context. It is the one mechanical delivery slot crosslink gives a project for interactive sessions.

- **Only a fixed set of filenames is emitted.**
  - Always: `global.md` (with `external-content.md` appended), `project.md` (under "Project-Specific Rules"), `knowledge.md`, `quality.md` (`prompt-guard.py:75-78`).
  - The active `tracking-<mode>.md`, chosen by `tracking_mode` (`:532-553`).
  - The file for each detected language, from a fixed map of 22 filenames such as `rust.md` (`:86-98`).
- **Any other file is read and never emitted.** A file with another name is stored under a "language" derived from its filename and then filtered against the detected languages, so it never matches (`:136-140, 232-237`). This applies to a new file such as `vsdd-rust.md`, and to the deployed `rigor.md` and `web.md`, which are dead content today.
- **Language detection is per project, not per file being edited.** It looks for marker files (`Cargo.toml` and similar) in the root and first-level subdirectories, and for file extensions in the root, `src/` and `*/src/` (`:146-229`).
- **`rules.local/<same name>` replaces the tracked file; it does not extend it** (`:37-50`). It is gitignored, so it is machine-local and absent in kickoff worktrees.
- **There is no size limit and no truncation.**
- **Cadence in the main checkout:**
  - the full block (project tree, dependencies, all rules) when the marker `.crosslink/.cache/guard-full-sent` is missing or older than 4 hours (`:501-515`); the marker is per checkout, not per session;
  - otherwise only when the prompt counter is a multiple of `reminder_drift_threshold` (3 in vsdd-cli; 0 means every prompt) (`:657-664`). This "condensed" block drops the tree and dependencies but **still carries every rule in full** (`:557-577`): about 23 KB, roughly 5.8k tokens, on the pre-PR-#50 tree this was read from; a few KB on main today;
  - a full re-injection when an estimated `context_budget_chars` is reached (default 1,000,000) (`:597-612`). That key is a re-injection trigger, not a cap.
- **In an agent context the condensed block is sent on every call** (`:634-637`). Agent context means `agent.json` with the role "agent", or a working directory under `/.claude/worktrees/` or `/.codex/worktrees/`.
- **The block is named `<crosslink-project-context>`.** The hook never blocks.
- **Subagents:** the same hook is wired to subagent start with no event-specific branch, so it follows the same counter and usually emits nothing. One research agent, itself a subagent in this checkout, found no rules block in its own context. Whether Claude Code feeds that hook's plain output to a subagent was not determined.
- **Upstream ships every rule file empty** (commit `62e637ab7`, 2026-08-16, "zero bundled rules", no explanation given). Crosslink's `preflight` skill says to confirm the rule files "remain zero bytes". Crosslink's own Rust guidance now ships as two skills. vsdd-cli kept 30 customised rule files until PR #50 (2026-10-01) emptied 29 of them; only `project.md` carries content now.
- **What the custom marker does and does not do.** `crosslink workflow diff --check` does not report a marked file as drift (`src/commands/workflow.rs:43`), and `crosslink style sync` skips a marked file (`src/commands/style.rs:150-176`). `init --update` ignores the marker and classifies by manifest hash: vsdd-cli's files count as conflicts, which a non-interactive update keeps and an interactive "yes" blanks. `init --force` rewrites every managed rule name with the empty template (`src/commands/init/mod.rs:1200-1208`).
- **vsdd binding:** the rules carry crosslink-usage and project policy. They do not carry the supplements, and the 2026-10-02 review advised against adding supplements here. See `content-delivery-assessment-2026-10-02`.

---

### 4. daemon + http/websocket server

> Not re-verified on 2026-10-02; see the currency note at the top.

Two distinct long-running components:

- **`crosslink daemon`** (`src/daemon.rs`): a detached background process (PID/log at `.crosslink/daemon.pid`/`daemon.log`), **not a network listener**. Every 30 s it hydrates the SQLite `issues.db` from the hub-v3 ref namespace (reduce checkpoint → hydrate) and rewrites the `session.json` mirror. It is the freshness engine behind every file-mirror consumer.
- **The web server** (`src/server/`, started by `crosslink dashboard serve`; the deprecated `crosslink serve` is the no-dashboard variant): axum on **`127.0.0.1:<port>` only**, bearer-token auth on all `/api/` routes (token printed/rotated at startup; `/api/v1/health` and `/ws` exempt), 10 MB body cap, CORS for the Vite dev server (:5173). Serves the React dashboard from an embedded `rust-embed` bundle (or `--dashboard-dir` for development).

**REST API** (`src/server/routes.rs`, all under `/api/v1`): agents (`/agents`, `/agents/{id}`, `/agents/{id}/status` — these read `.kickoff-status` from worktrees), locks (+ `/locks/stale`, `/locks/notify`), full issue CRUD with comments/labels/blockers and ready/blocked views, sessions (current/start/end/work), milestones, knowledge (list/get/create/search), unified `/search`, sync (status/fetch/push), config (GET/PATCH — validates `signing_enforcement` against `off|audit|warn|enforce`), token usage, and the **orchestrator handler**: `/orchestrator/plans`, `/decompose` (LLM-assisted document decomposition via the `claude` CLI), `/execute`, `/pause`, `/resume`, `/snapshot`, `/status`, `/agents/poll`, and per-stage `retry|skip|running|done|failed` transitions (`src/orchestrator/` = plan/DAG/executor with kickoff integration). Nested dashboard-only routers add project aggregation, a GitHub API bridge, export, webhooks, and a **PTY API** (REST + WebSocket) for embedded terminals.

**WebSocket hub** (`/ws`, `src/server/ws.rs`): broadcast events `Heartbeat`, `AgentStatus`, `IssueUpdated`, `LockChanged`, `ExecutionProgress`, `DashboardProjectUpdated`, `DashboardAlertsChanged`; fed by the heartbeat fs watcher and the handlers.

**Consumers today**: the bundled React dashboard (multi-project mission control) and its PTY terminals (spawned with `CROSSLINK_DASHBOARD=1` in the env). The VS Code extension (`vscode-extension/`, activates on `workspaceContains:.crosslink`) manages the daemon and shells the CLI rather than speaking HTTP. `crosslink mission control` is tmux-based, not a server consumer. **Is it consumer-facing?** Technically yes — localhost + bearer token, stable `/api/v1` JSON — but nothing in vsdd-cli consumes it: no `daemon.pid`, no `session.json` present in the live `.crosslink/`; vsdd's chosen integration is the subprocess CLI client (the un-designed #14 work). This whole class is an **unwired surface** for vsdd.

Related autonomous component: the **sentinel loop** (`src/commands/sentinel/`, PID/log `.crosslink/sentinel.pid|log`) — a poller that sources work (default source: GitHub issues labeled `agent-todo: replicate|fix`), dispatches kickoff agents (propagating `GH_TOKEN` for CI-verify dispatches), tracks a seen-set, and escalates failed attempts to a stronger model with cooldown/attempt caps. Fully configured via the `sentinel` block of `hook-config.json`; **disabled in vsdd-cli** (`sentinel.enabled: false`).

---

### 5. signing / trust material

> Not re-verified on 2026-10-02; see the currency note at the top.

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

> Not re-verified on 2026-10-02; see the currency note at the top.

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

> Not re-verified on 2026-10-02; see the currency note at the top.

Agent containers run the GHCR image `ghcr.io/dollspace-gay/crosslink-agent:latest` (`src/commands/container.rs:43`) with a root entrypoint (`resources/container/entrypoint.sh`) that remaps the agent user to `HOST_UID`/`HOST_GID`, resolves Claude auth (Keychain-less macOS token handoff), and gosu-drops to the agent user to execute `claude` over the bind-mounted worktree's `KICKOFF.md` — full details live in the knowledge page **`attended-design-autonomous-execution`** and `docs_src/guides/container-agents.qmd`.

---

### 8. the hook payload (readiness-era layout)

Verified 2026-10-02. The deployed scripts in `.crosslink/integrations/hooks/` are byte-identical to `resources/agent/hooks/`. They are machine-local, gitignored, hash-tracked in the init manifest, and overwritten by `init --update` when unmodified. **A project cannot ship edits to them.** The wiring is in the tracked `.claude/settings.json`.

| Hook | Claude Code event | What it does | Can it block |
|---|---|---|---|
| `session-start.py` | Session start: startup, resume, clear, compact | Runs `crosslink daemon ensure --wait-ready`; prints one `<crosslink-session-context>` block: the provenance notice and `external-content.md`, a stale-session warning, the last handoff, session status and last action, agent identity, sync output and locks, the knowledge page count, open issues, a workflow reminder; and, **when the active issue carries `design-doc:<slug>` labels, up to 3 knowledge pages of at most 8,000 characters each** (`session-start.py:194-255`) | Exits 2 when readiness fails |
| `prompt-guard.py` | Each user prompt; subagent start | Prints the rules block (section 3) | No |
| `work-check.py` | Before `Write`, `Edit`, `Bash` | Blocks on: repository not ready; kill or pause flags; blocked git commands; `git commit` without an active issue; a missing plan or result comment when `comment_discipline` is "required"; strict tracking with no active issue | Yes |
| `post-edit-check.py` | After `Write`, `Edit` | Stub-pattern findings, linter output and a test reminder, capped at 12,000 characters. The extension-to-linter table is hard-coded; other extensions exit silently | No |
| `pre-web-check.py` | Before `WebFetch`, `WebSearch` | A fixed provenance notice | No |
| `heartbeat.py` | After every tool call | Runs `crosslink heartbeat` at most every 120 s | No |

- **No hook has a per-file-type or per-path trigger a project can configure, and none gates an edit on a file having been read.**
- **The only project-controlled additions to session start** are `external-content.md` and the `design-doc:<slug>` label. Nothing applies that label automatically; `knowledge add --from-doc` adds a `design-doc` *tag*, which is a different thing.
- **Keys the hooks read from `hook-config.json`:** `tracking_mode`; `blocked_git_commands`, `gated_git_commands`, `allowed_bash_prefixes`, `comment_discipline`; `agent_overrides` (its own tracking mode, block list, gate list, comment discipline, and lint and test commands that only extend the allowed-command list); `reminder_drift_threshold`, `context_budget_chars`; `crosslink_binary`. No key adds context files, injected text or commands to run.
- **`hook-config.local.json` is merged over the tracked file**; a key prefixed with `+` appends to a list. It is gitignored, so it can change the tracking mode and the block lists invisibly.
- **In agent context the hook takes `agent_overrides`.** vsdd-cli's tracked `agent_overrides` are looser than upstream's shipped default (recorded on vsdd-cli#855 and on `content-delivery-assessment-2026-10-02`).
- **`CROSSLINK_HOOK_PROVIDER`** (`claude` or `codex`) is the only environment variable the hooks read. It selects the output format.
- **De-duplication suppresses most prompts.** `claim_event` (`hook_protocol.py:293-326`) keys on session id, turn id and tool-use id with a 600-second window, and the prompt-submit payload observed on Claude Code 2.1.284 carries `prompt_id` but no `turn_id` (vsdd-cli#890). So every prompt in a session hashes to the same key: the prompt hook emits at most once per ten minutes per session, and its every-third-prompt counter counts only the prompts that get through. The same applies to subagent start. Read from the code against the observed payload; not run end to end. A candidate upstream issue, not filed.
- **Not determined:** what Claude Code does with the session-start hook's exit 2.

---

### 9. what a project can ship through crosslink, and what init does to it

Verified 2026-10-02.

| Slot | What the project controls | How it reaches the agent | Limits |
|---|---|---|---|
| Fixed-name rule files (section 3) | File content | Mechanical: the prompt hook | Fixed names; upstream expects them empty |
| `design-doc:<slug>` label | Which knowledge pages | Mechanical: session start | 3 pages, 8,000 characters each |
| Kickoff prompt template | The dispatched agent's prompt | Mechanical, dispatched agents only | See `kickoff-swarm-dispatch-pipeline` |
| `hook-config.json` | Block and gate lists, tracking mode, cadence | Mechanical: the hooks | No content or path keys |
| Hook entries in `.claude/settings.json` | The project's own hooks | Mechanical, where the entries survive | Replaced by any plain `crosslink init` that is not skipped; always replaced in a kickoff worktree |
| Extra servers in `.mcp.json` | The project's own entries | Tool availability | Preserved on merge |
| Knowledge pages | Content and tags | By the agent's judgment | Substring search |
| The project's own folders under `.claude/skills/` and `.claude/commands/` | Everything | Claude Code's own discovery | Crosslink ignores them; its managed ignore block ignores both directories wholesale |

- **Crosslink has no way to register a downstream skill.** Its 18 skills are compiled in from `resources/agent/skills/` (`build.rs:138-251`), deployed to `.claude/skills/` for Claude and `.agents/skills/` for Codex. Their frontmatter is `name` and `description` only. They load by description match or explicit invocation; no hook references them. The 14 files in `.claude/commands/` are thin wrappers that say "use the `<name>` skill"; a fresh init writes both sets.
- **Crosslink does not manage `.claude/agents/` or `.claude/rules/`.**
- **The init manifest** (`.crosslink/init-manifest.json`) maps each managed path to the hash of its template and the writing version. It lists only crosslink's files.
- **`init --update`** walks template paths and old manifest paths only (`src/commands/init/mod.rs:757-801`). A missing file is "deleted by user" and not recreated. A file the user changed is left alone when the template is unchanged, and is a conflict when both changed: prompted on a terminal, kept otherwise. Unknown files are left alone and not flagged. It never touches either ignore file (crosslink#105).
- **A plain `crosslink init`** skips only when every managed file is already present (`mod.rs:1011-1029`); otherwise it always rewrites the merge-aware files (`mod.rs:1225-1235`). For `.claude/settings.json` that means **the whole `hooks` object is replaced with crosslink's template** (`src/commands/init/merge.rs:216-218`); other top-level keys survive. Observed 2026-10-02 in an isolated clone (vsdd-cli#890): a project hook entry was dropped, the template's fail-open wrapper replaced vsdd-cli's fail-closed one, and three tracked files were left modified. It also rewrites the managed block of the root `.gitignore`, converting the legacy markers vsdd-cli uses and replacing everything between them (`merge.rs:10-12, 61-74`), which would drop vsdd-cli's keep-lines for `.claude/commands/vsdd-*` (crosslink#20). Kickoff runs a plain init in every worktree.
- **`init --force`** overwrites every managed file, rewrites all rule files empty, and resets `hook-config.json`.
- **`crosslink context check`** verifies presence only (rules, hooks, commands, provider files, valid hook-config JSON) and recommends `init --force` on failure. **`crosslink context measure`** prints bytes divided by four per rule file and records nothing. Neither enforces a budget or takes a project-defined file list.
- **`crosslink style sync`** (the `house_style` key) copies rules, hooks and commands from a git repo on a manual command and skips marked files. It was only skimmed.

---

### upstream doc index (docs_src/)

> Not re-verified on 2026-10-02; see the currency note at the top.

Guides: `hooks`, `kickoff`, `knowledge`, `multi-agent`, `swarm`, `session-workflow`, `tracking-modes`, `tui`, `web-dashboard`, `container-agents`, `design-workflow`, `maintenance`. Reference: `commands`, `hook-config`, `kickoff-report`, `rules`, `state-files`.

### the most design-relevant unwired surfaces (vsdd-cli, at basis)

> Not re-verified on 2026-10-02; see the currency note at the top.

1. **The localhost REST/WS server + orchestrator API** — stage-lifecycle transitions, `agents/poll`, decompose/execute, WebSocket progress events; nothing in vsdd consumes it (the daemon isn't even running here), yet it is the only machine-readable *push* channel for exactly the run-state vsdd's Status/gate designs re-derive from files.
2. **The sentinel loop** (`sentinel.enabled: false`) — issue-sourced autonomous dispatch with an escalation ladder; the closest existing mechanism to the phases-dispatched keystone (#840) and never evaluated for it.
3. **`swarm.toml` trust-model config** (absent) — the triage-prior surface `swarm review` consumes; Slice 6 binds `crosslink swarm review` as the phase-3 exit act, so its priors file is a direct, currently-unauthored vsdd input. (Runner-up: the `house_style` seam, named in the vsdd contract as an install target, unset.)

Known gaps carried into vsdd designs: upstream **#60** (timeout kill never writes a terminal sentinel — RUNNING lies) and **#61** (swarm launch surface lacks the effort dial).
