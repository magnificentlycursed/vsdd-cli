---
title: "Run-record capability inventory — the WAS-side oracle"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
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

**Dispatch-primitive dependence (verified both directions):** `effort` is present in runtime-harness Agent-tool subagent transcripts (a5e20d9b: `effort: high` ×24) and **absent from crosslink-kickoff records** — on that path the dial is neither settable (upstream #61) nor recorded, so **the dispatch manifest is the only reliable dial source across primitives**.

Each assistant message: `model`, `content[]`, `stop_reason`, `stop_details`, `usage`. `usage` verbatim: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`, `cache_creation.{ephemeral_5m,ephemeral_1h}`, `service_tier`, `inference_geo`. `content[]` carries every `tool_use` with FULL input (Read `file_path` + `offset`/`limit`; Bash `command`) and, in user events, `tool_result` content + sizes.

### token semantics (reading the numbers honestly)

- `output_tokens` = generated (reply/edits/reasoning). `input_tokens` = fresh uncached input (tiny when cached).
- `cache_creation_input_tokens` = **fresh load — the real cost**. `cache_read_input_tokens` = served from cache — cheap reuse.
- A subagent's dispatch prompt + Reads show as cache_creation first, cache_read after.
- **The raw read-count lies; offset/limit + the cache split tell the truth**: 7 partial Reads of a 186KB doc with `limit=30` + line-windows is targeted discipline, not waste (a5e20d9b: fresh 92,900 vs reuse 599,256).
- Real waste is cross-agent + dead-agent: the 11-agent run showed ~2.64M fresh vs ~25.7M cache-read (10:1 reuse — caching works); the failure mode is each cold agent re-fresh-loading shared context, and the dead contract-drafter's 408,894 fresh-load-then-died (100% waste, measurable).

### provenance discipline (anti-fabrication, four-valued)

Every figure carries a source tag: **recorded** (verbatim from usage/tool events) · **measured** (deterministic over an actual file) · **judgment** (a labeled assessment, never a fake metric) · **could-not-check** (the oracle was unreachable or agent-writable-only). The local `agent-<id>.jsonl` is agent-writable — evidentiary only when server-synced (the #815 corroboration keystone); un-synced transcripts ground could-not-check, never verified claims.

### known traps (each caught live in this estate)

- `spawnDepth` — fabricated; does not exist (see above).
- `effort` claimed from kickoff-path records — absent there; manifest is the source.
- "the journal" referenced without its schema — it is `journal.jsonl` as documented above, nothing more.
- Read-count as a waste metric — see token semantics.
- An agent-writable record cited as verification — the four-valued provenance rule exists precisely for this.

