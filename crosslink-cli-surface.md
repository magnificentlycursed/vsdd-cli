---
title: "The crosslink CLI surface — exhaustive inventory"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
---


## Design Specification

### overview

| Family | Commands |
|---|---|
| Tracking | `issue` (24 subcommands), `timer`, `milestone`, `archive`, `export`, `import`, `session`, `workflow trail`, hidden shortcuts `create` `quick` `list` `show` `close` `subissue` `start` `stop` |
| Orchestration | `kickoff` (11), `design`, `swarm` (29), `sentinel` (7), `agent` (8), `container` (9), `daemon` (3) |
| Knowledge | `knowledge` (8) |
| Infrastructure | `init`, `config` (6), `migrate` (5), `sync`, `locks` (5), `trust` (5), `integrity` (6), `compact`, `prune`, `cpitd` (3), `style` (5), `context` (2), `workflow diff`, hidden `heartbeat` |
| Observability | `tui`, `mc`, `dashboard` (5), hidden deprecated `serve` |

**Hidden top-level shortcuts** (work, absent from `crosslink --help`; documented in `commands.qmd`): `create`, `quick`, `list`, `show`, `close` → the matching `issue` subcommand; `subissue <parent> <title>` → `issue create --parent`; `start`/`stop` → `timer start`/`timer stop`; `heartbeat` — push an agent heartbeat (what the heartbeat hook calls); `serve` — deprecated alias for `dashboard`. There is **no top-level `intervene`** — intervention logging is `issue intervene` (see Deltas).

---

### tracking family

### issue

Issue lifecycle. Subcommands and key flags (binary-verified):

| Subcommand | Purpose | Key flags |
|---|---|---|
| `create <title>` | Create issue | `-d` desc, `-p` low/medium/high/critical (default medium), `-t` template (bug/feature/refactor/research), `-l` label (repeatable), `-w` set as session work item, `--parent` (subissue), `--scheduled` / `--due` (YYYY-MM-DD or RFC 3339), `--defer-id` (batch: display ID assigned later), `--force` (bypass `template_required_fields` validation, gh#658) |
| `quick <title>` | Create + label + `session work` in one act | same as create minus `-w`/`--defer-id` |
| `list` | List issues | `-s` open/closed/all, `-l` label, `-p` priority, `--repo` (external repo: URL, local path, or @alias), `--refresh` |
| `search <query>` | Text search | `--repo`, `--refresh` |
| `show <id>` | Issue details | `--repo`, `--refresh` |
| `update <id>` | Update | `-t` title, `-d` desc, `-p` priority, `--scheduled/--no-scheduled`, `--due/--no-due` |
| `close <id>` | Close | `--no-changelog` (skip the auto-CHANGELOG entry) |
| `close-all` | Close all matching | `-l`, `-p`, `--no-changelog` |
| `reopen <id>` / `delete <id>` | Reopen / delete | delete: `-f` skip confirmation |
| `comment <id> <text>` | Typed comment | `--kind` note/plan/decision/observation/blocker/resolution/result/handoff/human (default note; validation additionally knows `intervention` and `system`; unknown kinds warn but are accepted) |
| `intervene <id> <desc>` | Log a driver intervention | `--trigger` REQUIRED (tool_rejected, tool_blocked, redirect, context_provided, manual_action, question_answered), `--context`; gated by `intervention_tracking` config; stamps `driver_key_fingerprint` when the driver key resolves |
| `label` / `unlabel <id> <label>` | Label management | — |
| `block` / `unblock <id> <blocker>` | Dependency edges | — |
| `blocked` / `ready` | List blocked / unblocked-open | — |
| `relate` / `unrelate <id> <other>` / `related <id>` | Undirected relations | — |
| `next` | Suggest next issue | — |
| `tree` | Parent/child hierarchy view | `-s` open/closed/all (default all) |
| `tested` | Reset the test reminder (writes `.crosslink/last_test_run`) | — |

VSDD-RELEVANCE: core consumed surface (issue/quick/comment/close/list/show per the affordance map's issue-lifecycle entry; typed comments are the audit-trail substrate the finding-query-join design reads). `intervene` is mapped (mid-flow-intervention-record) — present in this binary, nested under `issue`. Under-consumed: `block/blocked/ready/tree` (decomposition dependency edges live in prose today), `--scheduled/--due`, `-t` templates + `template_required_fields` (a mechanical required-fields gate vsdd could ride for finding-shaped issues), `--defer-id` batch creation, `close-all`.

### timer

`start <id>` / `stop` / `show` — per-issue time tracking into the `time_entries` table. Top-level `start`/`stop` alias it.
VSDD-RELEVANCE: unadopted; direct substrate for the Cost-is-knowable slice (wall-clock per issue).

### milestone

`create <name> [-d]`, `list`, `show <id>`, `add <id> <issues...>`, `remove <id> <issue>`, `close <id>`, `delete <id>`.
VSDD-RELEVANCE: **actively consumed** — the estate's slice decomposition lives here (live `milestone list` shows #7 Engine through #14 Slice 7), but slices carry 0/0 issues — no issues have been attached via `milestone add`, so progress is not machine-readable. Cheap win.

### archive

`add <id>` (archive a closed issue), `remove <id>`, `list`, `older <days>` (bulk-archive).
VSDD-RELEVANCE: hygiene only; the project rules note re-reading state after `archive add` (0.8.0 persistence-defect conduct, held until re-verified).

### export / import

`export [-o file] [-f json|markdown]` — full issue export; `import <file>` — JSON import.
VSDD-RELEVANCE: unadopted; `export -f json` is a one-shot evidence snapshot the verification designs could consume instead of walking SQLite.

### session

`start`, `end [-n handoff-notes]`, `status`, `work <id>` (bind the active issue), `last-handoff`, `action <text>` (breadcrumb for context-compression recovery).
VSDD-RELEVANCE: heavily consumed — the phase primers reference "crosslink session breadcrumb" 14 times (that is `session action`); session-binding is a mapped affordance; `session end -n` handoff notes are the cross-session memory channel. `--json session status` is CI-consumable.

### workflow trail

`trail <id> [--kind plan,decision,...]` — chronological comment trail filtered by kind.
VSDD-RELEVANCE: named in the consumer's usage surface; this is the built query surface closest to the finding-query-join design's needs (kind-filtered trail per issue; cross-issue joins remain vsdd's to build).

---

### orchestration family

### kickoff

Launch an agent to implement a feature (local tmux process or container).

| Subcommand | Purpose | Key flags |
|---|---|---|
| `run <description>` | Launch implementation agent | `--issue`, `--doc <design.md>`, `--branch`, `--container none/docker/podman` (default none), `--image` (default `ghcr.io/dollspace-gay/crosslink-agent:latest`), `--verify local/ci/thorough` (default local), `--model` (default opus), `--timeout` (default 1h), `--dry-run` (print prompt), `--skip-permissions` (one-shot `--dangerously-skip-permissions`), `--permission-mode acceptEdits/auto/bypassPermissions/default/dontAsk/plan` (mutually exclusive with `--skip-permissions`) |
| `status [agent]` | Agent status; no args = pipeline overview | — |
| `logs <agent>` | Tail event log | `-l` lines (default 20) |
| `stop <agent>` | Stop agent | `--force` SIGKILL |
| `plan <doc>` | Read-only gap analysis of a design doc against the codebase | `--issue`, `--model`; **mirror adds** `--timeout`, `--dry-run`, `--skip-permissions`, `--permission-mode` (gh#66/PR#72 — headless plan dispatch needs `--skip-permissions` to clear the workspace-trust dialog; NOT in the installed binary yet) |
| `show-plan <agent>` | Display a gap report | — |
| `report [agent]` | Spec validation report from a completed agent | `--json`, `--all` (aggregate across worktrees) |
| `list` | All kickoff agents across worktrees, tmux, Docker | — |
| `cleanup` | Remove completed/stale worktrees/sessions/containers | `--dry-run`, `--keep <n>` |
| `graph` | Branch topology of kickoff feature branches | `--all` |
| `launch [doc]` | Interactive pipeline wizard or direct launch | `--plan` / `--run` (non-interactive), plus run's flags |

VSDD-RELEVANCE: the mapped autonomous-execution affordance (conditional: container kickoff was blocked upstream; #837 recorded kickoff broken — the mirror's permission-flag work is the retest trigger). `kickoff plan`/`show-plan` is the real gap-analysis surface (the affordance map's `design --gap-analysis` does not exist — see Deltas). `report --json` is a machine-readable per-criterion verdict artifact vsdd's conformance verifier could consume. The design-doc pipeline consumes `.design/<slug>.md` + the pipeline state file (`composition-slice.pipeline.json` in the working tree is one).

### design

`design [description] [--issue N] [--gh-issue N] [--continue <slug>]` — foreground Claude session for design authoring; writes `.design/<slug>.md`. No other flags. VSDD-RELEVANCE: consumed (the `/design` path); `--continue` for iteration.

### swarm

Multi-agent coordination. 29 subcommands:

- Plan/lifecycle: `init --doc <design.md>` (build phase plan), `status`, `resume`, `sync-status` (reconcile live worktree state into phase JSON), `adopt --slot <slug> <agent>` (bind an external agent/branch to a slot), `archive`, `reset [--no-archive]`, `list`.
- Execution: `launch <phase> [--budget-aware]`, `gate <phase>` (run the project test suite as the phase gate), `checkpoint <phase> [--notes]`.
- Budget/cost: `config --budget-window <dur> [--model]`, `estimate <phase>`, `harvest` (scan completed agents, update cost history), `plan [--budget-window]`, `plan-show`.
- Review pipeline: `review [--agents N] [--mandate adversarial] [--doc out.md]` — partitions the codebase by seams, **writes a review plan to the hub (`swarm/review-plan.json`) and does NOT itself launch agents** (source-verified at mirror HEAD, `src/commands/swarm/review.rs`); `fix --issues 1,2,3` (parallel fix agents); `merge [--branch] [--dry-run]`; `pipeline [--agents] [--mandate] [--auto-file-issues]` (the standalone review→fix driver that does launch and consolidate); `review-continue`, `review-status`; `trust-init [--model local-only/multi-tenant/public-api]`.
- Plan surgery: `move --to-phase`, `merge-phases`, `split-phase --after`, `remove-agent`, `reorder --position`, `rename-phase`.

VSDD-RELEVANCE: `swarm review` (phase-3-review-round) and `swarm gate` (phase-exit-gate) are mapped, conditional on the swarm live-fire criterion. The budget subfamily (`config/estimate/harvest/plan`) is unconsumed and is the second Cost-is-knowable substrate upstream already carries. Note: the consumer usage list says "swarm run" — no such subcommand; the launch act is `swarm launch <phase>`.

### sentinel

Autonomous maintenance: `run [--dry-run] [--label]` (one-shot sweep), `watch [--interval min]` (daemon), `status`, `history [--limit]`, `stop`, `metrics` (dispatch success per model/rule), `patterns` (recurring hotspots). Sources configurable: GitHub labels (e.g. `agent-todo: fix`), periodic cpitd scans.
VSDD-RELEVANCE: deployed config has `sentinel.enabled: false` — named in the consumer usage surface but not actually active. Its metrics/history tables (see data model) are a dispatch-outcome record vsdd's efficiency insight engine could read.

### agent

Identity + control plane: `init <agent-id> [-d] [--no-key] [--force]`, `status`, `prompt <session> <message> [--no-submit]` (send a prompt into a running tmux agent), `bootstrap --repo <url> --identity <id> [--branch] [--target]`, `request <target> kill|pause|resume|reprioritise [--subject-issue] [--reason]` (signed control file on the hub), `requests [--target] [--pending]`, `poll-requests` (apply pending pause/kill/reprioritise locally, ack to hub), `flags [--strict]` (JSON `{paused, kill, reprioritise}`; `--strict` exits non-zero for PreToolUse hooks).
VSDD-RELEVANCE: unadopted control plane. `agent flags --strict` is a shipped harness-level pause/kill switch wireable as a PreToolUse hook — an enforcement-grade rung above vsdd's convention-grade controls that the estate has never turned on. `agent prompt` is the programmatic steering channel for attended kickoff runs.

### container

`build [--force] [--tag] [--dockerfile]` (local image `ghcr.io/dollspace-gay/crosslink-agent:<tag>`), `start <worktree> [--name] [--prompt] [--issue]`, `ps`, `logs <name> [-f] [--tail]`, `stop`, `rm`, `kill`, `shell <name>`, `snapshot <name>` (cache installed toolchains as an image).
VSDD-RELEVANCE: named consumed (container-based execution posture); `snapshot` is unadopted and would cut Rust toolchain warm-up cost per dispatch.

### daemon

`start` / `stop` / `status` — background daemon (state flushes, `.crosslink/daemon.log`). VSDD-RELEVANCE: infrastructural; not directly consumed.

---

### knowledge family

### knowledge

`add <slug> [--tag]... [--from-doc <path>] [--contributor <id>]...`, `show <slug> [--repo] [--refresh]`, `list [--tag] [--contributor] [--since YYYY-MM-DD] [--repo] [--refresh]`, `edit <slug> [--append | --from-doc] [--tag] [--source] [--contributor]`, `remove <slug>`, `sync`, `import <dir> [--overwrite] [--dry-run]` (bulk markdown), `search [query] [-C context] [--source <domain>] [--since] [--contributor] [--repo] [--refresh]`.

VSDD-RELEVANCE: consumed (add/edit/search); `--from-doc` is the design-doc-to-knowledge-page path; contributor attribution (GH#628) unconsumed.

### Can knowledge pages be shared across projects? (verified from source + binary)

- **(a) Where pages live:** an orphan branch `crosslink/knowledge` on the project's own `tracker_remote` (constant `KNOWLEDGE_BRANCH` in `src/knowledge/core.rs`), materialized locally at `.crosslink/.knowledge-cache/` (a worktree of that branch). The hub-v3 migration does **not** move it — `migrate hub-v3`/`hub-branches` rename only `refs/crosslink/agents/*`, `checkpoint`, and `meta`; `prune` treats hub and knowledge as separate branches (`--hub-only` / `--knowledge-only`).
- **(b) One shared remote for many projects:** `tracker_remote` (config, default `origin`) is the remote name for **both** hub and knowledge branches. Pointing several projects at one remote is physically possible but nothing namespaces the data: one flat page-slug namespace on `crosslink/knowledge`, one set of per-agent hub refs (`crosslink/agents/<agent-id>` — same agent id from two projects collides on the same ref), one issue display-id counter, one `.crosslink/repo-id`-keyed compact-agent-id scheme. Not a sanctioned mode; collision-prone by construction.
- **(c) Manual paths as-built:** `knowledge show <slug>` (pipe out) and `knowledge import <dir>` / `add --from-doc` (pull in) are the write-side transfer paths; the cache worktree files under `.crosslink/.knowledge-cache/` are plain markdown with frontmatter and can be copied wholesale.
- **(d) Federation surface:** cross-repo **reads** are built and present in the installed binary: `knowledge show/list/search --repo <URL | local path | @alias>` (and the same on `issue list/show/search`), backed by `src/external.rs` fetching the other repo's `crosslink/knowledge` branch into `.crosslink/.external-cache/` with TTLs (`external-cache-ttl`, default 300s) and named aliases via the `repo-alias` config map. The multi-project dashboard does **not** read knowledge stores (grep of `src/dashboard/` finds no knowledge access). No push/subscription federation exists anywhere.

**Verdict:** knowledge is per-project — each estate's pages live on its own repo's `crosslink/knowledge` orphan branch, and hub-v3 leaves that unchanged. The sanctioned sharing path today is read-only pull: `knowledge show/list/search --repo <url|path|@alias>` through the external cache (plus manual `show`-out / `import`-in for a real copy); co-locating multiple projects on one `tracker_remote` is unsupported and collides on page slugs, agent refs, and issue counters. Push/subscription federation has no surface at all, so if the estate wants it, that is an upstream feature ask.

---

### infrastructure family

- **init** — deploy/refresh the managed install. Flags: `--defaults` (skip TUI, team preset), `--reconfigure`, `--force`, `--update` (manifest-tracked three-way merge upgrade), `--dry-run`, `--no-prompt` (CI: keep user-modified files), `--python-prefix`, `--skip-cpitd`, `--skip-signing`, `--signing-key`. Deploys (from `src/commands/init/mod.rs`, embedded at build time): 7 hooks → `.claude/hooks/`, 3 MCP servers → `.claude/mcp/`, 14 slash commands → `.claude/commands/`, 18 skills → `.claude/skills/`, `.claude/settings.json` (hook wiring, `__PYTHON_PREFIX__` substituted), `.mcp.json`, `.crosslink/hook-config.json`, `.crosslink/rules/` (31 files), tracked in `.crosslink/init-manifest.json`.
- **config** — `show`, `get <key>`, `set <key> [value] [--add|--remove] [--local]` (`hook-config.local.json`), `list`, `reset [key]`, `diff`; bare `config` opens the walkthrough, `--preset team|solo` applies directly. 34 registered keys incl. `tracking_mode`, `comment_discipline`, `signing_enforcement`, `tracker_remote`, `repo-alias`, `template_required_fields`, `kickoff.allowed_tools`, the sentinel tree.
- **migrate** — `to-shared`, `from-shared`, `rename-branch` (locks→hub), `hub-v3` (`--finalize --yes-delete-v2`, `--adopt-stale`, `--remigrate-from-v2`), `hub-branches` (hidden refs → visible `crosslink/*` branches, #767, idempotent). Hidden alias `migrate-from-shared`.
- **sync** — fetch hub refs from `tracker_remote`, replay/compact events, hydrate SQLite, push own agent ref. The one command CI runs in the consumer's routing gate.
- **locks** — `list`, `check <id>`, `claim <id> [-b branch]`, `release <id>`, `steal <id>` (stale locks).
- **trust** — `approve/revoke <agent-id>`, `list`, `pending`, `check` — SSH signing trust store for hub entries.
- **integrity** — `counters`, `hydration [--accept-data-loss]` (SQLite↔JSON, snapshot always written under `.crosslink/integrity/`), `locks`, `schema`, `layout` (mixed V1/V2 hub files), `sign-backfill [--confirm] [--key]` (retroactively sign unsigned hub entries with a human key — attestation).
- **compact** `[--force]` — manual event compaction; **prune** `[--dry-run] [--force] [--keep-commits N] [--hub-only|--knowledge-only]` — squash hub/knowledge branch history.
- **cpitd** — `scan [paths] [--min-tokens 50] [--dry-run]` (file clone issues), `status`, `clear`.
- **style** — `set <url>`, `sync [--dry-run]`, `diff`, `show`, `unset` — house-style source syncing.
- **context** — `measure` (token overhead of injected context), `check` (verify all expected crosslink files deployed and valid).
- **workflow diff** `[--check]` — deployed policy files vs embedded defaults; `--check` is CI mode (exit 1 on undeclared drift without a `# crosslink:custom` marker).

VSDD-RELEVANCE: `sync` is CI-consumed (routing gate). `workflow diff --check` and `context check` are shipped CI-grade drift/presence gates the estate has not wired into its own verify workflow — directly relevant to the installed-artifact-integrity check the contract names. `integrity sign-backfill` + `trust` + `signing_enforcement` are the tamper-evidence ladder the corroboration keystone binds to (currently `signing_enforcement: audit` in the consumer). `context measure` is a third Cost-is-knowable input (context injection overhead).

---

### observability family

- **tui** — read-only interactive terminal dashboard (has a knowledge tab).
- **mc** `[--layout tiled|even-horizontal|even-vertical]` — tmux mission control of all active agents.
- **dashboard** — multi-project control panel (GH #429): `serve [--port 3100]`, `track <path> [--slug] [--init --agent-id]`, `untrack <slug>`, `list`, `discover [--root]... [--depth 4] [--track]`. Reads hub state; shells out to the real CLI for writes.
- **serve** (hidden) — deprecated alias for `dashboard`.

VSDD-RELEVANCE: the contract's "vsdd builds no viewer" stance points here; the dashboard's `token_usage`-backed views make it the Cost-is-knowable display surface. None are consumed today (operator-side tools).

---

### hooks and skills: the deployed-payload surface and its utilization

### What crosslink ships (mirror `crosslink/resources/claude/`)

**Hooks** (7 Python scripts, embedded into the binary, deployed to `.claude/hooks/`, wired via the deployed `.claude/settings.json`):

| Hook | Trigger (as wired) | Purpose |
|---|---|---|
| `crosslink_config.py` | (shared module) | Config loading + utilities imported by the others |
| `session-start.py` | SessionStart (startup\|resume) | Load crosslink context, auto-start/auto-end stale sessions |
| `prompt-guard.py` | UserPromptSubmit | Inject best-practice reminders from `.crosslink/rules/` every prompt |
| `work-check.py` | PreToolUse (Write\|Edit\|Bash) | Block code changes without an active issue; enforce comment discipline; git command blocking/gating |
| `post-edit-check.py` | PostToolUse (Write\|Edit) | Stub-pattern detection, linters, test reminders |
| `pre-web-check.py` | PreToolUse (WebFetch\|WebSearch) | Inject prompt-injection interdiction framing before web calls |
| `heartbeat.py` | PostToolUse (all tools) | Throttled (2 min) `crosslink heartbeat` push — liveness for stale-lock detection |

**MCP servers** (3, deployed to `.claude/mcp/`): `knowledge-server.py`, `safe-fetch-server.py`, `agent-prompt-server.py`.

**Slash commands** (14, `.claude/commands/`): audit, check, commit, crosslink-guide, design, dev-release, featree, feature, kickoff, maintain, preflight, qa, review, workflow.

**Skills** (18, `.claude/skills/`): the 14 above as skills (review as `review-pre-commit`) plus architect, rust-quality, rust-fix-discipline, rust-gpu-discipline.

**Rules** (31 files → `.crosslink/rules/`): global, project, quality, rigor, knowledge, tracking-strict/normal/relaxed, sanitize-patterns.txt, plus 22 language files.

### What vsdd-cli has deployed, and drift

All 7 hooks, all 3 MCP servers (enabled in `settings.local.json`), all 14 commands (plus 21 vsdd-native `vsdd-phase-*`/`vsdd-domain-*` commands), all 18 skills, all 31 rules are present. `crosslink workflow diff` (run 2026-08-01):

- `hook-config.json`: customized (107 lines) — strict tracking, `comment_discipline: encouraged`, gated `git commit`, agent_overrides (cargo lint/test commands, destructive-git block list, relaxed agent tracking), sentinel disabled, `signing_enforcement: audit`.
- `rules/project.md`: customized (64 lines) — vsdd's naming/register rules, version-hold history, contract pointers.
- `.claude/hooks/work-check.py`: customized (175 lines) — **deliberate**: carries the git-posture PR-boundary policy (operator-ratified, vsdd-cli #856: agents commit/branch/push feature branches; merges and history surgery are the human's), and reroutes the shipped hook's `crosslink intervene ...` guidance (a command that does not exist at top level — upstream bug, see Deltas) to `issue comment --kind result`.
- Everything else matches defaults. The consumer also hardened the settings.json wiring itself: each hook command fails closed with a "hook payload missing" error if the script is absent (vsdd-cli #658) — a consumer-side improvement over the shipped template.

### Utilization audit — used vs ignored

**Demonstrably used:** the 5 wired hook events fire constantly (that is the behavioral-guard experience); `/design` and `/commit` are session-consumed; the MCP knowledge/search server is live in sessions; milestones, sessions/breadcrumbs, typed comments, knowledge add/edit/search, sync-in-CI.

**Referenced-but-nonexistent (phantom):** the vsdd-phase-1c and vsdd-phase-2a primers name a `post-design-md-modification.py` hook that auto-scaffolds `manual-tests/layer-N.md` checklists and Red Gate stubs. **No such file exists** in the mirror's shipped resources, the deployed `.claude/hooks/`, or anywhere in either repo — and no auto-scaffolded stubs have ever appeared. The primers assert a mechanism that was never built. Either build it as a vsdd-native PostToolUse hook (matcher on `.design/*.md` edits) or amend the primers.

**Authored-not-exercised:** `templates/registry/anonymization-patterns.yaml` exists and the security/privacy domain lenses plus the verifiable-conformance design reference an anonymization check, but no hook/script anywhere enforces it (the #840 finding stands: patterns data without an executor).

**Deployed-but-never-invoked** (each with what adopting would buy, for the adopt/decline ruling):

| Item | Would buy |
|---|---|
| `/preflight` | Loads rules + tracking mode once instead of per-prompt re-injection; directly serves the efficiency subsystem's context-cost goal |
| `/audit` | The stuck-session context dump; overlaps vsdd's own routing discipline — plausibly decline as superseded |
| `/check` | Structured monitoring of kickoff agents; becomes relevant the moment kickoff is retested |
| `/maintain` | Periodic hygiene incl. issue hygiene; overlaps mdatron + cargo CI — partial overlap, could decline |
| `/qa`, `/review-pre-commit` | Generic review gates; vsdd's domain-lens reviews are stronger and contract-bound — decline as superseded, with a stated reason recorded (the affordance map's rule requires exactly this) |
| `/feature`, `/featree` | Branch/worktree conventions; vsdd's PR workflow covers the branch act — mostly superseded |
| `/dev-release` | Release pipeline; relevant only at first vsdd-cli release — defer, don't decline |
| `/crosslink-guide`, `/workflow` | Reference + config-drift walkthrough; `workflow diff --check` in CI is the mechanical version worth adopting instead |
| `/kickoff` (skill) | The attended launch path; blocked on the kickoff retest trigger |
| rust-quality / rust-fix-discipline skills | Already redundant with the estate's own loaded skills — decline |
| `agent flags --strict` as a PreToolUse hook | A shipped harness-grade pause/kill control, unwired — the only enforcement-grade rung the estate could turn on today without building anything |

The affordance map's own evidence line ("no crosslink workflow was ever self-summoned — every affordance use traced to an operator instruction") remains accurate for this surface: utilization is operator-driven, and 11 of the 14 shipped commands have no recorded use.

---

### the tracker data model

Source: mirror `src/db/core.rs` (migrations), `src/issue_file.rs` (hub file shape), `src/events.rs` (event log), live `.crosslink/issues.db` of vsdd-cli (schema version 17, matching the binary's `SCHEMA_VERSION`).

### Tables (consumer-verified schema)

- **issues** — id, title, description, status (open/closed), priority, `parent_id` (FK → issues, CASCADE — the subissue tree), created/updated/closed_at, `uuid` (migration v10 — stable identity for shared coordination; display id is per-hub and re-assignable at promotion), `created_by` (v10 — authoring agent id), `scheduled_at`/`due_at` (v17).
- **labels** — (issue_id, label) many-to-many, plain strings. Semantics live in consumers: changelog-skip rules, sentinel `github_labels` dispatch, and vsdd's finding-discrimination rule all read labels; nothing constrains the vocabulary.
- **dependencies** — (blocker_id, blocked_id) directed edges (`issue block`, `blocked`, `ready`).
- **relations** — undirected related pairs.
- **comments** — id, issue_id, content, created_at, `uuid` + `author` (v10), `kind` (v11, default note), `trigger_type` + `intervention_context` (v12), `driver_key_fingerprint` (v13).
- **time_entries** — per-issue timer intervals.
- **milestones** / **milestone_issues** — name/description/status + uuid; M2M to issues.
- **sessions** — started/ended_at, active_issue_id, handoff_notes, `last_action` (the breadcrumb), agent_id.
- **token_usage** (v15) — agent_id, session_id, timestamp, input/output/cache_read/cache_creation tokens, model, cost_estimate. Built for the dashboard.
- **sentinel_runs** / **sentinel_dispatches** (v16) — sweep stats; per-dispatch signal, disposition, agent, model_used, attempt_number, outcome.

### Comment kind vocabulary

CLI-advertised: `note, plan, decision, observation, blocker, resolution, result, handoff, human` (default note). The validation list (`KNOWN_COMMENT_KINDS`, `src/issue_file.rs`) additionally knows `intervention` and `system`. Unknown kinds log a warning but are **accepted** — the vocabulary is convention-grade, not schema-enforced. The REST server's enum is narrower (no handoff/human/system) and requires `trigger_type` when kind is intervention (#573).

### Authorship substrate

`comments.author` is stamped from the **local agent identity** at write time — a self-declaration, not a verification. `driver_key_fingerprint` is resolved from the driver's key (`.crosslink/driver-key.pub`) and attached **only on the `issue intervene` path, only by the local CLI** (`src/commands/intervene.rs`); the REST handler explicitly writes `None` ("not available via REST"), and ordinary comments never carry it. So driver-attestation exists for interventions logged at the tracked machine, and nowhere else. The stronger, general mechanism is one level down: every hub event is appended to the writing agent's own ref and SSH-signed with that agent's key; `trust` approve/revoke governs which keys count, `signing_enforcement` (audit → enforced) sets whether unsigned/untrusted entries are tolerated, and `integrity sign-backfill` lets a human key retroactively attest pre-signing history. The consumer currently runs `signing_enforcement: audit`.

### Hub + sync model

The SQLite database is a **local hydration cache**; the canonical shared state is git. Hub-v3 layout (visible branches after `migrate hub-branches`, #767): one append-only event log per agent at `crosslink/agents/<agent-id>`, a compaction checkpoint at `crosslink/checkpoint`, metadata at `crosslink/meta`, all on the `tracker_remote`; knowledge is a separate orphan branch `crosslink/knowledge`. Events carry a total ordering key (timestamp, agent_id, agent_seq) and are replayed/compacted into the issue-file state tree. `crosslink sync` = fetch all refs → replay/compact → hydrate SQLite → push **own agent ref only**. An agent can only append to its own ref; rewriting another agent's history is a git force-push visible in the ref history, and signature verification against the trust store detects impersonation. Locks and agent control requests are hub files under the same model. Heartbeats are per-agent JSON written by the PostToolUse hook.

### Evidentiary vs agent-writable, per record class

| Record class | Verdict |
|---|---|
| Issue bodies, labels, dependencies, relations, milestones | Agent-writable, self-attested; tamper-evident only via signed hub events + append-only per-agent refs |
| Comments (all kinds incl. `human`) | Agent-writable; `author` and `kind` are self-declared (a `human`-kind comment proves nothing by itself) |
| Intervention comments with `driver_key_fingerprint` | Driver-corroborated — the one comment-level record that binds to the driver's key; **only** minted by the local CLI path |
| Hub event signatures + trust store | The general corroboration layer: authorship verified against approved keys; grade depends on `signing_enforcement` (audit = detect, enforced = block) |
| Sessions, breadcrumbs, time entries, token_usage | Agent-writable telemetry; honest-by-default, not adversary-proof |
| Heartbeats | Mechanically generated by hooks; liveness evidence, trivially forgeable by a hostile agent, fine against drift |
| Checkpoint/compaction state | Mechanically derived — re-derivable from the event logs, so falsification is detectable by recomputation |
| sign-backfill attestations | Human-key attestation, retroactive — evidentiary but post-hoc |

**Top data-model finding for the corroboration keystone (#815):** the server-synced authorship substrate is real but narrower than the design language suggests — `author` is self-declared everywhere, and `driver_key_fingerprint` exists only on locally-logged interventions (REST writes None). Anything that must distinguish "the driver did this" from "an agent said the driver did this" has exactly two load-bearing anchors today: the intervention fingerprint path, and hub-event signature verification against the trust store — and the latter is running in `audit`, not `enforced`, in this estate.

---

### surfaces vsdd does not consume today

Plausibly useful, flagged ▲; rightly unused / operator-side, unmarked:

- ▲ `issue block/unblock/blocked/ready/tree` — machine-readable decomposition dependencies (slice ordering is prose today)
- ▲ `issue -t templates` + `template_required_fields` config — a mechanical required-fields gate for finding-shaped issues
- ▲ `milestone add` — attach issues to the already-created slice milestones so progress is queryable
- ▲ `timer` / `time_entries`, `token_usage`, `swarm config/estimate/harvest/plan`, `context measure` — the Cost-is-knowable substrate upstream already carries (Slice 7 should consume, not rebuild)
- ▲ `workflow diff --check` + `context check` in CI — the shipped installed-artifact-integrity gates (CI-backed block grade)
- ▲ `agent flags --strict` as a PreToolUse hook; `agent request/poll-requests` — the shipped pause/kill control plane
- ▲ `kickoff report --json` / `show-plan` — machine-readable spec-validation verdicts for the conformance verifier
- ▲ `export -f json` — one-shot tracker evidence snapshots
- ▲ `knowledge --repo/@alias` cross-repo reads — pull the crosslink mirror estate's pages without copying
- ▲ `container snapshot` — toolchain-warm images for Rust dispatch
- `issue --scheduled/--due`, `close-all`, `archive older` — hygiene conveniences
- `cpitd`, `style`, `prune`, `compact`, `integrity` (routine), `daemon` — operator/maintenance surface
- `tui` / `mc` / `dashboard` — viewers (contract: vsdd builds no viewer; these are the viewers)
- `sentinel` — disabled in config; revisit only with an autonomous-maintenance appetite
- `migrate` family — one-shot, already executed for this estate

---

### deltas vs the estate's prior knowledge

Checked against `knowledge search` for kickoff/design/swarm (pages: attended-design-autonomous-execution, conformance-efficiency-subsystem-audit, agent-first-controls) and the affordance map at `templates/registry/act-to-affordance-map.md`.

1. **`crosslink design --gap-analysis` does not exist.** The affordance map's spec-to-build-gap-analysis entry names it; the built surface is `kickoff plan <doc>` / `kickoff launch --plan` / `kickoff show-plan`. The map entry needs re-pointing.
2. **`issue intervene` exists** (this binary), nested under `issue` with `--trigger` required — the map's naming-drift note (contract prose says `crosslink intervene`, surface says `issue intervene`) is still accurate at 0.9.0-beta.1. New upstream bug found: the **shipped default `work-check.py` itself instructs agents to run top-level `crosslink intervene ...`** (3 occurrences), which errors as an unknown subcommand; vsdd's customized copy already routes to `issue comment --kind result`. Worth filing upstream (relates to dollspace-gay#71).
3. **Kickoff permission posture changed since the attended-design page was written.** That page's mismatch list said only `container start` passes `--dangerously-skip-permissions`. Now `kickoff run`/`launch` carry explicit `--skip-permissions` and `--permission-mode` flags, and mirror HEAD (PR #72, gh#66) extends them to `kickoff plan` for headless dispatch — not yet in the installed binary. This is the #837/retest-trigger surface moving.
4. **`swarm review` still emits a plan and does not launch agents** — source-confirmed at mirror HEAD (plan JSON to hub, assignments printed; `swarm pipeline` is the driver that launches review→fix stages). The estate's 0.8.0-era reading holds; the fallback shape (vsdd injecting composition into the dispatch vehicle's prompt) remains valid.
5. **`commands.qmd` doc drift:** documents `issue search/show --from <repo>` — the actual flag is `--repo` everywhere. Also documents the hidden top-level shortcuts the binary's `--help` omits.
6. **Binary-vs-mirror skew to expect on rebuild:** kickoff plan permission flags (above). `knowledge list/search --since` and `knowledge show --repo` are already in the installed binary (contrary to what a stale reading might assume — verified from raw help).
7. **Estate hygiene finding:** `crosslink knowledge list` (network path) currently fails in vsdd-cli — `git fetch origin crosslink/knowledge` dies on `bad object refs/remotes/origin/crosslink/agents/xqjG 2`, a corrupted ref in the knowledge cache; cache-local `knowledge search` still works. Needs a cache rebuild (the caches are documented safe to delete).
8. **Not previously recorded anywhere in the estate:** the `token_usage` table (migration v15) + swarm budget subfamily + `context measure` as an already-built cost substrate; the `agent request/poll-requests/flags` control plane; `template_required_fields`; `issue --scheduled/--due`; the `repo-alias`/external-cache cross-repo read federation; `integrity sign-backfill` as the retroactive attestation path.

