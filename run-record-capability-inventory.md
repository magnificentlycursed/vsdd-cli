---
title: "Run-record capability inventory — the WAS-side oracle"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-10-02
---

## Design Specification

### the audit surface — mechanism → proof it fired → fail loud when

| Mechanism | Proof it fired | Fail loud when | Home |
|---|---|---|---|
| Skill (primer / domain / supplement / design) | skill-invocation record (`attributionSkill`) | a composition-required skill not invoked (Read-only is the weak signal; paraphrase is nonconformance) | the #840 subsystem (REQ-16) |
| Hook (git / session-start / pre-tool / read-gate) | hook trace log vs git history | a governed act with no matching hook trace | conformance-at-action-time; mechanize |
| CI check / required gate | the check RAN (not merely passed) | a required status check didn't run | the subsystem + branch ruleset |
| Tool / mapped affordance | tool-call record | a hand-rolled Bash equivalent of a mapped affordance, no stated reason | the act-to-affordance map + the deviation registry |
| Schema / validator | validated NON-VACUOUSLY | validated over an empty set (the canary's class) | the non-vacuity canary, generalized |
| Composition function | composition COMPUTED, not echoed | a dispatch composition hardcoded/stored | Phase 2 |
| Dispatch preflight | preflight record precedes dispatch | autonomous dispatch with no preflight record | Phase 5 |
| Dispatch manifest | manifest recorded + round-parity | claimed dispatch with no manifest / count ≠ tracked children | Phase 5 |
| Red-gate / pins | executed-pin (ran, red→green) | a pin that never ran; undeclared skipped test | Phase 3 |
| Routing | filed-routing record | a fix-closed finding with no routing | Slice 1 (LIVE — `vsdd gate`) |
| Dials (model / effort) | recorded dials / the dispatch manifest | unspecified at dispatch (fail-closed) | the manifest discipline |
| Deviations | registry entry with retest trigger + expiry | lapsed/fired without SO re-arm | the deviation registry (leg: Phase 1, building) |

### run-level records (per workflow / dispatch run)

- `agent-<id>.jsonl` — one full transcript per subagent (42KB–508KB observed).
- `agent-<id>.meta.json` — carries `agentType`. **`spawnDepth` does NOT exist** — it was a fabricated field caught by the #840 review (C8); the delegation graph comes from `parentUuid` + `sourceToolAssistantUUID` + the `subagents/` directory.
- `journal.jsonl` — one line per agent lifecycle event `{type: started|result, agentId, key}`; `result` lines carry the agent's full return value. **Read the journal before diagnosing an empty workflow result** — it records what each agent actually returned.
- Harness completion usage — run totals: `subagent_tokens`, `tool_uses`, `agent_count`, `agents_done/error`, `duration_ms`.

### per-agent transcript — field schema (verified on agent-a5e20d9b)

Every event: `timestamp` (ms — wall-clock + per-op latency), `uuid` + `parentUuid` (turn tree), `sessionId`, `agentId`, `cwd`, `gitBranch`, `version`, `entrypoint`, `isSidechain`, `userType`, `slug`, `durationMs` (real per-entry wall-clock). Request-bearing events add: **`effort`**, **`attributionSkill`** (the skill-invocation signal REQ-16 audits), `attributionAgent`, `requestId`, `promptId`, `sourceToolAssistantUUID`.

**Dispatch-primitive dependence (verified both directions):** `effort` is present in runtime-harness Agent-tool subagent transcripts (a5e20d9b: `effort: high` ×24) and was **absent from crosslink-kickoff records** when this page was written. *Corrected 2026-10-02 (vsdd-cli#888):* on the kickoff path the dial is now settable (`--effort`, crosslink PR #77) and is recorded in `.kickoff-metadata.json` beside the resolved model and the budget. That file sits in the agent-writable worktree, unsigned, so **the dispatch manifest remains the only dial record outside the agent's reach**. Whether the kickoff transcript itself carries `effort` was not determined.

Each assistant message: `model`, `content[]`, `stop_reason`, `stop_details`, `usage`. `usage` verbatim: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`, `cache_creation.{ephemeral_5m,ephemeral_1h}`, `service_tier`, `inference_geo`. `content[]` carries every `tool_use` with FULL input (Read `file_path` + `offset`/`limit`; Bash `command`) and, in user events, `tool_result` content + sizes.

### token semantics (reading the numbers honestly)

- `output_tokens` = generated (reply/edits/reasoning). `input_tokens` = fresh uncached input (tiny when cached).
- `cache_creation_input_tokens` = **fresh load — the real cost**. `cache_read_input_tokens` = served from cache — cheap reuse.
- A subagent's dispatch prompt + Reads show as cache_creation first, cache_read after.
- **The raw read-count lies; offset/limit + the cache split tell the truth**: 7 partial Reads of a 186KB doc with `limit=30` + line-windows is targeted discipline, not waste (a5e20d9b: fresh 92,900 vs reuse 599,256).
- Real waste is cross-agent + dead-agent: the 11-agent run showed ~2.64M fresh vs ~25.7M cache-read (10:1 reuse — caching works); the failure mode is each cold agent re-fresh-loading shared context, and the dead contract-drafter's 408,894 fresh-load-then-died (100% waste, measurable).

### provenance discipline (anti-fabrication, four-valued)

Every figure carries a source tag: **recorded** (verbatim from usage/tool events) · **measured** (deterministic over an actual file) · **judgment** (a labeled assessment, never a fake metric) · **could-not-check** (the oracle was unreachable or agent-writable-only). The local `agent-<id>.jsonl` is agent-writable — evidentiary only when server-synced (the #815 corroboration keystone); un-synced transcripts ground could-not-check, never verified claims.

### crosslink-kickoff run records (added 2026-10-02)

Read from the crosslink source at fork tree `ddc0cbe57`, not executed. The full table is on `kickoff-swarm-dispatch-pipeline`.

- **Transcript:** `.crosslink/runtime/agent-events.jsonl` in the agent's worktree. Kickoff launches the agent headless with JSON-lines output and pipes it through `tee` into that file, so it holds the full event stream, tool-use events included. It is unsigned and the agent can rewrite it.
- **Usage harvest:** crosslink writes `token_usage` rows into the driver's local database, but only at `kickoff stop` or `kickoff cleanup`. It parses only lines with a top-level `usage`, which for Claude is the final `result` line. Classes captured: input, output, cache-read, reasoning. **The cache-creation class is hard-coded to none**, so "usage by cache class" cannot come from this table; read the transcript. Cost is a pricing-table estimate. A run killed by timeout probably yields no rows (inferred).
- **Read-back:** the dashboard API `/api/v1/usage` or the SQLite file. No CLI subcommand was found.
- **Other files in the worktree:** `KICKOFF.md` (the assembled prompt), `.kickoff-metadata.json` (start time, timeout, provider, resolved model, effort, budget), `.kickoff-criteria.json`, `.kickoff-doc.json` (the design doc's hash), `.kickoff-status`, and the agent's self-reported `.kickoff-report.json`. None is signed.
- **Out of the agent's reach:** only its signed hub events once synced, which it authors itself, and in container mode the pipeline file beside the design doc.
- **Not parsed by crosslink:** skill invocations. Only usage, status and the last message are extracted from the transcript.

### known traps (each caught live in this estate)

- `spawnDepth` — fabricated; does not exist (see above).
- `effort` claimed from kickoff-path records — since crosslink PR #77 it is in `.kickoff-metadata.json`, an agent-writable file; cite it as could-not-check, and the manifest as the source (corrected 2026-10-02).
- "the journal" referenced without its schema — it is `journal.jsonl` as documented above, nothing more.
- Read-count as a waste metric — see token semantics.
- An agent-writable record cited as verification — the four-valued provenance rule exists precisely for this.
