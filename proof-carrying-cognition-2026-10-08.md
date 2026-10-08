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


## Design Specification

### status

Reference summary, design input for the review stage (Slice 6), the conformance oracle, the efficiency advisories (Slice 7) and the phase-3 primer. Source: arXiv 2609.09776v1, "Proof-Carrying Cognition: Closing the Verification Gap with Reality-Settled Reward", submitted 2026-09-09, two industry-affiliated authors, CC BY 4.0; read on 2026-10-08 through the HTML rendering (first 100,000 of 110,513 characters; the unread tail is references and appendices). External content, treated as evidence. Pointed at by the Peritus maintainer's public post of 2026-09-11 as literature that "gives empirical support and vocabulary to choices reached independently" (see `cross-reference-verification-papers-2026-10-08`).

### the thesis

Progress from reinforcement learning on reasoning is concentrated where a cheap, sound verifier exists. The binding constraint is "the verification gap": no scalable, incorruptible reward source outside formal domains. Every existing reward source (human labels, model judges, process reward models) "is a model of correctness that a strong optimizer can game"; reality "is the unique reward function that cannot be gamed, only predicted." The proposal, proof-carrying cognition, is modeled on proof-carrying code: an artifact is admitted "not because an authority approves it, but because it carries the material needed to check it."

### what the experiments show

- **A judge under selection pressure loses soundness.** With a real generator and real model judges, executed unit tests as gold: the weak judge's soundness fell from 0.835 at two candidates to 0.729 at thirty-two on one benchmark and from 0.889 to 0.750 on another. A frozen learned verifier in the synthetic testbed peaked near 512 candidates and fell to 0.104 of gold at 2,048 while a sound verifier kept improving.
- **Selection alone manufactures a hacking gap from honest samples.** Choosing the sample that maximizes judge score minus gold over thirty-two honest samples produced a gap of +0.527; no deception was needed.
- **A frozen reward model under policy-gradient training overoptimizes.** Proxy reward rose while gold collapsed by 90 percent from its peak and completions degenerated; a reward model refit every 25 steps on a 10 percent settled sample reached six times the frozen arm's gold with far less drift; executed tests as the reward were the ceiling.
- **Settlement against reality beats a frozen verifier**, under random and adversarial pressure, and on-policy settlement is about ten times more label-efficient than random labeling in the synthetic domain; soundness scales log-linearly with settled labels.
- **Showing a judge its own settled mistakes in context made it worse** on both metrics; a learned settlement head cut pricing error by 59 percent. The authors' summary: "Amortize reality into a trained pricing model; do not merely show a judge its mistakes."
- **The anchored verifier errs conservative**, which the authors call "the safe failure direction but not a free one"; the price of trust was 250 reality queries per adversarial run, tens of dollars for the real-model suite.

### what failed, in the authors' own record

A pre-registered scaled replication falsified two quantitative claims: the closed-form exchange rate between verifier correlation and compute overstates the cost at practical candidate counts by up to 24 times and is now treated as an optimistic ceiling; the weak verifier plateaued rather than collapsed in the richer domain. The drift alarm, the claim that measured drift on settled claims predicts the hacking gap, failed its first test (rank correlation 0.06 against a registered threshold of 0.5). One registered criterion was itself invalid. The on-policy label-efficiency result reversed on real code and is named "the program's top open problem." The paper reports all of this in its abstract.

### the architecture, as proposed and as tested

Three components: a claim ledger of typed, probabilistic claims (causal graphs, executable snippets, formal statements, calibrated forecasts); a persistent, versioned world model whose only training loss is prediction of held-out reality ("amortized reality"); an internal prediction market where reasoner instances stake calibration capital, settled by proper scoring rules. The loop per epoch: emit and price claims, update the reasoner on near-term reward, enqueue claims for settlement, update the world model on settled outcomes, compute drift on settled claims and, when it exceeds a threshold, downweight near-term reward and raise the settlement rate. The authors say the experimental predictors are "settlement models", not world models, and that the ledger and the full world model are untested. The trusted computing base is "the executor and measurement apparatus plus the settlement scheduler and staking rules", small "but not zero".

### stated limits

Synthetic domains for most results; one model family and two code benchmarks for the real-model suite, with half of one benchmark excluded for zero gold variance; search adversaries, not a learning adversary trained against settlement; no strategic claim selection tested; the regime that matters most, "tasks at the judge's capability frontier under RL-scale optimization pressure", untested. Named failure modes of the paradigm: the legibility tax, vague-claim incentives, optimizing predicted reality in harmful ways, and meta-reward hacking against the trusted computing base.

### named benchmark

RSR-Bench, proposed, not released: soundness under pressure (the ratio of gold at the verifier's best-of-N pick to gold at the oracle's pick), scored as area under that curve over log N, with five registered predictions for the next scale.

