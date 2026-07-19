---
title: "Operator interview 2026-07-18: intent, root causes, rulings for the agent-first respec"
tags: ["respec", "operator-record", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-07-19
updated: 2026-07-19
---


## Design Specification

### statements of intent

- The purpose of vsdd-cli: a utility that provably enables and enforces the VSDD methodology, automating what vsdd-suite did manually. Session-start intent should expand into the full process (crosslink engaged, correct phase, correct composition, enforcement active) without the operator restating it.
- Interpretation belongs agent-side. The operator shifts feedback mechanisms as close to the agent as possible; the AI Engineer domain and the claude-code-cli supplement were early attempts to work out what to shift and how.
- Determinism belongs to the process, and specifically: the phase is knowable ("what phase are we in" has a deterministic answer), progression is provable, and directives classify into the flow ("that's a new feature, it goes through phase 1a") with crosslink tracking.
- Provability means both in-flight gating and after-the-fact audit. Phase state is a written artifact.
- Crosslink is the required chassis; the suite's two-mode manual parity is retired.
- Observability's purpose is insight, and cost means native units: the operator runs a subscription, so dollars are a projection; the questions are "is 100k tokens a lot, what makes it up, cached or not, would a rewrite reduce it" — concrete, with recorded-evidence answers.

### root-cause reports

- Critical process properties: session entry and finding lifecycle above all; phase-exit verification with review dispatch as part of it; review dispatch is where the wheels come off. Cluster shape, isolation mode, and active sets were suspect vocabulary (later confirmed to have no whitepaper basis).
- Directive orphaning: VSDD held through the first phase-3 review round; the operator then gave directives believed to be phase-4 routing; the agent executed them silently without classifying them against the process, the flow was orphaned (137 findings, 82 orphaned), and the agent never raised it.
- Reification on utterance: things said once became named concepts, then schema fields, then version-compatibility obligations, while the schema was still being designed. A dedicated vocabulary scan was run because coinages caused heavy finding traffic.
- Proliferation routed around validation: mdatron was built to stop document-structure drift and doc-type proliferation; agents got more creative with review-log filename slugs instead. Many of the suite's 13 hooks were per-axis patches for the same class of escape.
- Chassis non-use: the operator believed crosslink was in use during the cycles; it was not (in-session subagents instead of swarm), and nothing was positioned to notice or guide back.
- Specificity drift in both directions: specific words are sometimes literal, sometimes examples of a general rule; agents have over-generalized utterances and under-generalized examples. The wanted behavior: infer scope and state it back at receipt.
- The AI Engineer domain exists partly to solve naming, coinages, vocabulary creep, and abbreviations in a way that sticks, and to deliver efficiency recommendations the operator did not know how to get: reusable artifacts for repeated searches, evidence-based provisioning of reference indexes, model-tier right-sizing for subagents.
- Intent tiers were cut from vsdd-suite in practice; the operator mixed and matched reviewers ad hoc and wants a config (crosslink-style presets, customizable) instead.

### rulings

- Red Gate: keep and mechanize (red suite provably fails against the pre-implementation commit; mutation floor as the standing check; critic-style post-hoc tests).
- DSL scope: narrowed to cross-file and registry validation, the 7 revisions parked behind a re-run falsifiability gate (adopted default, standing).
- Audit-record issues: bulk-close with dispositions on spec adoption; archive rather than bare-close.
- Spec shape: two documents (lean methodology.md plus toolkit design contract).
- Spec register: the spec is held to its own bar (operator voice lives in evidence and conversation, plain register in normative text).
- Evidence placement: separated from normative prose, one evidence line per contract.
- Review rounds: cold re-review after fix passes; operator gates closure.
- Workspace collapse (binary-first plan issue #15): superseded for vsdd; deferred to Phase 1c for mdatron as optional simplification.
- Domain pairs: preserved (generator-discriminator structure); folding a pair's halves is prohibited; the empirical scorecard governs activation, per the domain-value-scorecard knowledge page.
- Knowledge pages for expensive derivations: approved and published (domain-value-scorecard, thermite-assessment, this page).

