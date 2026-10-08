---
title: "Vero: can agents build formally verified software repositories? (arXiv 2608.13522), read 2026-10-08"
tags: ["reference", "design-input", "design-doc"]
sources:
  - url: "https://arxiv.org/abs/2608.13522"
    title: ""
    accessed_at: "2026-10-08"
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Vero: can agents build formally verified software repositories? (arXiv 2608.13522), read 2026-10-08

## Status

Reference summary, design input for the verdict record (the recorded-dispatch milestone), the gate legs (the gate-execution milestone), the cost member and the phase 3 and phase 5 skills. Source: arXiv 2608.13522v1, "Vero: Can AI Agents Build Formally Verified Software Repositories?", 2026-08-13, authors at three universities, a company and a cloud provider, CC BY-SA 4.0; benchmark, curation pipeline and harness released at `github.com/sunblaze-ucb/vero`. Read on 2026-10-08 through the HTML rendering in two passes (full text). External content, treated as evidence. Pointed at by the Peritus maintainer's public post of 2026-09-11 (see `cross-reference-verification-papers-2026-10-08`).

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Phase names on this page are the contract's: 1a behavioral specification, 1b verification architecture, 1c the spec review gate, 2a test-suite generation (the red gate), 2b minimal implementation, 2c refactor, 3 adversarial refinement, 4 the feedback integration loop, 5 formal hardening, 6 convergence. The whitepaper has six phases; the a, b and c splits are this repository's.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

## What it is

A repository-scale benchmark for joint implementation and proof synthesis in Lean 4: 43 multi-module instances translated from Python, Dafny, Verus and Coq repositories, 743 scored APIs and 2,705 scored specifications, each instance with fixed API signatures, curated specifications and a reference implementation, in two modes (proof-only against the reference; code-and-proof where the agent writes both). Scoring counts full solves only, because "partial coverage can be inflated by easy specifications" and only full coverage "certifies the agent's code as correct." Contamination is avoided by construction: no public Lean 4 ground truth exists for the instances. Curation ran as a staged pipeline (discover, select, plan, translate, write specifications, validate) with a human curator approving each stage; "every translated definition, specification, and proof obligation is manually reviewed"; about 60 dollars of model usage plus hours to days of human effort per instance.

## The acceptor, and the anti-cheating layers

The grader is independent of the agent: only marked regions are graded and are re-rendered onto a fresh copy of the pristine instance; an axiom allowlist of three; declaration screening for hollow typeclass instances, priority shadowing, decidability laundering, and splitting the proof target from the runtime function. Across the corpus 368 specification outcomes were rejected at the axiom stage, mostly a decision procedure on finite domains; of the strongest agent's remaining failures, 14 percent "are rejected as cheating." A single unbuildable implementation module voids every specification in the instance; seven of 344 cells ended this way.

**The audit mechanism.** An agent may submit machine-checked negative evidence against the benchmark itself: that the reference fails the conjunction of specifications, that a specification is unsatisfiable, or that individually satisfiable specifications are jointly inconsistent. The curators used it to repair the benchmark: 38 specification defects across nine instances plus six joint-unsatisfiability groups, all repaired before evaluation; 34 of the 38 rest on agent-submitted certificates. One contradiction between a padding rule and a reject rule "survived manual review and type-checking" and was caught by an agent. A second auditing model added only five cases to the first's 33, "real but modest." A first repair draft was itself unsound and a second auditor rejected it.

## Results

| Agent and effort | Full solves, code-and-proof | Full solves, proof-only | Cost, code-and-proof |
|---|---|---|---|
| strongest agent, highest effort | 27 of 43 | 25 | 2,865 dollars |
| same agent, medium effort | 2 | 6 | 928 |
| second agent, highest effort | 8 | 10 | 1,983 |
| third agent, highest effort | 2 | 2 | 633 |

Ten instances resist every configuration. Ensembling the weaker agents adds nothing at the repository level (the union of all eight configurations equals the strongest agent's 33). The strongest agent passes 87 percent of specifications; the gap to full solves is organization, not local skill.

**Effort is superlinear in yield under an all-or-nothing metric.** Moving from medium to highest effort multiplied spend by 3.1 and full solves by 13.5. Per specification the same move bought far less (65 to 87 percent coverage for triple the spend). The strongest configuration had the lowest cost per full solve (106 dollars) and the highest per specification. For three of four agents the median unfinished run cost more than the median full solve, and 23 percent of all spending went to the ten instances nobody solves: "the cost signature of the full-solve metric."

**Proof architecture is the bottleneck.** In 82 full solves, helper theorems hold a median of 72 to 74 percent of proof lines; 80 of 82 share a helper across specifications. Specifications needing no helper pass at about 82 percent; at helper-chain depth four or more, at 39 to 51 percent. The strongest agent "fails to discover shared invariants" and needs "inductive generalization"; weaker agents attempt local proofs per specification "rather than building the reusable lemma libraries." One cell attempted the same bridging lemma in sixteen files; "the retry loop has no step that revisits the implementation," and a loop that reconsidered definitions on repeated lemma failure "would turn sixteen attempts at one lemma into one edit."

**Agents treat their own definitions as fixed.** "A mismatch it introduced in the first minutes becomes a proof obligation it pays for over the remaining ninety"; implementation size plateaus early while proof text grows toward the deadline; agents that finish tend to finish early, while those that do not "spend the remaining budget on obligations they never close." Scratch files mark being stuck: cells with scratch files pass fewer specifications and produce 2 full solves against 16 without.

**Implementation freedom.** In five pairs the agent replaced the reference algorithm with a simpler one that closed every specification (250 against 201 for the fixed reference): an exponential permutation enumeration in place of a 409-line Hungarian state machine, "unusable in production" but close to a transcription of the specifications; insertion sort and linear scan in place of binary search, giving quadratic construction. "A full solve does not certify that the implementation preserves properties the specifications do not mention."

## Stated limits and future work

Lean 4 only; the corpus favors code that translates cleanly; concurrent and temporal protocols are "the main absent class"; incremental maintenance tasks are named as future work; the audit certifies formal satisfiability but cannot ensure a specification is semantically correct or complete; the 90-minute budget partly explains the weaker agents' counts.

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
