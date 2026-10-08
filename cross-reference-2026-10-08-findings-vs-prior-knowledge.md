---
title: "Cross-reference: the 2026-10-08 reference-practice findings against prior knowledge"
tags: ["design-input", "review", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Cross-reference: the 2026-10-08 reference-practice findings against prior knowledge (observability engineering, the Thermite assessments, the domain scorecard, and the portable memory rules)

## Status

Design input, 2026-10-08. The day's findings (`vsdd-in-practice-reference-repositories-2026-10-08` and its pages) read against four bodies of knowledge this estate already holds: the Observability Engineering, 2nd edition pages (`observability-engineering-2e-reading-map`, the three `o11y-*` pages, `oe2e-impact-analysis-2026-09-18`); the two earlier Thermite assessments (`thermite-assessment`, 2026-07-19; `thermite-state-architecture`, 2026-07-22); the `domain-value-scorecard` (2026-07-19); and the portable memory rules gist by the author of Observability Engineering (`portable-memory-rules-gist-2026-10-08`). Each section states what is confirmed, what is contradicted, and what is new. Adoption candidates are collected at the end; none is a decision.

## Against the observability engineering pages

The impact analysis of 2026-09-18 routed nine gaps (B1 to B9) and four conduct escapes (C1 to C4) and listed ten things already satisfied (A1 to A10). The reference repositories bear on them as follows.

**Confirmed, with cheaper forms than we planned.**

- **B1, the dispatch plan as a proposed object validated before launch.** The references have it as a document and a comment, not an engine object: Thermite's kickoff plan (increments, primary files, self-verify commands, per-increment gauntlet, done-when checklist) and its increment issue bodies; OpenClaudia's pre-wave lane audit ("strictly dependency-free slices are S-001, S-002, S-004, S-005; hold S-002 for sequential integration"); Peritus's Parallel ownership tables with worker counts. The explicit ceilings the book asks for (agents, concurrency, build jobs) are recorded in those plans and in receipts. Insight: build the plan as a reviewed document first; mechanize the simulation gate later.
- **B6, findings carry their routing.** Thermite's divergence issues name the authority the code diverges from (a requirement, a golden file, a thesis section), so routing is in the finding. Confirmed as cheap and attested.
- **B8, the wall-clock indicator.** Thermite tracked CI wall-clock as a number in its operations page (a sharding change halved it) and Peritus records resource ceilings; partial confirmation, no indicator discipline.
- **A4, authored is not exercised.** Thermite learned it the hard way (both agent-facing gates dormant five weeks while every document asserted they fire) and built the minimal response: a CI check that the wiring exists plus fixture-oracle tests for the gates themselves. Three independent sources now agree (the book's activities-versus-learning, our law, Thermite's control-plane rule); the reference shows the smallest build that satisfies it.

**Contradicted or absent in the references.**

- **Cost attribution is ours alone.** No reference records tokens per dispatch, per agent or per stage, declares token bands, or prices context. Their budgets are count and concurrency ceilings (three workers, one build job, a 6,000-token generated spec), enforced by the root agent's own queue, not by a hook. The book's Fin lens and our Cost is knowable member have no counterpart in the author's practice. That does not make them wrong; it means B2 (plan versus actual as a verdict), B3 (rejection and override rates), B4 (operator workload) and B5 (materialized dispatch rows) have no attested implementation to copy and must be priced as new.
- **A3, rejecting the agent-writable local transcript as the oracle.** Every reference accepts local evidence plus CI. The one server-side oracle in the references is not a transcript at all: Peritus's review ledger requires that "the exact approved record already exists unchanged on the protected Git base" before a transition is accepted, with a two-step authorize-then-apply pull request so a source edit cannot approve its own review. Insight for the oracle re-home ruling of 2026-10-08 (recorded on vsdd-cli#873): the protected ref is the oracle, the record is a content-addressed verdict, and no transcript sync is needed. This is the first attested mechanism that matches the ruling's direction.

**New insights.**

- **The references' learning loop is "incident becomes a mechanical gate with a named rule".** Thermite's anti-pattern gate, control-plane gate and "fix the cause's whole class" rule; the gist's escalate-bug-to-lint-rule; our contract's own line "a new defect class is closed by adding a pattern". The book's learning-loop indicator (B9) in the references is the count of gates born from incidents. Ours has been the count of contract members and register entries born from incidents, which is the activities side of the book's distinction.
- **Percentile aggregation is a Slice 7 constraint the observability pages did not state.** The gist's rule (never percentile a percentile across units; max per unit first) applies directly to a report that aggregates per-agent or per-stage usage across a round. It belongs in Slice 7's design as a falsifiable property.
- **"Tests pass and merged are checkpoints, not conclusions."** The gist's production-verification rule names, from outside, the failure in vsdd-cli#881 (closure claims without their oracles). The references' minimal form is Peritus's gate re-run on the merged commit on fresh main; for a methodology harness the production signal is the next cycle's conduct, which is what fresh-reader calibration of a primer measures.
- **C1 and C2 (the 32-agent round; the primer that never received the ceiling).** The references and the gist agree on the remedy's form: a number in the dispatch prompt, not a band in a primer. "Precise, falsifiable instructions over vague ones."

## Against the Thermite assessments

**The 2026-07-19 assessment is confirmed in every pattern it named**: the four tool-restricted roles, read-before-edit over a route table, drift pins, the no-self-oracle rule, greppable waivers, register control, the generated budgeted skill, crosslink as tracker, and the deliberate divergences (no red gate; binary finding taxonomy; mechanical stopping predicate).

**What changed after it, or what it did not see:**

- The binary shipped-or-not-started status it praised was superseded in June by the registry's six statuses (RFC-2); partial rows exist. The registry with typed evidence, not the binary rule, is the durable practice.
- The dormant-gates incident (21 June to 29 July) post-dates the assessment, and so does the rule drawn from it: anything load-bearing for a trust claim is CI-enforced and harness-agnostic; authoring-time tooling may never be cited as the reason a property holds. This bears directly on the assessment's standing conclusion to generalize the harness "as data-driven conformance checks plus crosslink-seam installation rather than hand-built per-project scripts": the hand-built scripts are what shipped and were exercised; the crosslink-seam installation is exactly what a re-initialization stripped. The reference's own lesson favors CI legs over hook wiring, whichever is data-driven.
- The kickoff plan document as the layer between the stage document and the increment issues, the design re-pass before each stage, the cadence (design and build in the same month), the RFC process, zero GitHub reviews, and the red gate holding only for critic pins were not in the assessment.

**The 2026-07-22 state-architecture page** ("Thermite's canonical process record is a structured registry with generated views, not markdown frontmatter"; "a schema pass is a shape fact, never a truth fact") now has a second instance and a negative example. Peritus's architecture policy file and obligations file are the second instance. OpenClaudia is the negative example: status lived in prose lines with no registry and no generator, the backlog's own six-value vocabulary was never used in the files, and zero of 108 slices reached Verified. Prose status without a registry drifts; the page's corrective number two (generate human views from canonical data) is reinforced. The page's "no precedent for next-action derivation" still stands: no reference derives the next action; the human orchestrator carries orientation in all four.

## Against the domain value scorecard

The scorecard (five datasets, 2026-07-19) found: value is a function of artifact shape, not domain identity; software engineer, quality engineer and security are load-bearing on code; technical writer and documentation reviewer on prose; red team, UX and data engineer situational; performance, privacy, accessibility and localization zero-yield on non-matching shapes; mutation testing the single most reliable value source; the operator's manual test the best single defect-finder; "spend cold review on code; spend operator attention and external readers on prose"; small early spec-stage review changed direction while 18-domain late cycles were about a fifth substantive and rounds past the stop signal manufactured hallucinated findings.

The references agree with the scorecard at every point where they overlap. No reference uses a domain roster; one reviewer plus CI; mutation testing runs in two of four; the only genuinely cold reviews were external (a trust audit, outside-filed issues); the human's approvals and audits are the strongest signal; review breadth appears nowhere. This is evidence bearing on the cost of the 2026-09-28 ruling that every review composition carries the twelve-domain process-governing set: both the estate's own empirical record and the author's practice say breadth is not where value comes from on most artifact shapes. It is recorded here for the regime decision, not as a reopening of the ruling.

## Against the portable memory rules

**Confirmations of this estate's design from an independent practitioner:** the plan must be human-defined and architecture cannot be guessed (the operator authors the oracle; Solution Owner change authority); document the design before implementing and have it reviewed first (design-first, freeze, verdict); a red test before a bug fix is trusted and a reproduction before fixing (the fix-lane red gate, and the references' failing-first for fixes); evidence before a performance claim (Performance Engineer's enforceable content); path-scoped rule files that load only for matching files (the 2026-10-02 delivery decision, with the author's reason: "instead of bloating every session"); a memory system of index plus topic files written after sessions (the knowledge base and the handoff); escalate a recurring bug to a static rule (the contract's "a new defect class is closed by adding a pattern"); file follow-ups in the system the owner watches (the hub as work ledger); verify an automated review finding before acting on it (evidence-gated filing; the false-positive disposition); check that a review read the current diff (the critic exact-tree rule).

**What it adds that this estate lacks:**

- **Capture confirmations, not just corrections.** The regression corpus, the register, the rulings and the agent memory are all corrections. "Otherwise the system only ever learns caution and drifts away from approaches that already work." The one confirmation form in the design is the clean twin fixture in the control-effectiveness registry.
- **Calibrate warning volume.** A latent, mitigated risk gets one clause. The contract's register repeats its defensive caveats ("could-not-check, never clean") many times; Thermite's standard names the same fault as "arguing with an imagined skeptic". A register supplement should carry the calibration rule.
- **Comprehensibility over diff size.** Together with the references' "complete, not minimal", this retires "minimal" as a virtue in the 2b primer; the virtue is the comprehensibility of the end state, with a minimal diff reserved for refactors that must prove behavior preservation.
- **Scope partial retractions.** A directive-reconciliation sub-rule: a mid-sentence correction cancels only what its reason invalidates; "silently not-doing something is itself a choice that needs the same justification as doing it."
- **Answer questions first.** A session-skill conduct rule: a mid-task question is a gating interrupt; answer it before acting, and pause a gated action until the answer is seen.
- **Never stage all files in one shot.** The kickoff worktree leaves three tracked files dirty after init (recorded on `content-delivery-assessment-2026-10-02`); a blanket add would carry them into a pull request. The gist's reason is secrets; the mechanism is the same.
- **An attribution footer when a tool authenticates as a human.** The agent signs tracker writes with the operator's driver key (Trust boundaries). The gist's rule is the plain-language form of the same problem and its minimal remedy.
- **Scope verification cost to the change**, and run the expensive form before opening the pull request. The per-commit wall-clock budget obligation and Thermite's per-crate gauntlet are the same rule.
- **Delete spike code** once a design converges, including superseded design drafts and pages.
- **Do not cite a search summary as a source.** A research-conduct rule for the primers and the research skill; today's agents were held to it by instruction.

## Against the clarity-review skill

The author's companion skill (`clarity-review-skill-2026-10-08`) is a pre-flight lint for AI-authored code and prose, calibrated against about two hundred real review comments, never invoked by description match, run by the author before the pull request exists, and scoped to one concern. Its seven pattern classes map onto this estate as follows: unverified technical claims are the provenance tags and evidence-gated filing; tautological tests are the executed-test discipline and the mutation floor; metaphor-borrowed jargon and unexplained abbreviations are the concrete-referent and no-coinage rules on the code side; a new exported symbol with no caller is Thermite's consumer rule; "stated once at its most specific home" is the contract's own obligation.

**New insights.**

- **Derive the domains' enforceable content from the review corpus, not from principles.** The estate holds the raw material (the respec's review rounds, the scorecard's datasets, the hallucinated-finding series, the register corrections). The practitioner's list exists because each pattern kept drawing pushback; the Technical Writer and Documentation Reviewer domains could be rebuilt the same way, with a litmus test and a closed report form per pattern.
- **A litmus test per rule.** "If a reader who knows the language loses nothing by deleting it, cut it" is checkable by the author and by a cold reader. The register rules are prohibitions; a litmus is what makes each falsifiable from the inside.
- **A pre-flight author pass is cheaper than a domain in the round.** The skill separates the author's own clarity lint from the bug-finding reviews and from description quality; the estate's phase-3 round does all three with a composed roster. Thermite's and Peritus's self-verify-before-commit disciplines are the same split.
- **`disable-model-invocation: true` is the concrete form of Availability is not activation** for skills that must only fire by invocation.
- **Comment slop is the practitioner's top complaint; coined labels are this estate's.** Same mechanism (the reader anchors on what it reads), and the design-document form of comment slop is prose that restates frontmatter, counts or mechanism.

## Adoption candidates, with homes

| Candidate | Home |
|---|---|
| Record confirmations (what worked and why) as first-class entries beside corrections | the regression corpus's twin, the agent memory, the handoff |
| Calibrate warning volume; affirmative not defensive | the register supplement |
| Comprehensibility over diff size; complete, not minimal | the 2b primer |
| Scope partial retractions; answer questions first | the session skill; Directive reconciliation's conduct text |
| A ceiling as a number in the dispatch prompt, never a band in a primer | the 3 primer and the dispatch prompt template |
| Never stage all files; attribution footer under a human-authenticated tool; scope verification cost | the runtime-harness supplement; the commit skill |
| Percentile aggregation as a property | the Slice 7 design |
| The protected ref as the oracle, with a content-addressed verdict and authorize-then-apply | the oracle re-home design (Slice 6) |
| Incident to mechanical gate, with a count of gates as the learning-loop indicator | Slice 4 and the Slice 7 criteria |
| The plan as a reviewed document first; the kickoff plan as the layer between stage document and increments | the Slice 2 design and the 1c primer |
| Evidence for the cost of mandatory review breadth | the regime and scope decision |
| Derive Technical Writer and Documentation Reviewer enforceable content from the review corpus, with a litmus and report form per pattern | the domain-prompt rewrite |
| A pre-flight clarity lint the author runs before dispatch, separate from the review round, invoked only explicitly | the 2b primer; the commit skill; the paved-path map |
| The seven clarity classes as yes-or-no rules in the supplements and the design-authorship primer | the Rust supplement; the 1a primer |
