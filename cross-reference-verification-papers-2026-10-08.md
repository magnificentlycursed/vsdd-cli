---
title: "Cross-reference: the two verification papers, the Peritus maintainer's reading of them, and this estate's design (2026-10-08)"
tags: ["design-input", "review", "design-doc"]
sources:
  - url: "https://bsky.app/profile/dollspace.gay/post/3mvbp7qxpy225"
    title: ""
    accessed_at: "2026-10-08"
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Cross-reference: the two verification papers, the Peritus maintainer's reading of them, and this estate's design (2026-10-08)

## Status

Design input, 2026-10-08. Two papers (`vero-formally-verified-repositories-2026-10-08`, `proof-carrying-cognition-2026-10-08`) read against the Peritus maintainer's public thread of 2026-09-11 that pointed at them, against the contract, and against the day's reference-practice findings. The thread's text, in order: "Peritus already embodies ideas the literature is only now measuring like persistent external state instead of trusting model context. Mechanical execution evidence outranking agent confidence. Explicit staged workflows rather than vague roleplay coordination. Long-running iteration with warm artifacts and revised plans. Separation between agents that propose work and machinery that accepts it. Deliberately spending time and tokens for assurance instead of optimizing for flashy one-shot scores. The papers give empirical support and vocabulary to choices reached independently. They also reveal the next frontier: evidence ancestry, event-triggered grounding, adversarial canaries, typed harness mutations, and repository-level proof architecture." The quoting post adds: "Peritus is scientifically on the right path. Now we tighten the ratchet." Source: the thread at `bsky.app/profile/dollspace.gay/post/3mvbp7qxpy225` and its quoted post, read through the public API on 2026-10-08. None of what follows is a decision.

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Phase names on this page are the contract's: 1a behavioral specification, 1b verification architecture, 1c the spec review gate, 2a test-suite generation (the red gate), 2b minimal implementation, 2c refactor, 3 adversarial refinement, 4 the feedback integration loop, 5 formal hardening, 6 convergence. The whitepaper has six phases; the a, b and c splits are this repository's.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

"This estate" means vsdd-cli together with crosslink and mdatron, the two sibling tools it runs on.

## The six embodied ideas, checked against the papers and this estate

| Idea | In the papers | In this estate's contract | Note |
|---|---|---|---|
| Persistent external state instead of trusting model context | Proposed (a claim ledger and a versioned world model), untested; the grader re-renders onto a pristine copy | The state artifact; the hub as record; events derived at query time; the dispatched agent re-reads after compaction | The paper's negative result matters more than its proposal: showing a judge its settled mistakes in context made it worse. External state must be mechanized into the check, not reminded |
| Mechanical execution evidence outranking agent confidence | Central to both. Only machine-checked proofs count; a judge's soundness falls as candidates rise (0.835 to 0.729); a frozen reward model's gold collapses 90 percent; selection over honest samples alone manufactures a hacking gap of +0.527 | The operator authors the oracle; agent assertion plays no part; the executed-test discipline; could-not-check | The strongest external support the contract has had. Also a direct warning about review rounds: a reviewing model under selection pressure is the measured failure mode, and the domain scorecard's "rounds past the stop signal manufactured seven of eight hallucinated findings" is that effect observed here |
| Explicit staged workflows rather than roleplay coordination | Vero's curation pipeline with a human approving each stage; the paper's four-step settlement loop | Phases, gates, the dispatch preflight | Neither paper tests roleplay; the support is by construction, not comparison |
| Long-running iteration with warm artifacts and revised plans | Vero's main negative finding: agents treat their own definitions as fixed and never revisit the implementation when proofs stall; sixteen attempts at one lemma where one edit would do | Waves and amendments during the build (the reference practice); the retrofit form | The paper argues for a loop step that reconsiders definitions on repeated failure; the ExoMonad wave boundary is where that step lives |
| Separation between agents that propose and machinery that accepts | Vero's independent grader (slot-scoped re-render, axiom allowlist, declaration screening; 14 percent of residual failures rejected as cheating); the paper's trusted computing base, "small but not zero" | Gates as commands run by CI; the oracle the agent cannot author; Peritus's authorize-then-apply | Both papers put the acceptor's own integrity in scope: the acceptor is attacked next (meta-reward hacking; a repair draft that was itself unsound) |
| Spending time and tokens for assurance | Effort is superlinear in yield under an all-or-nothing metric: 3.1 times the spend, 13.5 times the full solves; the price of trust is 250 reality queries per adversarial run | Cost is knowable; effort dials per lens and stage; "the review budget follows the evidence" | With a sharp caveat from Vero: the yield is superlinear only against a full-solve metric; per specification the same spend buys little, and 23 percent of spend went to instances nobody solves. The metric decides whether assurance spend pays |

## The five frontier items

| Item | In the papers | Nearest thing here | Reading |
|---|---|---|---|
| Evidence ancestry | Vero's manifest records upstream repository, license and pinned commit per instance; the paper's claims carry settlement handles but no provenance chain | Provenance tags on every figure; content-hash inputs on dispatch manifests; the knowledge-page provenance proposal | Present in data form; the ancestry chain (which evidence rests on which) is not specified anywhere, here or in the papers |
| Event-triggered grounding | The settlement rate rises when measured drift exceeds a threshold; settlement is time-lagged, by events. The drift alarm failed its first test | Register retest triggers (date, issue state, artifact presence) re-ground a deviation on an event | The same shape; the paper's failed alarm is a caution that a drift signal may not predict harm, so such triggers stay could-not-check until validated |
| Adversarial canaries | Not named in either; nearest are the settlement-aware adversary, audit by resettlement of a random claim fraction, and Vero's anti-cheating screens | Negative-case fixtures with a clean twin; seeded coinages; mutation | The canary form, a planted known-bad input the control must catch on every run, is what the control-effectiveness registry's fixture pairs are |
| Typed harness mutations | Not present; the paper has typed claims only | The registry's negative-case fixtures, which mutate the harness's input to prove the control fires | If "typed" means a closed vocabulary of mutation kinds per control class, that is a data set this estate could author |
| Repository-level proof architecture | Vero's main finding: the bottleneck is organization, reusable lemma libraries and shared invariants, with pass rates halving at helper depth four | No proof execution by declaration; the analog is shared oracles and fixtures as a library rather than per-criterion tests | The lesson transfers off proofs: a fixture corpus organized per criterion is the "local proofs per specification" failure; shared oracle libraries are the architecture |

## Three further insights from the papers for this estate

- **The reviewer is a verifier under selection pressure.** The soundness-under-pressure result is the mechanism behind the scorecard's finding that late, broad rounds manufacture findings. It argues for a small number of candidates per judgment, a mechanical gold wherever one exists (executed tests, mutation, the diff-only verifier), and a written stop rule, rather than for more reviewers. It also argues against feeding a reviewer its own past errors in context as a remedy.
- **The acceptor can be wrong, and an agent can prove it.** Vero's audit route let agents certify that a specification was unsatisfiable or a reference wrong, and 34 of 38 benchmark defects were found that way, including one that survived human review. This estate routes disputes with the oracle to the Solution Owner as prose; the attested form is a machine-checked negative certificate submitted through the same acceptor.
- **The all-or-nothing metric shapes spending.** Vero's cost signature (unfinished runs costing more than solves; a quarter of spend on the unsolvable) is what a per-milestone "complete, not minimal" rule produces when the milestone is too large. It is a quantitative argument for the live self-governance milestone's size and for a stop rule on budget inside the dispatch prompt, which the proptest-praxis skill already carries ("if low on time or budget, stop at a sub-unit boundary and report what landed").

## Adoption candidates, with homes

| Candidate | Home |
|---|---|
| Few candidates per judgment, a mechanical gold where one exists, a written stop rule; no in-context error replay as a remedy | the phase 3 skill; the review config's round budget and stop sensitivity |
| A machine-checked route for an agent to contest the oracle (a failing fixture against the specification), through the acceptor | The gate-execution milestone gate design; the Solution Owner change authority's dispute path |
| Canaries as the registry's fixture pairs, with a closed vocabulary of mutation kinds per control class | the control-effectiveness registry data set (the gate-execution milestone) |
| Shared oracle and fixture libraries rather than per-criterion tests | The gate-execution milestone fixture corpus |
| A loop step that reconsiders the definition on repeated failure | the phase 2b skill; the wave boundary in the recorded-dispatch milestone |
| Retest triggers and drift signals graded could-not-check until a failed-alarm test is passed | the register; the cost milestone advisories |
| The metric chosen before the spend, because it decides whether assurance spend pays | the Cost member's acceptance criteria; the dispatch plan |

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
