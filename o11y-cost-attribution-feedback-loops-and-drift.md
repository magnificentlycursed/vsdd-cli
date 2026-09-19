---
title: "Cost attribution, feedback loops, and drift to danger (OE2e Ch22, 27, 18)"
tags: ["design-input", "reference", "observability"]
sources:
  - url: "http://oreilly.com/catalog/errata.csp?isbn=9781098179922"
    title: ""
    accessed_at: "2026-09-19"
contributors: ["xqjG"]
created: 2026-09-19
updated: 2026-09-19
---

# Cost attribution, feedback loops, and drift to danger (Ch 22, 27, 18)

Source: *Observability Engineering* 2e, Ch 22 (Kesha Mykhailov, Fin), Ch 27, Ch 18 (Hugo Santos + Liz Fong-Jones). Reading context: [[observability-engineering-2e-reading-map]].

## Fin (Ch 22)
- One efficiency metric (resolution rate) with **aligned incentives** (charge only for resolved), improved ~1 pt/month by stacking dozens of fractional A/B wins. Capability was deliberately prioritized over speed until customers complained.
- The fix required a **customer-centric end-to-end signal measured where the user is** (time to first token on the frontend): "every change we made to the system had to positively affect" it.
- First attempt failed: engineers could answer *what* but not *why* — traces lacked **low-level spans at the points where control flow is handed off**, and the naive serial model of the transaction was wrong (overlapping/parallel work; a real critical path). Re-instrumented; then found wins (eager LLM requests: −2 s median).
- Protect gains with an **event-based SLO** (% of interactions faster than baseline) — thresholds are too noisy under provider variance.
- "**Finance enters the chat**": eager requests fired even when the agent was never engaged → tokens wasted, **no latency metric emitted, so invisible** → caught by finance's quarterly gross-margin review. Fix validated with finance's own query; then **cost per interaction added as a trace attribute** (the warehouse query took minutes and refreshed daily — useless for engineers) **with an SLO on cost**.
- **Rasmussen drift-to-danger**: three boundaries — acceptable performance, economic failure, unacceptable workload. "The art of observability is to surface those safety boundaries to the operators… a dataset that combines a customer-centric signal with a business-centric one… connected to highly detailed, low-level tracing telemetry."

## Diagnosing the investment (Ch 27)
- Two loops with different governance: **operational** (detect/respond/recover — cost-center) vs **developer learning** (deploy/observe/learn/adjust — strategic investment). Conflating them = "observability prices for monitoring outcomes."
- Firefighting trap: "If your goalie is making 50 saves a game, your defense is not working."
- **Activities vs learning**: "Activities are the means; learning is the outcome." Chaos engineering without attribution is disruption; feature flags without validation are complexity; progressive delivery without detection is a slower pipeline. Diagnostic: "For each practice you've invested in, can you point to the feedback mechanism that turns it into learning? What percentage of the time is it being used?"
- Learning-loop indicators: engineers answer novel questions; deploy frequency up / batch size down; more people can debug; hypothesis→validation time shrinking; issues found internally up, escalations down.
- Advice: cap each investment at its point of diminishing returns; tiered telemetry (high/low by criticality) with **feature-flag-controlled verbosity**; "**Invest in attribution.** Cost is a vital component of architecture, and so are tokens. Tag every request… with the cost of running it"; **per-team budgets** so teams self-govern.

## CI/CD as production (Ch 18)
- CI/CD "are, at their core, production systems for developer feedback"; build observability "has the best ROI" — bounded volume, a human waiting on every request, and "LLMs and agents cost tokens, and thus money, to create pull requests."
- First SLIs: success rate + **end-to-end user time**; also flake rate (% successful retries), main-branch presubmit failure rate, resource use vs commitments. "Transforms the CI/CD system from a mystery into a measurable process."
- **Predictability → cacheability**: deterministic inputs→outputs is "a correctness contract," so results can be remembered. Contention → flakes = "a tail-latency tax" (retries don't disappear, they add time); compare **P90 vs P50 per workflow-trace**. "Expect countable failures to increase as fan-out increases… as coding agents start producing changes at a higher rate." Retries hide failure in the tail: "your P50 barely moved, but sometimes you end up waiting twice as long."
- Treat CI like production: correlate traces with host/cgroup metrics; **experiment** (idle cores under single-threaded webpack; Go elastic with CPU; network-bound pushes).
- Honeycomb: SLO "alert if >5% of builds take >15 min"; but **6–7 minutes** is the threshold below which developers stay present instead of task-switching (their mental state "never gets paged out"). Amdahl's law bounded parallelism; the real bottleneck was 3.5 GB of inter-step data transfer, found only by tracing; BuildKit spans stitched into job traces via `TRACEPARENT`. Custom attributes worth adding: output sizes, cache hit, counts of deprecated patterns.

## Mapping to vsdd
- **Cost per dispatch is a record attribute with an SLO**, not a tracker comment. The 2026-08-10 roast (1.79M vs a declared 300–700k band) is Fin's "finance enters the chat" in miniature; the class to close is **dispatch paths that emit no vsdd record** — a Workflow run today produces harness usage totals but no vsdd event, so its cost is invisible to the gate ([[run-record-capability-inventory]] lists what the harness does expose).
- **Rasmussen for the harness**: performance boundary = review quality; economic boundary = token spend; **workload boundary = the operator's cognitive load** — "shapes I didn't anticipate" is a workload-boundary crossing, and the book says surface all three in real time on the same dataset.
- **Activities-vs-learning is the ratification diagnostic**: every vsdd process artifact (primer clause, declaration, registry entry) should name the feedback mechanism that turns it into learning, or it is reliability theater. The 4-cluster shape + 28 verifiers was an activity without attribution.
- **Per-dispatch hard budgets = the book's per-team budgets** (self-governance): the Workflow budget directive is the harness-grade form; a declared band is convention-grade.
- **What-vs-why**: a run's total tokens without per-agent/per-stage spans is Fin's first failed attempt — unexplainable. Per-agent records with dials are the low-level spans.
- The deferred **per-commit wall-clock budget** (Slice-1 residual) is Ch 18's end-to-end-user-time SLI; the 6–7 minute attention threshold is a design target for attended round duration; verdict instability across repeated review rounds is the harness's flake rate; parallel width raises countable failures (fan-out ↑ → failures ↑).
- Governance split: `vsdd gate` (operational loop, detection) vs Slice 7's efficiency insight engine (learning loop) — different success indicators, per Ch 27's two lists.
