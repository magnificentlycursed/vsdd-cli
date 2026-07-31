---
title: "Run-record capability inventory + audit-surface reference"
tags: ["design-input", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-07-31
updated: 2026-07-31
---


## Design Specification

### a. audit surface — mechanism → proof-it-fired → fail-loud when → home

The instances of the governing law (*authored is not exercised*). "Home" = built / this-subsystem / other-slice.

**Grade honesty (#840).** Every "this-subsystem" row is **0% built**: the proof / fail-loud columns state the *target*; the current grade is **convention / hand-audit** (bootstrap). A control whose triggering **occasion is absent**, or whose oracle (the **server-synced, tamper-evident** transcript, REQ-10 — the local `agent-<id>.jsonl` is agent-writable and not authoritative) is **unavailable**, reports **could-not-check**, never clean. Registry *completeness* is keyed to an **independent census** (the §131 installed-artifact manifest / a build-break), not self-registration; and each control pairs with a **committed negative-case fixture** (fires-on-bad AND clean-on-good), not merely an "it-ran" trace.

| Mechanism | Proof it fired | Fail loud when | Home |
|---|---|---|---|
| Skill (primer / domain / supplement / `/design`) | skill-invocation record | a composition-required skill not invoked (Read-only is the weak signal; paraphrase is nonconformance) | this subsystem (REQ-16) |
| Hook (git / session-start / pre-tool / read-gate) | hook trace log vs git history | a governed edit/act with no matching hook trace (bypassed or never wired) | §133/§136 — mechanize |
| CI check / required gate | the check **ran** (not merely passed) | a required status check didn't run — the mis-typed/never-triggered case | this subsystem + branch ruleset |
| Tool / mapped affordance | tool-call record | a hand-rolled Bash equivalent of a mapped affordance, no stated reason | REQ-5 / §130 |
| Schema / validator (mdatron, registry pairs) | validated **non-vacuously** | a schema-classed artifact never validated, or validated over an empty set | generalize the #825 canary |
| Composition function | composition **computed**, not echoed | a dispatch composition hardcoded/stored, not computed | Slice 2 / REQ-9 |
| Dispatch preflight (§108) | preflight record precedes dispatch | autonomous dispatch with no preflight record | §116 / Slice 6 |
| Dispatch manifest | manifest recorded + round-parity | a claimed dispatch with no manifest / count ≠ tracked children | Slice 6 / §101 |
| Red-gate / pins | executed-pin (ran, red→green) | a pin that never ran; an undeclared skipped/ignored test | Slice 4 |
| Routing | filed-routing record | a fix-closed finding with no routing | Slice 1 (live) |
| Read-gate | recorded Reads precede the governed edit | governed edit with no prior governing-doc Read | §133 |
| Dials (model / effort) | recorded **manifest** dials (model / effort) | unspecified at the **dispatch manifest** (fail-closed at preflight); `effort` as a per-event *transcript* field is **could-not-check** (#840 — see below) | REQ-7 |
| Signing | signature present | unsigned manifest/write where signing is configured | §116 |
| Session-start (operator session) | session-start hook fired + session skill invoked | operator session started without the hook / session skill | this subsystem (operator-session control) |

### b. run-record capability inventory — what the harness already records (no telemetry stack needed)

### Run-level records (per workflow / dispatch run)
- `agent-<id>.jsonl` — one full transcript per subagent (42KB–508KB observed).
- `agent-<id>.meta.json` — `{agentType}`. **Correction (C8/#840): `spawnDepth` was previously listed here but is *fabricated* — it appears in no record.** The real parent/child linkage is `parentUuid` + `sourceToolAssistantUUID` on request-bearing transcript events, plus the `subagents/agent-<id>.jsonl` directory layout.
- `journal.jsonl` — one line per agent lifecycle event `{type:"started"|"result", agentId, key}`; `result` lines carry the agent's full return value. **This is the lifecycle *index*, NOT the conformance oracle** — the oracle is the server-synced, tamper-evident run transcript (REQ-10); `journal.jsonl` is a distinct file and is never conflated with the transcript.
- harness completion usage — run totals: `subagent_tokens`, `tool_uses`, `agent_count`, `agents_done/error`, `duration_ms`.

### Per-agent transcript — full field schema (verified on agent-a5e20d9b, a #821 cold lens)
Per event (all events): `timestamp` (ms — on **every** event → wall-clock + per-op latency), `uuid` + `parentUuid` (turn tree), `sessionId`, `agentId`, `cwd`, `gitBranch`, `version`, `entrypoint`, `isSidechain`, `userType`, `slug`. On request-bearing events: `attributionAgent`, **`attributionSkill`** (the recorded skill-invocation capture-source — REQ-16's activation signal), `requestId`, `promptId`, `sourceToolAssistantUUID`. **Correction (C8/#840):** `effort` was previously listed here as a per-event field; the #840 review reports it is **absent from dispatched-agent transcript records** — it is a **dispatch-manifest / kickoff parameter** (REQ-7's preflight presence-gate reads the manifest, not the transcript). This **contradicts the worked example below** (`agent-a5e20d9b`, "effort high") and is flagged **could-not-check pending re-verification** against a real transcript; the manifest is the gate's source on timing grounds regardless (preflight precedes the transcript).
Event types: `assistant`, `user`, `attachment`.
Each assistant message: `model`, `role`, `content[]`, `stop_reason`, `stop_details`, `stop_sequence`, `diagnostics`, `container`, `context_management`, **`usage`**.
`usage` (per message, verbatim): `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`, `cache_creation:{ephemeral_5m_input_tokens, ephemeral_1h_input_tokens}`, `service_tier`, `inference_geo`.
`content[]`: `tool_use` (name + **full input** — Read `file_path` + **`offset`/`limit`**; Bash `command`); `text`; and in `user` events `tool_result` (content + sizes).

### Token semantics (for reading the numbers honestly)
- `output_tokens` = what the model generated (reply/edits/reasoning).
- `input_tokens` = fresh uncached input (tiny when everything is cached).
- `cache_creation_input_tokens` = **fresh load — the real cost**.
- `cache_read_input_tokens` = **served from cache — cheap reuse**.
- For a subagent, the dispatch prompt + file Reads show as `cache_creation` first, `cache_read` after.

### Worked example (agent-a5e20d9b, #821 cold lens — all RECORDED)
Wall-clock 5m16.5s; model `claude-opus-4-8`; effort `high` [**could-not-check — #840 disputes `effort` as a sub-agent transcript field; re-verify**]; input 43; output 5,490; **cache_creation (fresh) 92,900**; cache_read (reuse) 599,256; peak single-msg cache_read 33,832. Tool calls: 5 Bash (rg/git-diff) + **7 Reads of the contract, all partial** (`limit=30`, then 1–4-line windows at offsets) — the count "7 reads" looked like waste; the offset/limit + the modest cache_creation proved *targeted discipline*. Cross-run (11 agents): ~2.64M cache_creation vs ~25.7M cache_read (10:1 reuse — caching works; the waste is cross-agent redundant fresh-load, and the dead contract-drafter's 408,894 fresh-load-then-died).

### The boundary (honest)
Recorded/measurable here: tokens (incl. cache split), reads (paths + offset/limit), tool calls, model, effort, wall-clock (timestamps). **Not** here: server-side window occupancy, rate-limit-window headroom — the only place SDK signals would add anything, a deferred optional complement.

### Provenance discipline (anti-fabrication)
Every figure tags its source: **recorded** (verbatim from usage/tool events) · **measured** (deterministic over an actual file, e.g. tokenizing a Read target) · **judgment** (a labeled assessment, never a fake metric) · **could-not-check** (the source record is unavailable — an un-synced transcript per REQ-10, or a field the record does not carry — reported honestly, never dropped nor shown as a real metric). = §151 provenance, right-sized to "read the transcript."

