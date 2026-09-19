---
title: "Impact analysis: Observability Engineering 2e → vsdd Slices 6/7, GH#33, escape corpus, primers (2026-09-18)"
tags: ["design-input", "observability"]
sources:
  - url: "http://oreilly.com/catalog/errata.csp?isbn=9781098179922"
    title: ""
    accessed_at: "2026-09-19"
contributors: ["xqjG"]
created: 2026-09-19
updated: 2026-09-19
---

# Impact analysis: Observability Engineering 2e → vsdd (2026-09-18, vsdd-cli #869)

**Method.** Each book claim (the four `o11y-*` pages + [[observability-engineering-2e-reading-map]]) was checked against vsdd's actual state: the contract (`.design/agent-first-vsdd-toolkit.md` §Recorded review dispatch, §Cost is knowable), the ratified conformance+efficiency subsystem ([[verifiable-conformance-and-efficiency]], REQ-1..23), the Slice 2 design ([[composition-slice]]), build-plan Phases 5/6 (Slices 6/7), the deviation registry, and the Phase-3 primer. Verdicts: **ahead/satisfied**, **gap → design input**, **conduct escape**, **decision needed**. Nothing here edits a governed artifact; every route names its owned process (spec amendments re-enter under the owning domain + cold review; SO ratifies).

**Headline.** vsdd's contract already anticipates most of the book, and is stricter in three places the book doesn't reach (the could-not-check provenance class, the agent-writable-oracle rejection, the falsifiable exercise law). The book's real yield is (1) one missing noun — the per-dispatch **plan** validated before launch — and (2) evidence that two ratified controls were **authored but never exercised**, including against the 2026-08-10 roast.

## A. Already satisfied or ahead (grounding only — no change)
- **A1** Native units recorded, dollars a projection through a declared billing context (§Cost is knowable) ⊇ Ch21 "derive cost at query time." vsdd also names usage windows + operator time as the binding constraints; the book doesn't.
- **A2** Four-value provenance incl. **could-not-check** (REQ-14/AC-14) — stricter than Ch17/21 lineage, which has no dormant-vs-clean distinction.
- **A3** REQ-10 rejects the agent-writable local transcript as oracle; the book assumes trustworthy telemetry.
- **A4** The governing law *authored is not exercised* + exercise registry + negative-case fixtures + occasion predicates IS Ch27's "activities vs learning" made falsifiable.
- **A5** Invariant classes: Slice 2 determinism/byte-match/exact-slice; REQ-9 `WAS ⊇ SHOULD`; manifest "inputs by content hash" = Ch17's determinism hash.
- **A6** Injection by construction (REQ-2/3/4/12/16) = Ch10's maintained context layer; Ch1 "encoded into the system or it doesn't exist" = §133.
- **A7** Phases-dispatched keystone (REQ-17) is stricter than Ch8's commander/caretaker split; caretaker = container kickoff.
- **A8** Three-valued preflight (pass/fail/inconclusive, fail-closed) ⊇ Ch21's pass/fail/maybe.
- **A9** Ch32 ("relocate rigor out of the implementation into the system around it"; "code becomes cache") is the contract's thesis; generated context + build-plan-from-contract is the tooling Ch32 says "does not exist yet."
- **A10** Ch25/27 two loops: `vsdd gate` = operational loop, Slice 7 engine = learning loop — already split by design.

## B. Gaps → routed design inputs
- **B1 [Slice 6, phase-1a]** **The dispatch plan as a PROPOSED object validated before launch** (Ch17 ActionPlan). Today the manifest is written *at* dispatch (what was sent, dials, expected band) and REQ-21 leg 1 binds shape+ceiling at ratification of the *composition* (a document), not per dispatch; preflight members are environment checks. Gap: no per-dispatch plan (predicted agents / token band / wall-clock, stages, dials) rendered to the operator and checked against the ratified ceiling + calibration band *before* launch; no `plan_id` on the manifest; no post-run plan-vs-actual verdict. Input: a `plan-within-ceiling` preflight member (fail-closed at vsdd interception points; could-not-check on the native Task/Agent path per AC-27), `plan_id` on the manifest, and the rendered plan as the instrument of the consent grant ("the dispatch record names the operator act that invoked it" — the plan is what the operator consents to). Plain name: *dispatch plan*.
- **B2 [Slice 7, phase-1a]** **Plan-vs-actual as an engine verdict**, not an orchestrator comment. "The orchestrator compares actuals per result in-cycle" (§Cost is knowable) is convention-grade; make it a measured output with `proposal_status` / `rejection_reason ∈ {band_exceeded, ceiling_exceeded, width_unplanned, dials_unrecorded}` (Ch17 simulation gate).
- **B3 [Slice 7, advisory data]** Two new down-signals for the effort-scaling catalog (versioned data authored at phase-1c → owned amendment): **dispatch-plan rejection rate** and **operator override/kill rate** (Ch17 "highest-fidelity AI-reliability signal"; Ch22 SLO on cost).
- **B4 [Slice 7, phase-1a]** **Operator workload as a recorded dimension** (Ch22 Rasmussen's third boundary). §144 already names operator time as a binding constraint; nothing records it. Input: attended wall-clock + intervention count per dispatch/cycle from session records + tracker `decision`-kind events; provenance measured/judgment.
- **B5 [Slice 7 — DECISION, SO]** **Materialized dispatch rows vs pure reader.** Ch17/25 persist wide events; REQ-11 retires the *dollar* ledger and says "a reader over the records, not a persisted ledger." Recommendation: the engine's unit of output is one wide **dispatch record** in native units (identity, versions, plan, dials, per-agent usage by cache class, conformance verdict, provenance tags), materialized as *regenerable* versioned data (a cache of the reader, mdatron-governed) so viewers and the gate read the same row (signal parity, Ch17). Not the retired dollar ledger. Needs a ruling.
- **B6 [Slice 4/5]** Conformance findings carry their REQ→home entry (Ch17 "the ontology tells you where to look") so routing is in the finding, not re-derived each Phase 4. Small.
- **B7 [Slice 2, #839 — question]** Should the context generator inject a "known deviations (registry) + recent incidents (incident-corpus-index)" layer (Ch10's context layers)? A design question for #839, not a requirement.
- **B8 [Slice 4 residual + Slice 7]** The deferred per-commit wall-clock budget gets its shape from Ch18: end-to-end user time as the SLI, P50/P90 over routing-gate/verify workflow runs (GitHub Actions job records are harness-produced), flake rate = retry-success %.
- **B9 [Slice 7 ACs]** At least one **learning-loop indicator** among Slice 7's acceptance criteria (Ch27: hypothesis→validation time shrinking; questions answerable from records without new capture), not only "advisories emitted."

## C. Conduct escapes (REQ-22 escape corpus + governed-corpus edits)
- **C1 — the 2026-08-10 roast (#867) violated ratified REQ-21.** 32 agents / 1,795,085 tokens against a declared 300–700k *band*; the verify pass was a synchronous per-finding refuter fan-out. REQ-21 leg 1 requires a declared fan-out shape + agent-count ceiling (a band is neither); AC-23 names "a refutation implemented as a synchronous per-finding verifier fan-out (instead of across-round non-resurfacing)" as the caught bypass. It is the **same escape as the recorded mdatron#11 precedent** (39 agents / 2.27M). Route: escape-corpus entry — control REQ-21 legs 1+2; trace = harness usage totals (could-not-check grade, local, un-synced); gradient-shift today = none (advisory).
- **C2 — the Phase-3 primer never received REQ-21.** Its pre-session template carries "Cost budget (per-swarm-invocation token band; per-session wall-clock budget)" — band only. The #840 Part-2 primer pass did not land REQ-21's fields: *authored is not exercised*, live. Route: governed-corpus edit under Part-2 process (owning domain + cold review; SO ratifies): template gains fan-out shape, agent-count ceiling, hard token ceiling, refutation-across-rounds, and the operator-rendered plan (B1).
- **C3 — REQ-21 leg 2 is partly inaccurate.** "Runtime admission control does not exist today" is false for the Workflow path: the harness budget directive is a hard *token* ceiling (agent() calls throw past it). The *agent-count* interceptor remains unbuilt. Route: small amendment to the subsystem text; the escape-corpus entry notes the available leg. (Ch27 "invest in attribution… tokens"; Ch18 "expect countable failures to increase as fan-out increases.")
- **C4 — REQ-17 bootstrap marking.** In-session orchestration acts must be *marked* hand-audited (AC-17); the #867 declaration recorded shape/isolation/budget but not the marking. Conduct; the session memory `orchestration-legibility-preference` already carries the operator's plain-words rule (convention-grade; REQ-21 is its ratified home).

## D. GH#33 answer (ready — #865; outward post awaits operator go)
From the contract + the book: (1) **coverage** = recorded actuals per check family per output form on the *current* build, occasion-predicated — a family that never ran on this build is could-not-check, not covered; a "representative shapes" table is a calibration band, not evidence; (2) **latency** is a dimension on the same record (native units already include wall-clock) — no second ledger; (3) **staleness** = a completed cycle with no linked refreshed actuals (the contract's falsifier), re-triggered by any change to model, prompt, harness, or output contract — not by "release"; (4) native units recorded, dollars derived through a declared billing context (A1). Ch21 corroborates all four.

## E. Upstream asks
- **E1** The crosslink **transcript sync + corroboration tamper-evidence channel** (REQ-10 / REQ-23 leg 2 dependency) is **unfiled upstream** (searched 2026-09-18). Ch22's "finance enters the chat" class (cost invisible where no record is emitted) is the priority argument. Route: file on dollspace-gay/crosslink; add to the fork epic magnificentlycursed/crosslink#3 under Epic 2. Outward — operator go.

## F. Deviation registry
No predicate changes, no new entries: Workflow-path dispatch is already covered by `manual-dispatch-fallback`. C1 is a conduct escape, not a registry deviation.

## G. Terminology grounding (Ch1)
"Guardrail" in the book = production-validation controls (feature flags, progressive delivery, canaries, SLOs, characterization tests, rollbacks) that let you "dial it up incrementally… trace your intent into production and validate precisely what it is doing." vsdd's "guardrail" = a slice's enforcement control. Compatible (both: a control that permits speed safely); no rename.

## Operator decision slots (batched)
1. **B5** — SO ruling: materialized regenerable dispatch rows vs pure reader.
2. **C2 + C3** — authorize the governed-corpus edit cycle (Phase-3 primer + REQ-21 leg-2 correction) under the Part-2 process; dispatched per REQ-17, or in-session marked hand-audited during bootstrap.
3. **D** — go to post the GH#33 answer (closes #865).
4. **E1** — go to file the transcript-sync ask upstream + on fork #3.
5. **B1–B4, B6–B9** — confirm they enter Slices 6/7 (and #839, Slice 4 residual) as design inputs at their phase-1a openings; no action now.
