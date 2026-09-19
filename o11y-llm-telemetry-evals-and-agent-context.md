---
title: "LLM telemetry, evals, and the agent context layer (OE2e Ch21, 10, 8, 7)"
tags: ["design-input", "reference", "observability"]
sources:
  - url: "http://oreilly.com/catalog/errata.csp?isbn=9781098179922"
    title: ""
    accessed_at: "2026-09-19"
contributors: ["xqjG"]
created: 2026-09-19
updated: 2026-09-19
---

# LLM telemetry, evals, and the agent context layer (Ch 21, 10, 8 §GenAI, 7 §agents)

Source: *Observability Engineering* 2e, Ch 21 (Phillip Carter, Honeycomb Query Assistant/Canvas/MCP cases), Ch 10 (Boris Tane), Ch 8 §"Automating Analysis with Generative AI", Ch 7 §"Using AI Agents to Instrument Your Code". Reading context: [[observability-engineering-2e-reading-map]].

## Telemetry design for LLM applications (Ch 21)
- Predictability vs nondeterminism is squared by **instrumentation + evals**, not by controlling inputs: "you must test in prod."
- For agentic apps a discrete operation's failure is not the signal; **task completion** is. Cost is a first-class dimension: "the trade-offs between token usage and user satisfaction."
- Follow the **OpenTelemetry GenAI semantic conventions** (token counts in/out, model, sampling params, system prompt on client spans). Large inputs/outputs: **store a link to the system of record as the attribute**, not the payload. Include **cache read/write tokens** from the provider response on spans — cache behavior is a top cost lever. Track provider rate-limits/retries/timeouts with **SLOs, not threshold alerts**.
- **Cost: store (model, tokens) per event and derive dollars at query time** — "we recommend the latter, as it allows you to easily create eyeball cost differences between different prompt versions or model updates." Never store the dollar figure.
- Add **user feedback signals** (thumbs, retry) as attributes — fast-but-wrong doesn't spark joy; latency proxies fail for LLM UX.
- Telemetry must capture **the connections between the AI part and the rest of the system**, not just the LLM calls.

## Evals and the flywheel (Ch 21)
- An eval has **three results: pass, fail, and maybe.**
- **Question evals** (golden data, string-match scoring) vs **task evals** (judgment workflows with no single right answer, scored on **the steps taken** — which requires telemetry of the workflow execution). Canvas task evals track "how many steps it takes… how many tokens it uses, and if the answer contains a few specific conclusions"; LLM-as-judge pass/fail plus a **hand-reviewed sample on a cadence** ("one of the most important parts").
- The flywheel: production telemetry → promote traces to evals → harness improvements → visible in telemetry. Re-run evals on every model/prompt/harness change (that is the staleness trigger). Public benchmarks are misleading; build your own set from real use.
- "**The harness is the deterministic code you're building around the AI**" — tools, input/output correction, plumbing. Query Assistant: 25% error → 14% via programmatic output correction → <1% via prompt. The corrections also revealed an unmet user need.
- Canonical failure mode: **the model imputes meaning** (`is_slow`/`slow_request` read as a feature flag); fixed by a field *description*, not a prompt.

## The context layer (Ch 10) and personas (Ch 8)
- Agents querying without context: "doesn't know what it doesn't know"; naming chaos; enormous data surface; correlation ≠ causation without topology. Poor context → "generic output that looks plausible but does not fit."
- Context layers to maintain explicitly: **topology, naming/aliases, deployment context, known issues, recent incidents, business context.** "The mental model that's disappearing": coding agents remove engineers from the details, so the context layer must be a **maintained first-class engineering concern** — "the context problem is yours to solve." Role shift: "from direct execution toward direction: setting context, reviewing outputs, and making judgment calls at the boundaries where automation is not yet reliable enough."
- Personas: **copilot** (alongside), **commander** (goals, less oversight — Claude Code named), **caretaker** (autonomous, alert/deploy-triggered, "requires a significant amount of knowledge transfer"). "As the autonomy of AI increases, so does the amount of potential error — and the degree of error compounds." Agents are "optimized for confidence" and must "freshly reconstruct the system state each time."

## Agents doing instrumentation work (Ch 7)
- Two modes: **planning** (spec first, no code) and **execution** (give patterns + rules; brownfield code is imitated, good or bad; keep changes review-sized).
- "**Automate the automation**": have the agent write a script that finds and changes N sites, not edit N files by hand. "Telemetry tests" diff diagnostic output before/after.
- **Treat agent rules like software**: versioned, shared "agent skills"; a daily CI job that runs the agent on a toy artifact, emits telemetry, and independently evaluates the output.
- Manage context: subagents for isolated repetitive work; an external task file for long horizons; small chunks; disposable tooling; tailor agents narrowly.

## Mapping to vsdd
- **GH#33 (cost-ledger evidence bar) answers**: (1) rows carry (model, tokens in/out/cache) and lineage; price is a derived view, so a price change never stales a measurement; (2) latency is a dimension on the same wide event — one ledger; (3) staleness trigger = any model/prompt/harness change re-runs the eval set, not "release"; (4) a per-dispatch efficiency record = steps + tokens + specific conclusions.
- **Slice 6 record fields** (with [[run-record-capability-inventory]]): identity (issue/phase/session/run), versions (contract hash, primer, template, model, effort), invariant results (SHOULD hash, WAS ⊇ SHOULD verdict, cap hit), cost (per-agent tokens incl. cache, tool uses), outcome classes, transcript **by link**, operator feedback (kill/override) as an attribute.
- vsdd's *plausible* = the book's **maybe**; *hallucinated* = **imputed meaning**.
- Attended round = commander; container kickoff = caretaker → the injection seam (upstream gh#62) is the caretaker's knowledge transfer, and caretaker dispatches need the strongest invariants.
- The 28-verifier lesson (2026-08-10) is "automate the automation" + "subagents for isolated repetitive work" — batch, don't per-item.
- Domain skills and primers are "agent skills"; the book's daily CI self-test is where live self-governance (Slice 1) was already heading.
