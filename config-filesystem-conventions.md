---
title: "config-filesystem-conventions"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
---


## Design Specification

### design specification

### 1. write-scope policy: repo-scoped by default; narrow, non-`.claude` $home exceptions

The core issue tracker writes **exclusively repo-scoped** paths: `.crosslink/` and `.claude/` *inside the repo*, plus git worktrees. The only writes outside the repo belong to two peripheral features (the multi-project **dashboard** and the **opt-in shell alias**), and they target `~/.crosslink/` and shell rc files — **never** `~/.claude`, `~/.claude.json`, or `~/.gitconfig`.

Verdict table:

| Target | Read? | Write? | Site |
|---|---|---|---|
| `~/.claude.json` | no | **no** | (no references anywhere in `src/`) |
| `~/.claude/` | read-only | **no** | container mounts `~/.claude/.credentials.json` `:ro` for auth — `commands/container.rs:325-334,~361` |
| `~/.gitconfig` | read-only | **no** | `git config --global user.signingkey` (read) — `signing.rs:244-248` |
| `~/.ssh/` | read-only | no | key discovery — `signing.rs:229-240`; tilde-expand — `sync/trust.rs:14-45` |
| `~/.config/…`, `~/.zshrc`, `~/.bashrc` | read (detect) | **write only if user opts in** | detect: `commands/config.rs:1132-1162`; append: `commands/init/walkthrough.rs:651-690` |
| `~/.crosslink/dashboard.db` | yes | **yes** | `dashboard/db.rs:53-61`, `resolve_home_dir` `dashboard/db.rs:174` |
| `~/.crosslink/.dashboard-token` | yes | **yes** (best-effort) | `server/state.rs:83-99`, `token_path` `server/state.rs:114-117` |
| `$HOME/<owner>/<repo>` (git clones) | walk `$HOME` to discover: `dashboard/discover.rs:93-106` | **yes** (dashboard clone) | `dashboard/github_api.rs:245-248`, `dashboard/api.rs:606` |

Bottom line: **no precedent exists for writing `~/.claude`, `~/.claude.json`, or `~/.gitconfig`.** Those are read-only auth/signing inputs. The only user-global *writes* are `~/.crosslink/` state (dashboard) and — only on explicit opt-in — shell-rc alias lines.

### 2. where state lives

- **Git refs (hub v3) = source of truth.** Each agent writes only its own append-only ref `refs/heads/crosslink/agents/<id>`; shared `checkpoint` and `meta` refs — `hub_v3.rs:37-49`. Legacy `refs/crosslink/*` — `hub_v3.rs:89-91`, `hub_source.rs:1098`.
- **SQLite `.crosslink/issues.db` = local materialization / hydration cache**, rebuilt from refs. Opened at `main.rs:2256`, created in init `commands/init/mod.rs:973-974`. `issues.db-wal`/`-shm` are machine-local (gitignored — `commands/init/merge.rs:14-16`).
- **`~/.crosslink/dashboard.db`** — separate SQLite for the cross-project dashboard aggregator (`dashboard/db.rs`, schema `dashboard/db.rs:71+`). Distinct from the per-repo tracker DB.
- **JSON sidecars** in `.crosslink/`: `agent.json`, `session.json`, `hook-config.json`, `hook-config.local.json`, `init-manifest.json`, `promotion-log.json`, `hub-v3-shadow-stats.json`.

### 3. config layering (`hook-config.json` vs `.local` vs `agent_overrides`)

Resolution precedence (lowest→highest), in `read_config_layered` — `commands/config.rs:57-132`:
1. **Embedded defaults** — `HOOK_CONFIG_JSON` from `commands/config_registry.rs` (`read_defaults` `config.rs:184-186`).
2. **Team** — `.crosslink/hook-config.json` (tracked/committed). Overlay `config.rs:87-96`.
3. **Local** — `.crosslink/hook-config.local.json` (gitignored, per-machine). Overlay `config.rs:99-124`.

Each merged key carries provenance `Source::{Default,Team,Local}` (`config.rs:36-55,82-119`). Local supports **`+key` array-extend semantics**: a `+foo` key appends non-duplicate items onto the base `foo` array rather than replacing it — `config.rs:104-116`.

Writes are scope-explicit via `WriteScope::{Team,Local}` → filename `hook-config.json` / `hook-config.local.json` (`write_config_scoped` `config.rs:170-182`); default write scope is Team (`config.rs:166-168`).

**`agent_overrides`** (`agent_lint_commands`, `agent_test_commands`) lets kickoff agents self-validate. Auto-populated at init by `populate_agent_tool_commands` — but **only fills empty arrays**, never overwrites manual config — `commands/init/mod.rs:428-484`.

### 4. managed-file / init update semantics

**Managed set** = all init-deployed files: `.claude/hooks/*.py`, `.claude/mcp/*.py`, `.claude/commands/*`, `.claude/skills/**`, `.crosslink/rules/*`, and `.claude/settings.json` — enumerated in `managed_files` `commands/init/mod.rs:491-552`. For `settings.json` the tracked content is the template *after* `__PYTHON_PREFIX__` substitution but *before* the `allowedTools` merge, so user tool additions never register as "modified" (`init/mod.rs:547-549`, `manifest.rs:100-105`).

**Manifest** — `.crosslink/init-manifest.json` records SHA-256 of each managed file (`commands/init/manifest.rs:17-30`). `crosslink init --update` does a **three-way compare** (manifest hash vs on-disk hash vs new template hash) via `classify_update` — `manifest.rs:136-152`:
- unchanged/unchanged → `UpToDate`; user-untouched + template changed → `AutoUpdate`; user-modified + template same → `TemplateUnchanged` (skip); both changed → `Conflict` (prompt, or skip under `--no-prompt`/non-TTY); missing on disk → `Deleted` (never recreated). Applied in `run_update` — `init/mod.rs:555-800` (auto-apply `711-740`, conflict prompt `742-773`). Missing/corrupt manifest → treat all as potentially-modified and warn (`init/mod.rs:580-587`, `manifest.rs:77-81`).

**Adopter-customization preservation** across init/update:
- `settings.json`: `write_settings_json_merged` parses existing into a `serde_json::Map`, **preserves every unknown key**, unions `allowedTools` (existing ∪ embedded, dedup), and overwrites *only* the crosslink-managed `hooks` and `enableAllProjectMcpServers` keys — `commands/init/merge.rs:159-234`. Re-run on auto-update/conflict-accept rather than blind copy — `init/mod.rs:715-717,732-733,762-763`.
- `.mcp.json`: `write_mcp_json_merged` preserves unknown top-level keys and existing `mcpServers` entries, inserting crosslink servers and **warning** (not failing) on collision — `merge.rs:96-152`.
- Root `.gitignore`: `write_root_gitignore` maintains a **managed block between sentinel markers** (`GITIGNORE_SECTION_START/END`, `merge.rs:8-10`), replacing the block in place and preserving all surrounding user lines — `merge.rs:53-91`.
- `tracker_remote`: `populate_tracker_remote` is idempotent and preserves manual values across `--force` (GH#586) — `init/mod.rs:1004-1008` (writer `~300-394`).
- Inner `.crosslink/.gitignore` written only if absent or `--force` — `init/mod.rs:1010-1036`.

### 5. sentinel-file conventions (machine-local; all gitignored)

Listed for gitignore in `merge.rs:12-36` and `init/mod.rs:1015-1033`:
- **`.crosslink/.active-issue`** — active issue id for fast hook lookup (`work-check.py` reads it instead of shelling out; ~1ms vs ~100ms). Write/clear are best-effort — `commands/session.rs:27,33-42`.
- **`.crosslink/.last-hydrated-ref`** — hub HEAD high-water mark enabling lazy auto-hydration; compared in `maybe_auto_hydrate` (`hydration.rs:1170-1206`), recorded by `record_hydrated_ref` (`hydration.rs:1212-1217`). Both writes best-effort (`let _ =`).
- **`<worktree>/.kickoff-status`** — agent lifecycle word `LAUNCHING`→`RUNNING`→`DONE`/`FAILED`/`CI_FAILED` (and `TIMEOUT` as of gh#60). Written by launcher `commands/kickoff/launch.rs:623-640` (best-effort on failure paths), swarm `commands/swarm/mod.rs:433-466`, agents themselves (`kickoff/prompt.rs:181`); read by monitor `kickoff/monitor.rs:37-193`, sentinel `sentinel/collect.rs:65`, server `server/handlers/agents.rs:294,351`.
- **`<worktree>/.kickoff-slug`** — records the feature slug in the worktree — `kickoff/run.rs:101-102`, `kickoff/plan.rs:179-180` (hard error via `.context()?`).
- **`.crosslink/daemon.pid` / `daemon.log`** — heartbeat daemon liveness — `daemon.rs:27,74,98`.
- Other machine-local sidecars: `last_test_run`, `.promoted-uuids`, `promotion-log.json`, `hub-v3-shadow-stats.json`, `sentinel.log` (gitignored, `init/mod.rs:1027-1033`).

### 6. write-safety idioms

- **Atomic write (canonical)** — `utils::atomic_write` — `utils.rs:140-195`: unique temp via `tempfile::Builder` **in the same dir** (avoids concurrent-writer clobber), `write_all` → `sync_all` (fsync) → `persist`/rename; then **best-effort** parent-dir fsync on Unix (WARN, non-fatal — `utils.rs:172-192`). Used for issue files (`issue_file.rs:213,432`) and compaction checkpoints (`checkpoint.rs:267`).
- **Manifest atomic** — separate tmp-write + `fs::rename` — `manifest.rs:84-96` (test asserts no lingering `.tmp` — `manifest.rs:222-233`).
- **Plain `fs::write` + trailing newline** — for pretty-printed JSON config that is not concurrency-sensitive: `write_config_scoped` (`config.rs:180-182`), settings/mcp merges (`merge.rs:150,232`), init hook-config (`init/mod.rs:984,990`), `agent_overrides` (`init/mod.rs:479-480`).

### 7. failure philosophy

- **Hard error (`?`/`bail!`/`.context`)** for durable config and managed files: merge functions **refuse to overwrite invalid or non-object JSON** rather than clobbering user data — `merge.rs:106-120,128-131,177-197`; every init `fs::write` is `.context(...)?`; `write_config_scoped`, `write_manifest`, `.kickoff-slug` all return `Result`.
- **Best-effort (`let _ =`, `.ok()`, `.is_ok()`, WARN)** for regenerable/ephemeral state: active-issue & hydrated-ref sentinels (`session.rs:35,41`, `hydration.rs:1203,1215`), dashboard token (`server/state.rs:93` ignores write error, degrades to in-memory-only per doc `server/state.rs:79-82`), `.kickoff-status` failure-path writes (`kickoff/launch.rs:634,640`), shell-alias append (warns on error, `walkthrough.rs:688`), and parent-dir fsync (`utils.rs:176-190`). Rationale: these are rebuilt from git refs / next run, so a failed write must not abort the primary operation.

### sources

- `src/utils.rs`
- `src/issue_file.rs`
- `src/checkpoint.rs`
- `src/hydration.rs`
- `src/daemon.rs`
- `src/signing.rs`
- `src/sync/trust.rs`
- `src/hub_v3.rs`, `src/hub_source.rs`
- `src/server/state.rs`, `src/server/handlers/agents.rs`
- `src/dashboard/db.rs`, `src/dashboard/discover.rs`, `src/dashboard/github_api.rs`, `src/dashboard/api.rs`
- `src/commands/config.rs`, `src/commands/config_registry.rs`
- `src/commands/session.rs`
- `src/commands/container.rs`
- `src/commands/init/mod.rs`, `src/commands/init/merge.rs`, `src/commands/init/manifest.rs`, `src/commands/init/walkthrough.rs`
- `src/commands/kickoff/{launch,run,plan,monitor}.rs`, `src/commands/swarm/mod.rs`, `src/commands/sentinel/collect.rs`

