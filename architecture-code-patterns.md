---
title: "architecture-code-patterns"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
---


## Design Specification

### design specification

### 1. module / command architecture

**Dual module tree.** The crate ships a `[lib]` and a `[bin]` from the same sources (`Cargo.toml:23-29`). `src/lib.rs:5-37` re-exports modules `pub` for fuzz/test consumers; `src/main.rs:45-77` re-declares the same modules privately for the binary. This duplication is deliberate and is why plumbing code carries `#[allow(dead_code)]` for constructors "the bin's duplicate module tree flags as dead code" (`hub_v3.rs:56-60`).

**Clap dispatch.** `src/main.rs` (3590 lines) is the single dispatch surface. `Cli` (`main.rs:86-114`) is a `#[derive(Parser)]` with four global flags — `--quiet`, `--json`, `--log-level` (env `CROSSLINK_LOG`), `--log-format` (env `CROSSLINK_LOG_FORMAT`). Version resolves `option_env!("CROSSLINK_VERSION")` with a `CARGO_PKG_VERSION` fallback (`main.rs:89`). The `Commands` enum (`main.rs:116`) drives one large `match` (`main.rs:2703+`). Issue verbs are nested under `IssueCommands` and dispatched by the `dispatch_issue` helper (`main.rs:2745`, arms at `main.rs:2334-2692`); top-level ergonomic aliases (`Commands::Create/List/Show/New/Subissue`, `main.rs:2793-2984`) forward into the same `IssueCommands` variants so there is one implementation per verb.

**Command modules.** `src/commands/mod.rs` is a flat `pub mod` list of ~50 command modules. Renames are documented inline, e.g. `lifecycle` "renamed from status.rs (#448)" (`commands/mod.rs:28-30`).

**Nested sub-command packages.** Complex commands are directories with a `mod.rs` that owns a local `dispatch()` plus focused sibling files. `kickoff/` is the canonical example: `mod.rs` declares `run, plan, launch, monitor, helpers, pipeline, cleanup, graph, prompt, types, wizard, tests`, re-exports the public surface (`kickoff/mod.rs:18-36`), and holds `pub fn dispatch(command, ...)` (`kickoff/mod.rs:45`). The same shape recurs in `swarm/`, `sentinel/` (with a further `sources/` sub-package), `init/`, `knowledge/`, `db/`, `sync/`, `dashboard/`, `server/handlers/`, `orchestrator/`, `shared_writer/`, `tui/`. Files are kept small and single-concern (see `quality.md`: "Split at ~200 lines").

**Where shared logic lives.** `db/` (SQLite access, split by entity: `issues.rs`, `comments.rs`, `labels.rs`, `relations.rs`, `milestones.rs`, `hydration.rs`); `shared_writer/` (the write path); `sync/` (`SyncManager`); `events.rs` + `compaction.rs` + `hub_v3.rs` + `hub_source.rs` + `checkpoint.rs` (event core); `utils.rs` (shell escaping, id generation, fsync helpers); `models.rs`, `identity.rs`, `signing.rs`.

### 2. error-handling doctrine

**anyhow everywhere.** Command and library fns return `anyhow::Result`; the canonical import is `use anyhow::{bail, Context, Result}` (`main.rs:79`, `kickoff/mod.rs:38`, `hub_v3.rs:20`). Counts in `src/`: ~393 `bail!`, ~562 `.context()/.with_context()`, ~299 `let _ =`.

- **`bail!`** for validated hard failures with an actionable message (`kickoff/mod.rs:245` `bail!("--run requires a design document...")`, `launch.rs:249` Windows-tmux guard).
- **`.with_context()`** wraps I/O with the offending path (`kickoff/mod.rs:69-70` "Failed to read design doc: {path}"; `hub_source.rs:121` "Failed to read agents dir: {}").
- **`let … else { bail! }`** for cheap guard-clause control flow (`kickoff/mod.rs:243-245`).

**Best-effort vs hard error.** Roughly 80 sites carry an explicit `INTENTIONAL:` / `best-effort` comment justifying a swallowed error, always paired with a `let _ =` and usually a `tracing::warn!`. The load-bearing rule is the status sentinel: `launch.rs:633,639` — `// INTENTIONAL: status file write is best-effort — used for monitoring, not control flow` (a failed `.kickoff-status` write must not abort a launch). Other exemplars: `lock_check.rs:41` (offline fetch), `signing.rs:809` (kill on timeout), `tui/mod.rs:800,813` (terminal restore in panic-hook/`Drop` must never itself panic), `server/handlers/issues.rs:46` (WebSocket broadcast is best-effort), `knowledge/sync.rs:179-212` (fetch/push/rebase-abort best-effort). Contract: git/network/monitoring side effects degrade gracefully; anything that would corrupt the event log or mislead the caller is a hard error.

**Clippy regime.** Crate-level `#![warn(clippy::pedantic, clippy::nursery)]` (`lib.rs:2`) with a curated block of scoped `#![allow(...)]`, each with a one-line justification (`lib.rs:3-43`) — e.g. `missing_errors_doc`/`missing_panics_doc` allowed off "at scale," casts reviewed at write time. CI and `justfile` enforce a stricter gate: `cargo clippy -- -D warnings -W clippy::unwrap_used -W clippy::expect_used` (`.github/workflows/ci.yml:56`, `justfile:209`) plus `cargo fmt --all -- --check` (`ci.yml:53`, `justfile:201`). **What it forces:** `-D warnings` makes every pedantic/nursery lint fatal; the `unwrap_used`/`expect_used` warns push non-test code to combinators — `.unwrap_or_default()` (`utils.rs:220`), `.unwrap_or("unknown")` (`kickoff/mod.rs:91`), `.map_or_else(...)` (`kickoff/mod.rs:101`, monitor display code), `.ok_or_else(|| anyhow!(...))` (`kickoff/mod.rs:149,389`), and `let … else`. `unwrap()`/`expect()` survive almost exclusively in `#[cfg(test)]` code and harnesses.

### 3. the event-sourced hub (v2 vs v3)

**v2 — worktree JSON on `crosslink/hub`.** `SyncManager` (`sync/core.rs:11-30`) checks out the `crosslink/hub` branch into `.crosslink/.hub-cache/` and materializes `issues/*.json`. `V2_HUB_BRANCH = "refs/heads/crosslink/hub"` (`hub_v3.rs:563`). The v2 **write** path is retired: mutations on a v2 hub are refused with a migrate prompt (`shared_writer/mutations.rs:13`).

**v3 — per-agent event refs.** Each agent writes only to `refs/heads/crosslink/agents/<id>` (`AGENT_REF_PREFIX`, `hub_v3.rs:37`) using git plumbing (`hash-object`, `mktree`, `commit-tree`, `update-ref`) with no index and no worktree; a fast-forward push *is* the compare-and-swap (`hub_v3.rs:1-13`, `CasExpectation` enum `hub_v3.rs:56-69`). Two singleton refs back the hub: `CHECKPOINT_REF = refs/heads/crosslink/checkpoint` (reduced `state.json`, pushed `--force-with-lease`; concurrent compactions are byte-identical so the lease loser just refetches — `hub_v3.rs:39-45`) and `META_REF = refs/heads/crosslink/meta` (version marker `hub.json` + `allowed_signers` trust store — `hub_v3.rs:47-49`). `HubMode` (`hub_v3.rs:576`: `V2Only` / `V3 {v2_branch_present}` / `Absent`) is resolved once and cached in `SyncManager.hub_mode` as a `Cell` so hot paths don't re-probe refs (`sync/core.rs:22-30`).

**Event / CRDT model (`events.rs`).** Per-agent logs are append-only NDJSON at `agents/{id}/events.log` (`events.rs:1-6`). Every record is an `EventEnvelope` (`agent_id, agent_seq, timestamp, event, signed_by?, signature?` — `events.rs:41-50`). Total order is the `OrderingKey = (timestamp, agent_id, agent_seq)` (`events.rs:21-26`), giving deterministic convergence regardless of read order. The `Event` enum (serde `#[serde(tag = "type")]`, `events.rs:70-72`) is tiered (`events.rs:52-69`): **T1** identity/exclusive events resolve *first-claim-wins* by `OrderingKey` (`IssueCreated`, `LockClaimed`, `LockReleased`, `MilestoneCreated`); **T2** causal events are last-writer-wins or set-union (`IssueUpdated`, `StatusChanged`, dependency/relation/label/parent edits, `CommentAdded`, `TimeEntryAdded`, `ScheduleChanged`, ...). `IssueDeleted` is a tombstone that "wins forever" — no later event can resurrect it (`events.rs:64-69,209-214`). I/O helpers: `append_event`, `read_events`, `read_events_after`, `sign_event`, `verify_event_signature` (`events.rs:393/423/459/488/505`).

**Reduction / compaction.** `compaction::reduce(&dyn HubSource)` (`compaction.rs:214`) is a *pure*, I/O-agnostic fold: read checkpoint + watermark, collect events per agent, `sort_by_cached_key(OrderingKey::from_envelope)` for determinism (`compaction.rs:254`), then per event `detect_clock_skew` + `check_unsigned` + `apply`, advancing the watermark to the last event (`compaction.rs:276-290`), returning a `ReductionOutcome` (state + changed-id sets, `compaction.rs:173`). `compact()` (`compaction.rs:329`) is the write side and uses a **two-lock model**: a mandatory same-machine `HubWriteLock` passed as a proof parameter, plus a cross-machine `CompactionLockGuard` lease (`compaction.rs:305-313`). The tombstone guard in the apply path skips any event referencing a deleted issue (`compaction.rs:501-538,1005`).

**HubSource abstraction (`hub_source.rs`).** The read path is a trait (`hub_source.rs:49-83`: `agent_ids`, `read_events`, `read_checkpoint`, `read_legacy_watermark`, `allowed_signers_file`) with two impls: `WorktreeSource` (on-disk hub-cache, current prod, byte-for-byte compatible — `hub_source.rs:100-130`) and `ObjectStoreSource` (reads a committed tree via `git ls-tree`/`cat-file`, no checkout, pinned to a resolved commit SHA so concurrent pushes cannot produce a torn view — `hub_source.rs:9-25`).

**SQLite ↔ git.** SQLite is a *derived cache*: "keeps SQLite as the universal read path while JSON on the git branch remains the source of truth" (`hydration.rs:1-6`). After every successful mutation the writer appends events, pushes, then re-hydrates: `mutations.rs` builds a `WriteSet` of `Event`s, routes it through `SharedWriter::write_commit_push` (append to the agent ref + FF push), and calls `hydrate_with_retry(db)` off the reduced `CheckpointState` (`mutations.rs:4-13,199,331,482,508`). `hydrate_to_sqlite` clears shared tables via `clear_shared_data` (sessions preserved as machine-local, `db/hydration.rs`) and re-inserts in one transaction (`hydration.rs:122`). `SharedWriter::new` returns `Option`: `None` in single-agent mode falls back to direct `Database` writes (`shared_writer/mod.rs:1-8`).

### 4. test conventions

**Unit tests in-module.** Nearly every core module has `#[cfg(test)] mod tests` (`compaction.rs:4431+` tombstone/LWW cases; `hub_source.rs` tests). Larger suites live in dedicated test-only modules gated `#[cfg(test)]`: `db/tests.rs`, `shared_writer/tests.rs`, `sync/tests.rs`, `knowledge/tests.rs`, `kickoff/tests.rs`, `commands/hub_v3_operation_tests.rs`.

**CLI integration harness (`tests/cli_integration.rs`).** `run_crosslink(dir, args)` spawns `env!("CARGO_BIN_EXE_crosslink")` with `current_dir(dir)` and returns `(success, stdout, stderr)` (`cli_integration.rs:5-16`); `run_crosslink_info` prepends `--log-level info` to surface `tracing::info!` (`:19-32`). `test_dir()` builds a **hermetic git repo** per test — `git init` then `git -c user.name=test -c user.email=test@test commit --allow-empty` (identity via `-c` so it works with no global config), required because `crosslink init` verifies `.git` exists (#401) (`:36-63`). No live network.

**Smoke harness (`tests/smoke/harness.rs`).** `SmokeHarness` (`harness.rs:37-53`) gives each test its own `TempDir`, a **bare git remote** for hub coordination, and optional server lifecycle. `new()` runs `git init --bare` remote + work repo, configures identity, commits, pushes, then `crosslink init --defaults --skip-cpitd --skip-signing` (`harness.rs:66-153`). `fork_agent(id)` clones a second agent off the same bare remote for multi-agent concurrency/lock tests (`harness.rs:325-405`). `run`/`run_ok`/`run_err` assert exit codes (`harness.rs:177-216`); `start_server` binds `127.0.0.1:0` to grab a free port, parses the bearer token from stdout, and polls TCP for readiness (`harness.rs:236-302`); `Drop` kills the server (`harness.rs:408`). Suites: `adversarial`, `cli_data`, `cli_infra`, `cli_tooling`, `concurrency`, `coordination`, `lifecycle`, `server_api`, `tui_proptest`.

**gh#34 env-scoped git isolation.** The "never prompt / never hang" discipline lives in production code, not the harness: advisory git queries disable every credential vector — `.env("GIT_TERMINAL_PROMPT", "0")`, `.env("GIT_ASKPASS", "")`, `-c credential.helper=` — so a private/moved remote fails fast on a headless host (`dashboard/projects.rs:208-219`, `poll.rs:349`; empty-identity handling `sync/core.rs:281-381`). Tests inject `GIT_AUTHOR_*`/`GIT_COMMITTER_*` (and `GIT_COMMITTER_DATE`) env for deterministic commits (`hub_source.rs:886-889`, `seam.rs:834-837`, `clock_skew.rs:316-317`).

**proptest & fuzz.** `proptest` + `arbitrary` are dev-deps (`Cargo.toml:64-65`). `proptest!` blocks live across `models.rs`, `locks.rs`, `identity.rs`, and `commands/{export,list,import,create,update,relate,tree}.rs`, plus `smoke/tui_proptest.rs`; failing cases are checked in under `proptest-regressions/`. The fast test lane skips them: `cargo test --bin crosslink -- --skip proptest` (`justfile:215`). Fuzzing is a separate `cargo-fuzz`/`libfuzzer-sys` crate (`fuzz/`) with 12 targets — each `#![no_main]` `fuzz_target!` over `#[derive(Arbitrary)]` input, e.g. `fuzz_state_machine.rs` drives issue/session/timer lifecycle ops against a real `Database` in a `tempdir` and asserts no panic (`fuzz_state_machine.rs:1-40`). CI: `.github/workflows/fuzz-nightly.yml`.

### 5. shell-command construction safety

**Two disciplines, one rule: never build a shell string with user data unless each field is escaped.**

- **argv vectors (default).** tmux and docker/podman invocations pass every value as a discrete `Command::args([...])` element, immune to shell metacharacters. tmux: `new-session -d -s <name> -c <dir>` and `send-keys -t <name> <cmd> Enter` (`launch.rs:584-592,627-628`). docker: `container.rs` assembles the whole `docker run -d` line via repeated `cmd.args([...])` — `--name`, `--label`, `--memory`, each `-v` mount, each `-e` env (`container.rs:349-441`); `docker build`/`docker info` likewise (`container.rs:64,262-269`). `run.rs` collects the runtime args into a `Vec` and `Command::new(runtime_cmd).args(&args)` (`run.rs:791-792`).

- **shell strings (only where unavoidable).** `shell_escape_arg` (`utils.rs:200-202`) single-quote-wraps and rewrites `'` → `'\''`. It is used exclusively to build the one string that genuinely runs under a shell — the `claude` command tmux executes. `build_agent_command` (`launch.rs:167-223`) escapes *every* interpolated field: `--model`, `--allowedTools`, the kickoff file, the worktree path, `CLAUDE_CONFIG_DIR`, and `--permission-mode` (`launch.rs:193-220`), and folds env via `env -u CLAUDECODE {KEY=val} claude ...` so assignments survive any wrapping `timeout`/sandbox prefix (`launch.rs:201-208`).

- **Proof by adversarial test.** `smoke/adversarial.rs:297-305` creates an issue titled ``Issue with $(whoami) and `id` and $HOME`` and asserts the literal `$HOME` is echoed back — confirming no expansion anywhere in the pipeline.

### 6. documentation conventions

- **`//!` module docs.** Almost every module opens with a `//!` header stating purpose and invariants (`events.rs:1-6`, `compaction.rs:184-213`, `hub_v3.rs:1-18`, `hub_source.rs:1-30`, `hydration.rs:1-6`, `shared_writer/mod.rs:1-8`, `mutations.rs:1-13`). `///` item docs are dense but `# Errors`/`# Panics` sections are deliberately lint-exempted "at scale" (`lib.rs:3-6`).

- **"E-ana tablet" header.** `// E-ana tablet — <description>` marks the 14 files of the kickoff/design pipeline feature-set (all `kickoff/*.rs` plus `design_doc.rs`, `design_cmd.rs`, `prune.rs`) — a provenance marker for code authored through the design-doc pipeline (`kickoff/run.rs:1`, `kickoff/mod.rs:1`, `design_doc.rs:1`, `prune.rs:1`).

- **GH#nnn references.** ~498 issue references in `src/` anchor rationale to history. Multiple accepted forms: `GH#361` (`events.rs:95`), bare `#332` (`compaction.rs:231`), `#754`/`#767` (`mutations.rs:4`, `hub_v3.rs:33`), `forecast-bio/crosslink#653` (CHANGELOG), and `gh#34` (`dashboard/projects.rs:208`). Renames/behavior changes are annotated in place (`commands/mod.rs:28-30` #448).

- **CHANGELOG discipline.** Root `CHANGELOG.md` follows Keep a Changelog (`CHANGELOG.md:5`) with a live `## [Unreleased]` section split into `### Added / Changed / Fixed` (`CHANGELOG.md:7-55`); every entry cites its issue (`gh#53`, `gh#49`, `gh#34`). Closing an issue auto-appends a changelog entry; `crosslink issue close-all [--no-changelog]` opts out (`CLAUDE.md:50`).

### sources

- crosslink/Cargo.toml, crosslink/src/lib.rs, crosslink/src/main.rs
- crosslink/src/commands/mod.rs
- crosslink/src/commands/kickoff/{mod,run,launch,monitor}.rs
- crosslink/src/commands/container.rs
- crosslink/src/utils.rs
- crosslink/src/events.rs, crosslink/src/compaction.rs
- crosslink/src/hub_v3.rs, crosslink/src/hub_source.rs
- crosslink/src/hydration.rs, crosslink/src/db/{mod,hydration}.rs
- crosslink/src/sync/core.rs
- crosslink/src/shared_writer/{mod,core,mutations}.rs
- crosslink/src/dashboard/{projects,poll}.rs
- crosslink/tests/cli_integration.rs, crosslink/tests/smoke/harness.rs, crosslink/tests/smoke/adversarial.rs
- crosslink/fuzz/fuzz_targets/fuzz_state_machine.rs
- .github/workflows/ci.yml, justfile, quality.md, CLAUDE.md, CHANGELOG.md

