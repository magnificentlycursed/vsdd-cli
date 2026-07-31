---
title: "Grounding audit — the verifiable-conformance + efficiency subsystem"
tags: ["design-input", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-07-31
updated: 2026-07-31
---


## Design Specification

### 0. founding evidence: "cargo cult vsdd"

Analyzing this session's own dispatch runs, the domain personas / phase primers / supplements shipped as FILES the methodology *describes* as loaded, but in the dispatch path the loading mechanism never fired — the process was performed in name only. §133 already forbids this ("availability is not activation … injection or recorded-Read gating, never description-match reliance"); we wrote the principle and violated it in our own review dispatch. This is the subsystem's reason to exist.

### 1. the measurable substrate (verified — no telemetry stack needed)

Run records the harness already emits carry ground truth. Per Claude Code agent transcript (`agent-<id>.jsonl` in the run dir), per event:
- **`timestamp`** (ms, every event) → wall-clock, per-op latency.
- **`effort`** (the reasoning dial), **`message.model`**, `gitBranch`, `cwd`, `sessionId`, `uuid`/`parentUuid` (turn tree), `slug`.
- **`message.usage`**: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`, `cache_creation.{ephemeral_5m,ephemeral_1h}`, `service_tier`.
- **`message.content[]`**: every `tool_use` with **full input** (Read `file_path` + **`offset`/`limit`**; Bash `command`), and `tool_result` content/sizes.
Run-level: `journal.jsonl` (per-agent lifecycle + return value), `<id>.meta.json` (agentType, spawnDepth), harness completion usage (subagent_tokens, tool_uses, agent_count, duration_ms).

**Provenance discipline (the anti-fabrication guarantee, = §151 right-sized):** every figure carries a source tag — **recorded** (verbatim from usage/tool events), **measured** (deterministic over an actual file, e.g. tokenizing a Read target), or **judgment** (a labeled assessment, never a fake metric). The oracle is harness-produced (the agent cannot forge its own Read/usage events); CI over server-synced records is the independent oracle (§264, the #815 tamper-evidence keystone).

### 2. worked findings (this session's runs, provenance-tagged)

Cold-review lens `a5e20d9b` (#821), RECORDED: wall-clock 5m16s; model opus-4-8; effort high; output 5,490; cache_creation (fresh) 92,900; cache_read (reuse) 599,256. Its 8 contract Reads were **all partial** (limit=30, then 1–4-line windows at offsets) — grep→targeted-slice, i.e. GOOD discipline.
- **The raw count lies; offset/limit + cache tell the truth.** "7 reads of the 186KB contract" looked like waste; the offset/limit proved targeted reads. → the subsystem must key on offset/limit + cache_creation, never tool-call counts.
- **Real waste is cross-agent + dead-agent, not within-agent.** Across the 11-agent #821 run: ~2.64M cache_creation vs ~25.7M cache_read (10:1 reuse — caching works). The waste is each cold agent re-`cache_creating` shared context (contract/design/code) → warm-handoff target. And the failed contract drafter (`a84044`) spent the **most fresh load, 408,894**, then died — 100% waste, measurable.
- **Conformance gap (the big one), RECORDED:** of the review lenses + drafters, only ONE read a `vsdd-domain-*` persona; several read no primer/domain/supplement. They got their instructions from the dispatch prompt, not the persona files → the personas were **bypassed**.
- **Tool histogram:** Bash 91, Read 64, Edit 60, StructuredOutput 7 — heavy repeated rg/git shapes = new-tool opportunities.

### 3. the invariant — verifiably true, from the record, per dispatched agent, every phase

Each is a CONTROL (mechanism + ladder grade + falsifier + the Red-Team bypass it closes), NOT a persona sentence. Injection-primary (Red Team's choice: read-gate leaves read-then-ignore + never-trigger bypasses; injection-by-construction leaves none).
1. **Phase known** — recorded (state artifact / session breadcrumb, §37).
2. **Phase primer loaded** — INJECTED by construction into the dispatch prompt; verified `Read ⊇ required`. Bypass closed: inline-in-prompt (no Read), read-then-ignore.
3. **Composed domains loaded** — injected; verified against the composition function's output (`was ⊇ should`). Bypass: fewer/wrong domains, inline.
4. **Supplements loaded** — injected; verified. Bypass: skip, wrong-language.
5. **Intended tools used / no hand-rolled affordance** — CI-detection vs the act-to-affordance map (§130 mechanized). Bypass: hand-roll the Bash equivalent (must make the tool the path of least resistance).
6. **Cache leveraged / warm handoff** — by construction (dispatch passes the slice). Bypass: cold-reload anyway (no penalty).
7. **Model + effort specified** — dispatch fail-closed if dials unspecified (§91 "never inherited"). Bypass: silent defaults.
8. **Reads offset/limit-scoped** — detection (flag limit=none loads of large files) + warm slice removes the reason to full-load.

**Binding:** Slice 2's composition function computes the SHOULD; the verifier reads the WAS (records); conformance is `was ⊇ should`; the gate blocks the delta. Composition + verifier are two ends of one loop → same cycle.

### 4. the three design decisions folded in

- **"Cost is knowable" re-scope:** keep the STATIC price (Slice 2 — priced bill + CI bloat-gate) + records-based INSIGHT (efficiency/right-sizing, "is 100k a lot" answerable from static bill + bands). RETIRE the telemetry apparatus (OTel collectors, dashboards, dollar ledger, calibration-tied-to-dollar-actuals) — under subscription, dollars are a projection. Rationale grounded in the founding operator interview (2026-07-18): "observability's purpose is insight … recorded-evidence answers."
- **Personas/supplements usefulness:** stop shipping prose-to-hopefully-read (convention grade = vibes = what cargo-culted). Enforceable content → checks; required content → injected by construction; irreducible judgment-prose → injected + cold-review-audited (§133's own instrument for read-but-ignored). Precedent: this session turned "no deprecated terms" (prose) into E0093 (CI-block) — the same convention→control move.
- **Cold vs warm review:** the load-bearing value is *different agent + adversarial stance* (independence of judgment). "Cold" conflates cold-on-FRAME (the value) with cold-on-ARTIFACTS (re-load cost = the cross-agent waste). Reframe: **independent-of-frame always; cold-vs-warm-on-artifacts a right-sized dial.** Warm = hand the RAW slice (diff/design/contract section), never a curated summary (curation re-imports the author's blind spot). True cold reserved for terminal rounds guarding collective anchoring.

### 5. domains: the adversarial pair that owns this subsystem

- **AI Engineer** (re-grounded from observability-lens): reads the measured record; judges right-sizing (model/effort as VM specs; over/under-provisioned) and yield (was the load worth it); owns the efficiency advisories (§149, surfaced through crosslink's viewers, vsdd builds no viewer).
- **Red Team** (optimized to this): finds how each control is circumvented; drives "correct path = only path"; probes the ORACLE's forgeability (harness-produced transcript + CI over server-synced state = the #815 tamper-evidence). Selects injection over read-gate.
Composition for the cycle: AI Engineer + Red Team as the adversarial pair, on the baseline.

### 6. leverage vs build — ride crosslink, supply the vsdd layer (§355)

Crosslink's dispatch primitive is "strong on crosslink protocol and carries NONE of the vsdd layer — the fallback shape is confirmed as vsdd injecting manifest and composition into the dispatch vehicle's prompt" (§355).
- **LEVERAGE (don't rebuild):** `swarm` (init-from-design-doc + launch-per-phase) / `kickoff` dispatch, worktrees, sessions (phase pointer in breadcrumbs §37), trust/signing, preflight (§108), and the viewers (mission control / tui / web — §149 "vsdd builds no viewer"). The agent transcripts (the conformance oracle) are produced regardless of orchestrator.
- **BUILD/SUPPLY (vsdd's, crosslink doesn't carry it):** the composition (what to inject), the INJECTION seam into the dispatch prompt (named upstream capability requirement §81; kickoff prompt-assembly the candidate), the conformance verifier, the efficiency insight engine.
- **Named crosslink dependencies (not routed around):** kickoff working (#837 — image `.claude.json` bug + dead default image org path); the injection seam (§81); the effort/thinking-budget dispatch parameter (§149, second named seam); the swarm live fire (§195).
- **`Workflow` tool status:** the *adopted* affordance for attended review fan-out (§130 map); a *stated-reason bootstrap fallback* for autonomous dispatch while kickoff is broken/swarm unproven — NOT the target orchestrator. Must not calcify.

### 7. open framing question for the cycle

Center of gravity: **"verifiable conformance (efficiency as one axis)"** — headline is *prove the harness's disciplines fired*; cost/efficiency lives under it. (Operator leaning; confirm at cycle open.)

### references (contract §, this session)

§133 availability≠activation · §130 act-to-affordance · §81/§149 injection + effort seams (upstream reqs) · §355 crosslink-carries-none-of-vsdd-layer · §149 vsdd-builds-no-viewer · §264/#815 tamper-evidence · §37 session breadcrumbs · §195 swarm live fire · §140-153 Cost-is-knowable (re-scoped) · #837 kickoff broken · operator-interview-2026-07-18 (knowledge page). Run analyzed: wf_0ac03bda-2ee (#821 cold review, 11 agents).

