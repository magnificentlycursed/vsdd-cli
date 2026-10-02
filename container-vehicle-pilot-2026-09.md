---
title: "Container vehicle pilot — 2026-09-24/25 (vsdd-cli #878)"
tags: ["reference", "dispatch", "crosslink"]
sources: []
contributors: ["xqjG"]
created: 2026-09-26
updated: 2026-10-02
---

# Container vehicle pilot — 2026-09-24/25 (vsdd-cli #878)

The first headless run of the adopted autonomous vehicle (`crosslink kickoff run --container`) on this repository, driven to a pass over eight runs. Everything below was observed, not inferred; commit and issue handles are the record.

## Outcome

Run 8 (2026-09-25 14:17–15:57 UTC, container 4b0568df0db6, agent E00r-ysOk, image `ghcr.io/magnificentlycursed/crosslink-agent:nightly`, `--effort low --budget-usd 2`, USD 0.66) completed the read-only task: readiness established in the worktree, lock held, container login honoured, hooks passed, `cargo run -p vsdd -- status --machine` built and ran, and the agent published three events to the hub from inside the container (plan 15:08, result 15:23, handoff 15:30). That is the exit condition the exception-register entry `manual-dispatch-fallback` was re-armed on.

## What each run stopped on

| Run | Stopped by | Class | Fix |
|---|---|---|---|
| 1 | Hook payload absent in the fresh worktree (`.claude/hooks/*` gitignored; tracked wiring fails closed) | image predates upstream dd0b7173 | rebuilt nightly from synced develop |
| 2 | `Not logged in`: image expects the new per-user container login; `container auth login` hard-codes the private upstream image | upstream | fork `--image` / `CROSSLINK_CONTAINER_IMAGE` (fork PR #5) |
| 3 | `repository readiness is missing` in the kickoff worktree | upstream 0a2ac7013 missed `launch.rs::init_worktree_agent` | `daemon::ensure_and_wait` after init (fork PR #6) |
| 4 | `Lock confirmation timed out after 34s (threshold 30s)` | v2-era bound vs per-event signature verification | 120 s (fork PR #6) |
| 5 | In-container readiness `blocked_corrupt`: hub-cache worktree registered at the pre-rename `Documents/Source` path; container has no symlink | ours (2026-08-03 rename) | `git worktree repair` on both caches |
| 6 | Same message: shared `.git` refuses `worktree add --orphan -b crosslink/hub-v3-host`, cache not mounted | upstream (kickoff never mounted the caches) | mount `.hub-cache`/`.knowledge-cache` (fork PR #6) |
| 7 | Agent reached the task; hub publication `could not read Username for https://github.com`; `vsdd`/`mdatron` not in the image | upstream (no git credentials) + task wording | `GH_TOKEN` passthrough via git env config (fork PR #6); task reworded to `cargo run -p vsdd` |
| 8 | passed | | |

## The hub migration and the frontier incident

The new binary required reconciling the hub to the per-checkout readiness model (local db 17→18, checkpoint causality 1→2, generation `79615b40…`; ~5 min, almost all per-event signature verification; the ghost record id −1 became issue #884). Afterwards every real event from the driver identity `xqjG` was refused: `frontier prefix hash for agent 'xqjG' sequence 1169 is forged or rewritten`. Cause: the migration seeded the agent's causal frontier from the pinned pre-v3 tip (commit-per-event numbering, 1169) while the ref's `events.log` holds 177 events; the writer seeded its next `agent_seq` from the tip alone, so 178 fell inside the frozen prefix. The hub was poisoned three times (a lock steal, then two ordinary comments) and recovered each time by resetting `refs/heads/crosslink/agents/xqjG` to `dfca5d03` and force-pushing (operator act). Fix (fork PR #6, third iteration): the writer's ceiling is the checkpoint frontier's sequence **only when the frontier's recorded tip is an ancestor of the live agent ref**; a frontier from a foreign lineage (a hub migrated from the v2 layout, whose v3 ref restarts at 1) is ignored. Verified: the driver's next event went out as 1170 and `integrity` passed 5/5. Lesson recorded in memory: check the fork PR's own test jobs after every commit — the unconditional version shipped a 14-test regression into upstream PR #103 before the lineage rule.

## Daemon behaviour worth knowing

Corrected 2026-09-28 (vsdd-cli#885; upstream correction on Corvidae-Coding-Projects/crosslink#102).

- **Readiness is crosslink's job, not the agent's.** Current crosslink's session-start hook runs `daemon ensure --wait-ready --json`, and its work-check blocks with the real readiness reason. This estate's hooks predated that model until PR #46, so nothing established readiness, and agents filled the gap by hand. Never drive readiness by hand (no ensure/restart/poll loops, no kills, no lock-file deletion). A readiness failure is a finding to report.
- **Daemon exits.** On 2026-09-25 daemons did exit on their own. Their logs show `readiness record is stale` and `hydrating current authority during daemon housekeeping`. The mechanism once given for this ("refresh deferred behind active mutation permits") is **disproved**: a write paused for 100 s did not kill the daemon. The trigger is still unexplained. Deaths from 2026-09-26 to 09-28 were a sibling session's `pkill -9 -f "crosslink daemon"`, which kills every checkout's daemon on the machine. When a daemon dies, check for concurrent sessions first.
- **Commit gate timeout.** The work-check gives `crosslink session status` 3 s and reads a timeout as "no active issue", so `git commit` can be refused while the daemon is busy (27.7 s observed). Reported upstream as #104.
- A daemon started from a `target/release` path dies when `cargo build` rewrites that binary; start daemons from `~/.cargo/bin`.
- **Slow reconciles.** The ~20-minute reconciles of a small hub are most likely a hung `git fetch` of `refs/heads/crosslink/reconciliation/*`, which has no timeout. A hung fetch holds the reconciliation transition, and there is no supported way to recover it. A transient `git ls-remote` SSL timeout at bootstrap is recorded `blocked_corrupt` (terminal).

## Where the record lives

Fork: magnificentlycursed/crosslink PR #5 (merged), PR #6 (merged; five commits + the lineage fix), tracker #62–#64. Upstream: Corvidae-Coding-Projects/crosslink #100 (CI), #101 (private image), #102 (umbrella with RCA), PR #103 (seven fixes from fork develop). vsdd-cli: #878 (pilot), #880/#883 (Status wiring, ghost-id gate), PR #42 (routing-gate readiness), PR #43 (register dispositions), memory `upstream-pr-ledger-2026-09`.

## Currency notes (2026-10-02)

Added under vsdd-cli#888 from the kickoff investigation recorded on `kickoff-swarm-dispatch-pipeline`.

- **The worktree readiness call has changed since run 3's fix.** The table above names `daemon::ensure_and_wait` (crosslink-fork PR #6). After crosslink-fork PR #8 removed the 30-minute readiness bound (merged 2026-10-02), the code calls `daemon::ensure(&wt, true)` at `src/commands/kickoff/launch.rs:617-624`.
- **The installed binary is `0.9.0-beta.1+973e395dc`,** which is fork develop at crosslink-fork PR #7. It does not include PR #8.
- **crosslink PR #103 is still open** (head `cc756de92`, checked 2026-10-02). It carries fork PRs #5 to #8.
- **The provenance of run 8's "USD 0.66" is not recorded here.** Crosslink's own usage harvest stores a pricing-table estimate, not a billed amount; if the figure came from there it is an estimate.
- **Kickoff's stall watchdog does not run in containers,** and an agent that crashes without timing out leaves the status at "running" until the wall clock expires.
