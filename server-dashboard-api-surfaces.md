---
title: "server-dashboard-api-surfaces"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
---


## Design Specification

### design specification

### 1. router and handler architecture

`src/server/routes.rs:38` `build_router(state, dashboard_dir)` composes everything:

- Core REST API `Router` (`src/server/routes.rs:41-126`): health, agents/locks (`:44-50`), issues CRUD + comments/labels/blockers (`:52-73`), sessions (`:75-78`), milestones (`:80-83`), knowledge (`:85-90`), unified search (`:92`), sync (`:94-96`), config (`:98`), usage (`:100-101`), orchestrator (`:103-126`). Static paths are deliberately registered before `/{id}` captures (`:51`, `:84`, `:99`, `:102`).
- Nesting (`src/server/routes.rs:128-137`): the core router at `/api/v1`; four dashboard routers all at `/api/v1/dashboard` — `dashboard::api::build_router()` (`:130`), `dashboard::github_api::router()` (`:131`), `dashboard::export::router()` (`:132`), `dashboard::webhook_api::router()` (`:133`); PTY REST at `/api/v1` (`:134`) and PTY WS at `/ws` (`:135`); the event hub WS at `GET /ws` (`:136`).
- Asset fallback (`src/server/routes.rs:147-152`): `--dashboard-dir` serves from disk via `tower_http::services::ServeDir` (dev workflow); otherwise the embedded bundle via `super::embedded::serve_embedded`.

`src/server/mod.rs`:

- `MAX_BODY_SIZE` 10 MB (`src/server/mod.rs:24`), applied as `DefaultBodyLimit::max` (`:139`).
- `auth_middleware` (`src/server/mod.rs:31-55`): Bearer-token check against `state.auth_token`; exempts `/api/v1/health`, `/ws`, and anything not under `/api/` (`:39`). Layered over the whole router (`:134-138`).
- CORS (`src/server/mod.rs:119-128`): allows origins `http://localhost:5173` + `http://127.0.0.1:5173` (Vite dev server), any method, `Content-Type`/`Authorization`/`Accept` headers.
- `run` (`:69`) delegates to `run_with_dashboard_db` (`:85`); when a dashboard DB path is present it spawns the 5-second poll loop wired to `state.ws_tx` (`:99-110`), starts the heartbeat watcher (`:113`), binds `127.0.0.1:<port>` (`:142`), and prints the dashboard URL with `?token=` baked in (`:148-160`); the frontend persists the token to sessionStorage (`:144-147`).

`src/server/embedded.rs`: React `dashboard/dist/` embedded via `#[derive(RustEmbed)]` (`src/server/embedded.rs:23-25`, GH #429). `serve_embedded` (`:35`) returns 404 (never SPA HTML) for unmatched `api/`/`ws` paths (`:43-45`), serves exact assets with `mime_guess` (`:53-58`), and falls back to `index.html` for client-side routes (`:63-68`).

### 2. dashboard data model

### Snapshot types (`src/dashboard/reader.rs`)

- `HubSnapshot` (`src/dashboard/reader.rs:31-60`): `hub_sha`, `layout_version`, `issues: Vec<IssueFile>`, `agents: Vec<Heartbeat>`, `locks: Vec<LockRecord>`, `agent_requests: Vec<AgentRequestsForAgent>`, `ci_status: Option<CiStatus>`, `signature_state: SignatureState`, `last_commit_at`.
- `AgentRequestsForAgent` (`:63-67`) wraps `crate::agent_requests::RequestWithAck` per target agent.
- `CiStatus` (`:77-83`): `{sha, state: "passing|failing|pending", url?}`; doc comment (`:69-76`) records the v2 contract — pipelines write `meta/ci-status.json` on the hub branch as a post-build hook.
- `SignatureState` (`:88-94`): `Valid | Unsigned | Invalid | Unknown` (wire strings at `:96-106`).
- `LockRecord` (`:109-113`), `ProjectCounters` (`:588-596`), `HubSnapshot::derive_counters` (`:612-667`) — open/overdue/due-soon/blocked issues, active agents (heartbeat window), stale locks.

### Mode routing: v2 file-scan vs v3 ref-read

`read_snapshot` (`src/dashboard/reader.rs:133`) mode-routes per project (`:144-152`): if `crate::hub_v3::HubMode::resolve(clone_path).is_v3()` (`src/hub_v3.rs:709`), freshness keys come from `CHECKPOINT_REF` (`reader.rs:149-150`, GH#4) and it takes `read_snapshot_v3`; otherwise the v2 path resolves `crosslink/hub`, prefers the `.crosslink/.hub-cache/` worktree via `resolve_hub_root` (`:404-420`), and file-scans: `read_all_issue_files` (`:170-174`), `read_agent_heartbeats` (`:466`), `read_locks` v2-dir/v1-json (`:498-549`), `read_agent_requests` (`:424-458`), `read_ci_status` (`:342-356`), `read_signature_state` (`:362-383`, mode-agnostic, via `SyncManager::verify_hub_signature_auto`).

`read_snapshot_v3` (`src/dashboard/reader.rs:212-267`):

- Issues + locks from the reduced `CheckpointState` read off `CHECKPOINT_REF`'s `state.json` blob — `read_checkpoint_state` (`:272-283`) via `git_rev_parse_optional` + `git_cat_file_blob_optional`, defaulting to empty when absent; `compact_issue_to_file` (`:289-337`) maps `CompactIssue` back to the `IssueFile` shape.
- Heartbeats from `crate::hub_v3::read_heartbeats_from_refs` (`reader.rs:236-242`; `src/hub_v3.rs:853-874` — `heartbeat.json` at each agent-ref tip, same `crate::locks::Heartbeat` schema as v2).
- `agent_requests` from `crate::hub_v3::read_all_agent_requests` (`reader.rs:247-252`, GH#49 — previously hardcoded empty on v3), best-effort.
- `layout_version: 3` (`:258`), and **`ci_status: None`** (`:263`) — the doc comment (`:207-209`) states: "`meta/ci-status.json` has no v3 ref home and the v2 worktree path is gone... a v3 CI-status writer + home is separate work". This is the open gh#49 half.

### Poll loop (`src/dashboard/poll.rs`)

- `DEFAULT_TICK` 5 s (`src/dashboard/poll.rs:36`); thresholds: agent-active 10 min (`:40`), stale-lock 60 min (`:44`). `run`/`run_with_tick` (`:53-93`) loop under a `CancellationToken`.
- `poll_all_projects` (`:97-129`) walks active projects serially; per-project success emits `WsEvent::DashboardProjectUpdated` and, when alert counts changed, `WsEvent::DashboardAlertsChanged` (`:113-126`).
- `poll_project` (`:142-240`): (1) best-effort `fetch_hub` (`:147`); (2) `read_snapshot` on the blocking pool; a hard read failure calls `mark_project_status(db, id, "error")` (`:157-171`, GH#48 — this is what arms `unreachable_project`); (3) status = `"active"` iff fetch succeeded or local hub data exists (`:178-182`); (4) `derive_counters` + `derive_alerts`, then one blocking pass: `write_project_state` (`:202-219`, SQL at `:284-334` — upserts `project_state` incl. `ci_status` column and refreshes `projects.hub_sha/hub_fetched_at/last_activity_at/status`) + `alerts_db::sync_alerts_for_project`; (5) fire-and-forget webhook dispatch per newly-opened alert (`:224-234`).
- `mark_project_status` (`:273-280`) sets only `projects.status`.
- `fetch_hub` (`:336-385`): glob refspec `+refs/heads/crosslink/*:refs/heads/crosslink/*` (`:366`) so BOTH v2 `crosslink/hub` and the v3 refs (checkpoint, meta, agents/*) are fetched (GH#4); all credential prompts disabled so a dead remote fails fast (GH#34, `:355-363`); v2-only hub-cache worktree materialisation (`:381-383`, `ensure_hub_cache_worktree` `:406-475`, dirty-tree preserving per #701).

### Alerts (`src/dashboard/alerts.rs`)

Pure derivation `derive_alerts(project, snapshot, now)` (`src/dashboard/alerts.rs:79`); coverage table at `:12-27`. Kinds and keys:

| Kind | Keys on | Severity | Code |
|---|---|---|---|
| `unreachable_project` | `project.status == "error"` (set by poll GH#48) | warning | `alerts.rs:89-96` |
| `stale_lock` | lock `claimed_at` older than 60 min (`STALE_LOCK_MINUTES` `:72`) | warning | `:113-127` |
| `silent_agent` | agent holding a lock with heartbeat silent >10 min (`SILENT_AGENT_MINUTES` `:73`) | critical | `:106-111`, `:131-141` |
| `overdue_issue` | open issue with `due_at < now`, one alert per issue | warning | `:147-168` |
| `ci_failure` | `snapshot.ci_status.state == "failing"`, subject `commit:<sha>` | warning | `:174-187` |
| `signature_invalid` | `SignatureState::Invalid` only (not Unsigned/Unknown) | critical | `:194-207` |
| `orphan_subissue` | open child whose parent is closed | info | `:213-242` |

Identity key for DB reconcile is `(kind, subject_ref)` (`:56-68`); sync lives in `alerts_db.rs`.

### Projects CRUD and dashboard API

- `src/dashboard/projects.rs`: `Project` row struct (`:28`), `WriteCapability` (`:52`, probed by `write_capability` `:93`), `track`/`track_with_init`/`track_at_path` (`:271`, `:284`, `:517`), `untrack` (`:613`/`:625`), `list` (`:657`/`:666`).
- `src/dashboard/api.rs` `build_router` (`:38-82`): reads `GET /projects` (`:45`, handler `:229`), `GET /projects/{*slug}` wildcard because slugs contain `/` (`:46`, handler `:300` — serves a live `HubSnapshot` incl. `agent_requests`, `ci_status`, `signature_state`; wire types `ProjectDetail` `:118-148`, `SerializableAgentRequests` `:151-201`), `GET /alerts` (`:47`, `:465`), `POST /clone` (`:48`, `:571`). Writes are namespaced under `/w/{owner}/{repo}/…` (`:39-43` explains the prefix): issue close/reopen/comment/block/unblock/relate/label/unlabel (`:49-56`), milestones (`:57-69`), locks claim/release/steal (`:70-72`), `agents/{agent_id}/request` (`:73-76`), `init` (`:77`), `integrity/sign-backfill` (`:78-81`).

### `handlers/sync.rs` `last_fetch_at`

`sync_status` (`src/server/handlers/sync.rs:62-114`): on v2 the hub-cache worktree directory mtime tracks fetches; on v3 the fetch adopts refs into `.git` and never touches a worktree (GH#53), so it uses the `FETCH_HEAD` mtime instead (`fetch_head_mtime`, derivation at `:84-99`). `hub_branch` reports `crosslink/checkpoint` vs `crosslink/hub` by mode (`:104-108`, GH#4). `sync_push` (`:158`) reduces to fetch+refresh on v3 — agents push their own refs at write time (`:147-153`).

### 3. gh#49 ci_status: v2 reality, v3 gap

**v2**: reader loads `meta/ci-status.json` from the hub worktree (`src/dashboard/reader.rs:342-356`) and drops it unless `status.sha == hub_sha` (stale gate; a missing `hub_sha` accepts at face value, `:349-355`). The writer was never crosslink — "pipelines... write that file as part of their post-build hook" (`:70-72`). Downstream: `project_state.ci_status` column (`poll.rs:197`, `:306-331`), `ci_failure` alert (`alerts.rs:174-187`), API `ProjectCountersView.ci_status` (`api.rs:110`) and `ProjectDetail.ci_status` (`api.rs:141`).

**v3**: no worktree exists; `read_snapshot_v3` hardcodes `ci_status: None` (`reader.rs:263`) and no writer exists anywhere in the crate.

**Candidate v3 homes**:

1. Checkpoint `state.json` — rejected. `CheckpointState` (`src/checkpoint.rs:17-52`) is a deterministic reduction of agent-ref events with a watermark; concurrent compaction safety depends on "the same event set reduces to the same state" (`src/hub_v3.rs:41-44`). An out-of-band CI field would break byte-identical convergence, and CI is not an event author.
2. An agent-ref event — rejected. Requires the pipeline to own an agent identity + ref (`AGENT_REF_PREFIX`, `src/hub_v3.rs:37`) and the reducer has no CI concept; disproportionate for an external, non-agent writer.
3. **A `ci-status.json` blob at the tree root of `META_REF` — recommended.** `META_REF` (`src/hub_v3.rs:47-49`) already holds exactly this class of data: hub-global, non-event metadata (`hub.json` + `allowed_signers`), written at bootstrap via the sibling-preserving commit core (`bootstrap_v3_hub`, `src/hub_v3.rs:1899-1941`) and read via rev-parse + `cat-file {tip}:hub.json` (`read_hub_meta`, `src/hub_v3.rs:780-791`). It is the structural successor of the v2 `meta/` directory.

### gh#49 ci_status v3: recommended design

**Home**: `ci-status.json` at the tree root of `META_REF` (`refs/heads/crosslink/meta`), reusing the existing `reader::CiStatus` wire schema (`reader.rs:77-83`) plus an optional `recorded_at` timestamp.

**Writer**: a new subcommand — e.g. `crosslink ci status set <passing|failing|pending> --sha <sha> [--url <url>]` — that CI pipelines invoke instead of dropping a worktree file. Implementation is a thin composition of existing primitives: `commit_upserts_to_ref(repo_dir, META_REF, &[("ci-status.json", BlobRef::Bytes(..))], &[], msg, "ci", CasExpectation::CurrentTip)` (`src/hub_v3.rs:2289-2310` — THE shared sibling-preserving core; `hub.json`/`allowed_signers` survive byte-identical), then `push_ref` / `push_ref_with_lease` (`src/hub_v3.rs:447`, `:481`). CAS + lease handles concurrent meta writers exactly as the existing meta/checkpoint writes do.

**Reader**: in `read_snapshot_v3` (`reader.rs:212`), mirror the `read_hub_meta` / `read_all_agent_requests` (GH#49 sibling fix, `src/hub_v3.rs:1085-1147`) pattern: `git_rev_parse_optional(META_REF)` → `git_cat_file_blob_optional("{tip}:ci-status.json")` → parse `CiStatus`, best-effort (`None` on absence, like the agent-requests degrade at `reader.rs:247-252`). Staleness: the v2 `sha == hub_sha` gate does not transplant — on v3 `hub_sha` is the checkpoint tip, which CI cannot know. Keep `sha` as the tested source commit (informational, surfaced in the alert detail exactly as today, `alerts.rs:176-178`) and gate staleness on `recorded_at` age (e.g. drop entries older than a configurable window), falling back to face-value acceptance matching the v2 `None`-hub_sha arm (`reader.rs:352`).

**Why this is zero-cost downstream**: the poll loop's glob refspec `+refs/heads/crosslink/*` (`poll.rs:366`) already fetches `META_REF` every tick, so the dashboard observes writes with no fetch changes; `poll.rs:197`, the `project_state.ci_status` column, the `ci_failure` alert, and both API payload fields consume `snapshot.ci_status` unchanged.

### 4. websocket, pty, github, export, webhook surfaces

- `src/server/ws.rs`: single `tokio::sync::broadcast` hub, capacity 256 (`:36`). `WsEvent` variants (`:49-60`); channel mapping (`:65-73`): `agents` (Heartbeat/AgentStatus), `issues`, `locks`, `execution`, `dashboard` (DashboardProjectUpdated/DashboardAlertsChanged). Every frame is wrapped in `WsEnvelope` with a per-connection monotonic `seq` (`:100-106`); clients may send a `subscribe` message to filter channels (`:137-158`); buffer overflow emits a synthetic `gap` message (`:92-98`).
- `src/dashboard/pty_api.rs`: `POST /api/v1/pty` spawn, `GET /api/v1/pty/sessions` list (`:36-40`); `WS /ws/pty/{session_id}` (`:42-44`) with replay-on-connect + broadcast so multiple tabs share one PTY (`:10-12`).
- `src/dashboard/github_api.rs` (`:30-34`): `GET|POST /github/config`, `GET /github/orgs/{org}/repos`, `POST /github/orgs/{org}/track-all`.
- `src/dashboard/export.rs` (`:32-37`): `GET /export/projects.{csv,json}`, `GET /export/alerts.{csv,json}`.
- `src/dashboard/webhook_api.rs` (`:31-32`): `GET|PUT /webhooks` (URL list consumed by `poll.rs:224-234` alert dispatch).

### 5. renderer drift surfaces (gh#16, gh#17)

- **gh#16 — session status dual hand-built renderers**: `src/commands/session.rs:141-234` `status()` contains two independently-maintained renderers. JSON branch (`:160-183`) emits `active, session_id, started_at, duration_minutes, agent_id, working_on{id,display_id,title}, last_action`. Human branch (`:185-231`) prints session/working-on/last-action/duration AND an activity summary (`count_issues_since` / `count_comments_since`, `:211-231`) that the JSON branch omits; conversely the human branch never prints `agent_id`. Any new field must be added twice. `src/main.rs:95` documents `--json` as supported by "list, show, search, session status". A third session-shaped renderer exists server-side: `GET /api/v1/sessions/current` (`src/server/handlers/sessions.rs:35-52`) serializes the raw DB `Session`.
- **gh#17 — no combined session+work+lock machine query**: session-status JSON (`session.rs:160-183`) has session + active issue but no lock state; lock JSON is a separate whole-file dump (`crosslink locks list --json`, `src/commands/locks_cmd.rs:23-32`); the server likewise splits `GET /sessions/current` (`routes.rs:75`) from `GET /locks` (`routes.rs:48`). Consumers needing "who am I, what am I working on, do I hold the lock" must join client-side; the hook workaround is the `.active-issue` sentinel file (`session.rs:27-42`, #522).

### sources

- `src/server/routes.rs` — build_router, nesting, fallback (`:38-155`)
- `src/server/mod.rs` — auth_middleware, CORS, DefaultBodyLimit, run (`:24-173`)
- `src/server/embedded.rs` — RustEmbed SPA fallback (`:23-71`)
- `src/dashboard/reader.rs` — snapshot types, mode routing, read_snapshot_v3, read_ci_status (`:31-667`)
- `src/dashboard/poll.rs` — poll loop, mark_project_status, fetch_hub glob refspec (`:36-475`)
- `src/dashboard/alerts.rs` — alert kinds table + derivation (`:12-245`)
- `src/dashboard/api.rs` — dashboard router + wire types (`:38-225`)
- `src/dashboard/projects.rs` — track/untrack/list, WriteCapability (`:28-737`)
- `src/server/handlers/sync.rs` — last_fetch_at v2/v3 split (`:62-114`)
- `src/hub_v3.rs` — META_REF/CHECKPOINT_REF/AGENT_REF_PREFIX, read_hub_meta, read_all_agent_requests, commit_upserts_to_ref, bootstrap_v3_hub, push_ref(_with_lease) (`:37-49`, `:447-515`, `:780-791`, `:1085-1147`, `:1899-1941`, `:2289-2310`)
- `src/checkpoint.rs` — CheckpointState (`:17-52`)
- `src/server/ws.rs`, `src/dashboard/pty_api.rs`, `src/dashboard/github_api.rs`, `src/dashboard/export.rs`, `src/dashboard/webhook_api.rs` — WS/PTY/GitHub/export/webhook routers
- `src/commands/session.rs` (`:141-234`), `src/commands/locks_cmd.rs` (`:23-32`), `src/server/handlers/sessions.rs` (`:35-52`) — gh#16/gh#17 surfaces

