---
title: "Observability Engineering 2e — reading map for vsdd (what was read, why, method, cost)"
tags: ["design-input", "reference", "observability"]
sources:
  - url: "http://oreilly.com/catalog/errata.csp?isbn=9781098179922"
    title: ""
    accessed_at: "2026-09-19"
contributors: ["xqjG"]
created: 2026-09-19
updated: 2026-09-19
---

# Observability Engineering, 2nd ed. — reading map for vsdd

**Book:** *Observability Engineering: Achieving Production Excellence*, 2nd edition — Charity Majors, Liz Fong-Jones, George Miranda, with Austin Parker (O'Reilly, June 2026, ISBN 979-8-341-60841-2). 633 PDF pages, 32 chapters; "twice as long as the original and mostly new or rewritten." Four chapters on LLMs/agents, ten on the organizational side. Several chapters are contributed (Ch17 Frank Chen, Ch10 Boris Tane, Ch18 Hugo Santos, Ch22 Kesha Mykhailov, Ch21 principal contributor Phillip Carter) — treat those as **patterns from one shop**, not standards.

**Why read (operator, 2026-09-18, vsdd-cli #868):** ground the harness goal — a *predictable, measurable, traceable* agent dev-process — and the recurring orchestration problems (unanticipated dispatch shapes, un-priced spend, ratification-as-ratchet) in Platform-Engineering doctrine (per [[terminology-grounding-priority]]: AI-Eng > Platform-Eng > SWE). Impact analysis on Slices 6/7, GH#33, and the deviation registry is the **next step, not done** — see #868.

## What was read (all inline, no fan-out)

| Chapter | Book pp | Relevance | Captured in |
|---|---|---|---|
| 17 Ontologies as a Shared Language for Humans and AI | 311–324 | THE chapter: nouns/invariants/ActionPlan/3 gates/signal parity — structurally vsdd's architecture | `o11y-ontology-invariants-and-the-dispatch-plan` |
| 18 Observability for CI/CD Pipelines | 327–341 | workflow=trace, job=span; critical path; predictability→cacheability; flake tax; build SLOs | `o11y-cost-attribution-feedback-loops-and-drift` |
| 21 Observability for LLMs | 391–402 | telemetry design for LLM apps, evals (pass/fail/maybe), the flywheel, cost derived at query time | `o11y-llm-telemetry-evals-and-agent-context` |
| 10 The Role of AI Agents for Observability | 161–171 | agents fail without context; the context layer; "the mental model that's disappearing" | same |
| 22 Fin's Case Study | 403–415 | north-star signal, what-vs-why spans, "finance enters the chat", Rasmussen boundaries | `o11y-cost-attribution-feedback-loops-and-drift` |
| 27 Diagnosing Your Observability Investment | 479–484 | activities vs learning; two loops; invest in attribution; per-team budgets | same |
| 7 §Using AI Agents to Instrument Your Code | 127–131 | planning/execution modes; "automate the automation"; treat agent rules like software | `o11y-llm-telemetry-evals-and-agent-context` |
| 8 §Automating Analysis with Generative AI | 141–147 | core analysis loop automation; copilot/commander/caretaker personas | same |

## Not read (deliberately)
Ch 1–6, 9, 11–16, 19–20, 23–26, 28–32. Foundations (structured/wide events, sampling, pipelines, datastores) are known from the 1st edition; the organizational Part VI and build-vs-buy are outside the harness question. Candidates if the impact analysis needs them: **Ch 32** "The Most Important Parts of Our System Have Never Been Specified" (571–579), **Ch 25** the two feedback-loop models (451–464), **Ch 6** arbitrarily-wide events (85–111) for the Slice 6 record shape, **Ch 1** §"The Agentic Incursion / Guardrails Are Having a Moment" (16–18).

## Method + cost actuals (for the ledger)
- Text extraction via macOS PDFKit (osascript JXA), no poppler — `brew install poppler` on Intel macOS tried to build LLVM from source and was killed. PDF page = book page + 36.
- ~350 tokens/page front matter, ~600–750 tokens/page body text. Pass 1 (TOC) ≈ 9k; Tier A 49 pp ≈ 30k; Tier B 34 pp ≈ 22k. Whole book inline would be ~400k+ — never do that; read by chapter.
