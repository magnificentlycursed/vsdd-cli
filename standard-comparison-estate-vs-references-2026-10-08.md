---
title: "Standard comparison: where this estate's process exceeds the reference repositories, and where it falls short (2026-10-08)"
tags: ["design-input", "review", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Standard comparison: where this estate's process exceeds the reference repositories, and where it falls short (2026-10-08)

## Status

The operator's takeaway from the 2026-10-08 reference-practice reconstruction, recorded as design input for the regime and scope decisions (index: `vsdd-in-practice-reference-repositories-2026-10-08`). The references are Thermite, Peritus, OpenClaudia and crosslink as practised by the methodology's author.

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

"This estate" means vsdd-cli together with crosslink and mdatron, the two sibling tools it runs on.

## Where this estate's contract is stricter than every reference

- A mechanized red gate for every increment at both scales, with executed-test discipline, removal lanes, invocation stamps and retrofit forms. No reference has one for feature increments.
- A multi-domain cold review of both the specification and the code, with pair separation, validator-differs-from-owner and a terminal verify round. Every reference uses one read-only reviewer plus CI.
- Proof from the trace that the phase skill, reviewer roles and rules files were loaded, with a control-effectiveness registry, could-not-check grades and spend-shape bounds. Nothing comparable exists; Thermite's control-plane gate checks that hooks are wired, not what an agent read.
- Signed dispatch manifests, approve-then-dispatch, three-valued preflight, launch-failure detection. The references record dispatch as hub comments and locks and let the human own the push.
- Directive reconciliation, the priced bill of materials with provenance on every figure, and an owned amendment route for every contract change. None has a counterpart.

## Where the references are stricter, and these are the mechanical ones

- A requirements registry with typed evidence bound to code, CI-checked (Thermite); an obligations file and a traceability table (Peritus). This estate's criteria are prose with a sentence for status.
- Read-before-edit over a route table, and document drift pinned to the governed code (Thermite). This estate's is a contract member, unbuilt, and a pin between two documents.
- Architecture as policy on every commit: layers, line limits, forbidden names, owned exceptions (Peritus). This estate governs markdown, not code structure.
- A review ledger with a fresh reviewer identity per record from a different model family, and an authorize-then-apply rule that stops a source edit from approving its own review (Peritus). This estate's trust boundaries admit that signing cannot tell an operator from an agent.
- Anti-stub gates before the edit (Thermite), proofs from the first slice (Peritus), mutation testing actually running (both). This estate's 80 percent floor has no mutation tool installed.

## The pattern

This estate invested in governing the agents' process; the references invested in governing the code and its evidence. Their controls are cheap per increment (a hook, a registry row, a digest) and were exercised. This estate's are expensive per act (a round of twelve reviewers, a ratification cycle) and were not built. The estate's own law, authored is not exercised, applies to it most of all: its highest standards are its least exercised. The references paid for their lower ceremony with known gaps: zero verified slices in OpenClaudia, a release policy never reached in Peritus, five dormant weeks of gates in Thermite. Neither side is simply right, but only one side shipped.

## Where the two meet

Three of this estate's members already state what the references practise and would need no amendment, only building: action-time activation (Palimpsest's exposition through need); authored is not exercised (Thermite's control-plane lesson); the operator authors the oracle (Peritus's human approvals). The members that have no counterpart anywhere are the ones to price before building: the multi-domain roster, the trace-based conformance proof, and the dispatch ceremony.

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
