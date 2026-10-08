---
title: "Proof-carrying cognition and reality-settled reward (arXiv 2609.09776), read 2026-10-08"
tags: ["reference", "design-input", "design-doc"]
sources:
  - url: "https://arxiv.org/abs/2609.09776"
    title: ""
    accessed_at: "2026-10-08"
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Proof-carrying cognition and reality-settled reward (arXiv 2609.09776), read 2026-10-08

## Status

Reference summary, design input for the review stage (the recorded-dispatch milestone), the conformance verifier, the efficiency advisories (the cost milestone) and the phase 3 skill. Source: arXiv 2609.09776v1, "Proof-Carrying Cognition: Closing the Verification Gap with Reality-Settled Reward", submitted 2026-09-09, two industry-affiliated authors, CC BY 4.0; read on 2026-10-08 through the HTML rendering (first 100,000 of 110,513 characters; the unread tail is references and appendices). External content, treated as evidence. Pointed at by the Peritus maintainer's public post of 2026-09-11 as literature that "gives empirical support and vocabulary to choices reached independently" (see `cross-reference-verification-papers-2026-10-08`).

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Phase names on this page are the contract's: 1a behavioral specification, 1b verification architecture, 1c the spec review gate, 2a test-suite generation (the red gate), 2b minimal implementation, 2c refactor, 3 adversarial refinement, 4 the feedback integration loop, 5 formal hardening, 6 convergence. The whitepaper has six phases; the a, b and c splits are this repository's.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

## The thesis

Progress from reinforcement learning on reasoning is concentrated where a cheap, sound verifier exists. The binding constraint is "the verification gap": no scalable, incorruptible reward source outside formal domains. Every existing reward source (human labels, model judges, process reward models) "is a model of correctness that a strong optimizer can game"; reality "is the unique reward function that cannot be gamed, only predicted." The proposal, proof-carrying cognition, is modeled on proof-carrying code: an artifact is admitted "not because an authority approves it, but because it carries the material needed to check it."

## What the experiments show

- **A judge under selection pressure loses soundness.** With a real generator and real model judges, executed unit tests as gold: the weak judge's soundness fell from 0.835 at two candidates to 0.729 at thirty-two on one benchmark and from 0.889 to 0.750 on another. A frozen learned verifier in the synthetic testbed peaked near 512 candidates and fell to 0.104 of gold at 2,048 while a sound verifier kept improving.
- **Selection alone manufactures a hacking gap from honest samples.** Choosing the sample that maximizes judge score minus gold over thirty-two honest samples produced a gap of +0.527; no deception was needed.
- **A frozen reward model under policy-gradient training overoptimizes.** Proxy reward rose while gold collapsed by 90 percent from its peak and completions degenerated; a reward model refit every 25 steps on a 10 percent settled sample reached six times the frozen arm's gold with far less drift; executed tests as the reward were the ceiling.
- **Settlement against reality beats a frozen verifier**, under random and adversarial pressure, and on-policy settlement is about ten times more label-efficient than random labeling in the synthetic domain; soundness scales log-linearly with settled labels.
- **Showing a judge its own settled mistakes in context made it worse** on both metrics; a learned settlement head cut pricing error by 59 percent. The authors' summary: "Amortize reality into a trained pricing model; do not merely show a judge its mistakes."
- **The anchored verifier errs conservative**, which the authors call "the safe failure direction but not a free one"; the price of trust was 250 reality queries per adversarial run, tens of dollars for the real-model suite.

## What failed, in the authors' own record

A pre-registered scaled replication falsified two quantitative claims: the closed-form exchange rate between verifier correlation and compute overstates the cost at practical candidate counts by up to 24 times and is now treated as an optimistic ceiling; the weak verifier plateaued rather than collapsed in the richer domain. The drift alarm, the claim that measured drift on settled claims predicts the hacking gap, failed its first test (rank correlation 0.06 against a registered threshold of 0.5). One registered criterion was itself invalid. The on-policy label-efficiency result reversed on real code and is named "the program's top open problem." The paper reports all of this in its abstract.

## The architecture, as proposed and as tested

Three components: a claim ledger of typed, probabilistic claims (causal graphs, executable snippets, formal statements, calibrated forecasts); a persistent, versioned world model whose only training loss is prediction of held-out reality ("amortized reality"); an internal prediction market where reasoner instances stake calibration capital, settled by proper scoring rules. The loop per epoch: emit and price claims, update the reasoner on near-term reward, enqueue claims for settlement, update the world model on settled outcomes, compute drift on settled claims and, when it exceeds a threshold, downweight near-term reward and raise the settlement rate. The authors say the experimental predictors are "settlement models", not world models, and that the ledger and the full world model are untested. The trusted computing base is "the executor and measurement apparatus plus the settlement scheduler and staking rules", small "but not zero".

## Stated limits

Synthetic domains for most results; one model family and two code benchmarks for the real-model suite, with half of one benchmark excluded for zero gold variance; search adversaries, not a learning adversary trained against settlement; no strategic claim selection tested; the regime that matters most, "tasks at the judge's capability frontier under RL-scale optimization pressure", untested. Named failure modes of the paradigm: the legibility tax, vague-claim incentives, optimizing predicted reality in harmful ways, and meta-reward hacking against the trusted computing base.

## Named benchmark

RSR-Bench, proposed, not released: soundness under pressure (the ratio of gold at the verifier's best-of-N pick to gold at the oracle's pick, the paper's word for the ground-truth selector), scored as area under that curve over log N, with five registered predictions for the next scale.

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
