---
title: "Ontology, invariants, and the missing noun: the dispatch plan (OE2e Ch17 + Ch18)"
tags: ["design-input", "reference", "observability"]
sources:
  - url: "http://oreilly.com/catalog/errata.csp?isbn=9781098179922"
    title: ""
    accessed_at: "2026-09-19"
contributors: ["xqjG"]
created: 2026-09-19
updated: 2026-09-19
---

# Ontology, invariants, and the missing noun: the dispatch plan (Ch 17 + Ch 18 §ontology)

Source: *Observability Engineering* 2e, Ch 17 (Frank Chen, contributed; energy-billing domain) and Ch 18 §"The Ontology of CI/CD". Reading context: [[observability-engineering-2e-reading-map]].

## The pattern (book)
- **Semantic failure** is the failure class monitoring misses: the same concept measured in several places with different rules → dashboards disagree; "data is everywhere, but truth is scarce." Green infra dashboard, screaming business dashboard, the AI "hallucinating price windows that don't exist."
- An **ontology** = core entities (nouns) + invariants (rules) + relationships, made operational through **semantic conventions** (schemas every emitter uses). "An ontology remains an academic exercise until it is codified."
- For a system with a nondeterministic AI inside, the nouns are: deterministic input, deterministic logic (versioned), deterministic output — plus the **ActionPlan**: "the system's structured interpretation of the user's intent... **untrusted until validated**." Schema: `plan_id, status: PROPOSED, predicted{…}, actions[…], lineage{…}`. It captures *what might happen and why* before anything executes.
- **Invariants** (coverage/completeness, arithmetic/integrity, determinism/reproducibility) become runtime assertions — "instead of testing the input (which is infinite), you use invariants to constrain the output." Simulate the AI's plan against the deterministic engine; reject on mismatch. A "semantic firewall."
- **Lineage in every output** (which input id, which logic version, which engine version) "transforms a simple cost figure into a traceable audit log."
- **Three CI gates**: *deterministic* (hash/arithmetic/coverage, binary), *simulation* (does the plan's predicted delta reproduce within tolerance?), *semantic* (golden dialogs, confidence floor). "When a gate fails, the ontology tells you where to look" — structural→schema, coverage→logic mapping, semantic→prompt/context.
- **Signal parity**: emit the same JSON in CI as in production so "the logical assertion preventing a pull request from merging is the exact same query that triggers an incident in production" and "fixing the build is indistinguishable from fixing the system."
- **Universal payload**: identity + versions (input id, logic id, ruleset version) + invariant results (coverage score, determinism hash) + `model_name`, `prompt_version`.
- **AI sandwich**: nondeterministic layer between two deterministic layers — "physics vs imagination"; "treat the AI's output as a hypothesis and the rules-based engine as the test." The **proposal-rejection rate** is "our highest-fidelity signal for monitoring AI reliability in production."
- **Close the loop**: rejected proposals → scrubbed → promoted to golden dialogs; "eval pass rate" tracked as an SLI, a drop treated as a reliability incident.
- Ch 18's ontology of CI: **workflow = trace, job = span under the root, step = span within job**; a workflow becomes a DAG; only the **critical path** matters for wall-clock — "you have a slow Go build, but your real problem lies elsewhere."

## Mapping to vsdd
- The contract IS the ontology; the deviation registry + gate legs + tracker-join falsifiers ARE invariants; the composition SHOULD is an ActionPlan for *context* (see [[verifiable-conformance-and-efficiency]], [[composition-slice]]).
- **The missing noun is the dispatch plan**: a PROPOSED object with `predicted` (cost, width, wall-clock), `actions` (stages, per-stage agent counts and caps, dials), `lineage` (contract hash, primer version, template version, model). The book's *simulation gate* over it = chosen-vs-observed **mechanized**: the 2026-08-10 roast's 2.56× overrun becomes `proposal_status=rejected, rejection_reason=simulation_mismatch` on a record instead of a tracker comment. This is what the operator asked to see before launch ([[orchestration-legibility-preference]] in session memory).
- **Determinism invariant for composition**: same (phase, domains, supplements) → same composed-context hash. Cheap, binary, first gate.
- **Signal parity is Slice 6's specification**: today the gate evaluates in CI while a live session's conformance is prose in comments. The book's bar: the session emits the JSON the gate checks.
- **Harness reliability SLI = dispatch-plan rejection rate**: plans rejected by the verifier *or killed/overridden by the operator*. "I didn't get why it fanned out to 28" is that SLI degrading, measurably.
- "The ontology tells you where to look" = Phase-4 routing encoded **in the finding** (gate class → fix location), not performed by hand each round.
- Orchestration inherits Ch 18's critical-path lesson: parallel width does not shorten wall-clock (the slowest reviewer + slowest verifier did); it only multiplies cost. Amdahl applies at the orchestrator.
- Caveat: Ch 17 is one contributed chapter from one domain — adopt as a pattern; the invariant *classes* (completeness, integrity, reproducibility) are the transferable part.
