---
title: "Cross-reference: the agent-generated-tests study against the red gate, the phase 2a findings and the accepted decisions (2026-10-09)"
tags: ["design-input", "cross-reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-09
updated: 2026-10-09
---


## Design Specification

### status

Cross-reference, design input for the phase-skill rewrite (`vsdd-cli#898`) and the composition milestone's increments. Reads `agent-generated-tests-study-arxiv-2602-07900-2026-10-09` (the paper, read 2026-10-09) against the contract's "Phase exit by gate", the phase 2a skill as it stands, and the pages of 2026-10-08: `reference-practice-phases-build-and-review`, `proptest-with-agent-swarms-recursion-wtf-2026-10-08`, `clarity-review-skill-2026-10-08`, `vero-formally-verified-repositories-2026-10-08`, `proof-carrying-cognition-2026-10-08`, and the decisions on `big-picture-and-decisions-2026-10-08`. Written in-session; no ruling is taken here.

Phase names on this page are the contract's: 1c the spec review gate, 2a test-suite generation (the red gate), 2b minimal implementation, 3 adversarial refinement. "This estate" means vsdd-cli together with crosslink and mdatron.

### what the paper measures, in this estate's words

The paper measures the default: what an agent does about tests when nothing makes it do anything. The default is probes. Tests arrive after the fix is underway, are run once or twice as a window onto runtime values, carry four to six prints per assertion, and half of the assertions constrain nothing an empty function body would fail. Moving the volume of that default up or down changes cost by a third and outcomes by two or three points.

That is the behavior the contract's gate member names at its last bullet: "Tests authored alongside the fix and never shown failing first are the assertion-based transition this gate exists to block, at any scale." The paper is the first tier-one measurement of how common that transition is when nothing blocks it: on five of six models, on most tasks.

### against the contract and the phase 2a skill

- **The red gate is defined on the right thing.** "Red-green, mechanized: phase 2a closes only when the layer's test suite fails against the pre-implementation commit and the failing run is recorded." A probe that prints cannot be red; a sanity assertion against an empty body is red for the wrong reason only if nothing exists yet, and the contract's "executed test, never a skipped one" plus the declared failure kinds at fix scale already refuse the trivial forms. The paper does not weaken the gate; it shows why the gate exists.
- **The skill's failure mode is the paper's majority class.** The phase 2a skill says: "test that passes against an empty function body is the failure mode; every Phase 2a test must assert the milestone's named behavior, not just liveness." The paper's sanity and property categories, 50 to 60 percent of all assertions written, are that class, measured. The skill names it; nothing checks it. The paper's classifier is a rule-based pass over the syntax tree that assigns each assertion a category. That is a mechanical census this estate could run over a red gate as a gate leg: a red gate whose assertions are all sanity and property is reported, not failed, until a floor is set.
- **The skill asks for volume where the paper says volume is free.** "All acceptance criteria from Phase 1c have at least one failing test" is a coverage rule, and coverage by count is what the interventions moved without moving outcomes. The unit that matters is an asserting test tied to a criterion whose failure the seeded-removal proof can demonstrate (the property-testing page's discipline: remove an invariant and watch the suite fire). "At least one" should become "at least one that fires on a seeded violation".

### against the 2026-10-08 findings

- **The phases page said no reference has a red gate for feature increments and that mutation substitutes.** The paper explains why that substitution is rational: default tests do not verify, and mutation testing measures whether a suite detects, which is the property the references buy directly. The contract already has the mutation floor as a standing criterion under the thorough preset; the paper raises its weight relative to the test count.
- **The property-testing article is the engineered counterpart.** Its results (thirteen, four, two and more than thirty bugs in repositories with thousands of tests) came from reference models, generators of interacting operations and assertions built on purpose, with a coverage proof by seeded removal. The paper measures agents without that discipline; the article measures agents with it. They are consistent, and together they say the phase 2a skill's job is to supply the discipline, since the default supplies probes.
- **The clarity-review skill's pattern 4 is the paper's category table.** "A test that would still pass if the logic under test were wrong", "asserting non-nil, or a debug print instead of an exact expected value" is sanity, property and print in the paper's terms. A published practitioner's review corpus and an academic measurement name the same defect; the Documentation Reviewer and Quality Engineer roles can cite both.
- **Vero and the proof-carrying paper, on cost.** Vero found effort superlinear in yield only under an all-or-nothing metric; this paper finds test volume linear in cost and flat in yield. The proof-carrying paper found behavior easy to move under selection pressure; this paper finds behavior easy to move by a sentence of prompt (37 to 75 percent of tasks) with outcomes unmoved (83 percent the same). Together: shaping what an agent does is cheap, and only a mechanical check tells whether what it did was worth anything.

### against the accepted decisions

- **The scaffold commit (decision 2) holds, with a condition.** Its failing tests are worth their cost only as asserting tests tied to the increment's criteria; a scaffold of probes would be the paper's default at commit time. The composition milestone's two increments should state which criteria their scaffold tests assert and how each is shown to fire.
- **The phase 2a split (decision 7) gets its sharpest input.** Fixes and critics' tests: failing-first, unchanged. Increments: machinery whose coverage is proved by seeded removal, unchanged. Added: a census of assertion kinds over the red gate, reported at 2a exit, with a floor the review config may declare the way it declares the mutation floor.
- **The cost member and the token-budget gate.** The paper prices the default at a third of a run's tokens. A dispatch's bill of materials that cannot see test-writing cannot report it; the recorded-dispatch milestone's manifest and the cost milestone's report should carry test-writing spend as a line, which the paper's cost-benefit monitoring suggestion asks for in other words.

### what to take, and what not to

Take: the assertion census as a candidate gate leg and as a Quality Engineer report form; "at least one test that fires on a seeded violation" as the phase 2a criterion wording; the paper as the evidence line for the gate member's last bullet; the cost line in the report. Do not take: the paper's magnitudes as this estate's, since it ran one Python scaffold with no gate and no CI, and named enforced CI as a setting where the numbers may differ; nor "a more conservative approach to agent-generated tests" as a rule, since the article's engineered machinery is the counterexample the paper did not study.

Feature candidates, for the sessions that own them: an assertion-kind classifier as a vsdd gate leg or an mdatron code-catalog family (the paper's four categories over Rust test bodies); a test-writing spend line in the dispatch manifest's usage.

### handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-09.

- `vsdd-cli#898`: Phase-skill rewrite (design-first): the ten phase skills, the prose reviewer roles and the register standard, rewritten from the 2026-10-08 reference practice [open]

