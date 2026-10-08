---
title: "Reference practice: prose control, and its cross-domain application to this estate's governed corpus"
tags: ["design-input", "process", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Reference practice: prose control, and its cross-domain application to this estate's governed corpus

## Status

Design input for the phase-skill rewrite and the register supplement. What Thermite and Palimpsest encode about agent prose, read on 2026-10-08 by the orchestrating session, and a mapping of Palimpsest's book-writing concepts onto phase skills, other skills, designs and documentation (index: `vsdd-in-practice-reference-repositories-2026-10-08`). The operator's own designs in this space (the register spec, the vocabulary registry, the concrete-referent and no-coinage rules, the Documentation Reviewer and Technical Writer domains) are the baseline the mapping is measured against.

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

"This estate" means vsdd-cli together with crosslink and mdatron, the two sibling tools it runs on.

## Thermite: a register standard bound by one rule

The standard is a 3.5 KB document (`.design/tone-and-voice.md`) bound by one rule in the locked goal statement and carried into the doc-author and builder agent definitions.

- **Affirmative, not defensive.** Much existing prose "argues with an imagined skeptic"; remove that register. State what the system does and why; cut defensive scaffolding; state a real limitation once as a plain fact.
- **Plain, not emphatic.** No capitals for emphasis; no intensifiers used as emphasis (exactly, precisely, actually, genuinely, deliberately, truly); no dramatic framing. "Exactly" is judged claim by claim and kept where it states an if-and-only-if.
- **Narrative stays localized** to introductions and conclusions; mechanism descriptions, requirements, architecture, API documentation and comments stay technical.
- **The tics:** the antithesis pair ("not X, Y"), virtue adverbs ("honestly", "loudly", "cleanly"), dash drama, rhetorical bold, cute asides.
- **Preserve substance.** "A register change, not a content change": claims, numbers, identifiers, requirements and structure unchanged; never soften a true specific guarantee into a vague one.
- **The mechanism statement:** "residual emphasis is what a downstream agent anchors on and drifts toward, so removing it keeps later work from reading meaning into tone." The builder definition repeats it: "Tonal residue in comments is what later agents drift on."

The comment pass ran as a gated base slice: other work paused until it landed and then rebased onto it; one agent per crate paired with an adversarial verifier whose only job was to confirm the diff is comments-only and no identifier or semantics moved; scoped by tic counts treated as upper bounds, not work units; smallest and cleanest crate first to set the pattern; a "context already done, do not redo" section.

Thermite also generates its agent-facing language definition from the same registries the implementation consumes and holds it to a 6,000-token budget in CI: a budgeted, generated prose artifact with no version skew and no cold-start corpus problem.

## Palimpsest: a prose harness at corpus scale

Palimpsest is a book-writing harness (Rust core, September 2026) built from one 142 KB root design document and fifty architecture documents of 2 to 11 KB, one concept each.

**Philosophy.** Separate creation from judgment. Read freshly, with isolated reader agents whose knowledge is restricted to what the manuscript has taught them, because fresh readers "possess something the author lacks: ignorance of authorial intent." Revise large before small. Preserve disagreement: "editorial claims should retain evidence, uncertainty, and assumptions rather than collapsing into a single quality score." Protect intentional weirdness: "the system must not optimize every irregularity into competent beige paste." Treat revision like refactoring: dependencies, impact radii, regression risks, provenance, reversible history. Plans are hypotheses; the text may teach the system what the book became.

**Mechanisms.**

- *Protected intent.* Intent tags with identifiers and stated effects ("Chapter 9 should feel disorienting"). "Reviewers may report reader effects. They may not silently fix protected choices." The editorial question becomes "is the intended effect working?" rather than "is this unusual thing conventional enough yet?"
- *Tickets that separate diagnosis from prescription.* Title, severity, claim, evidence, reader effect, qualitative confidence, explicit assumptions, required outcome, rewrite radius and optional suggested solutions are distinct fields; solutions do not bind the reviser. Duplicate detection runs on the normalized claim and target set, "so different prescriptions cannot turn one criticism into multiple votes." Resolution requires stable evidence and an independent verifier; "the latest implementation actor cannot verify its own work"; verification "must repeat the exact required outcome it claims to establish, so changed words cannot masquerade as a solved reader problem."
- *Voice as language decisions.* Typed tendencies per character and context (vocabulary, syntax, directness, euphemism, metaphor, formality, evasion, concealment); "there is intentionally no catchphrase field."
- *Canon levels* (canon, planned, provisional, exploratory, deprecated) on records; a promise ledger; a knowledge and continuity ledger separating world truth, character knowledge, reader knowledge and author knowledge.
- *The failure catalogue names accumulated default voice:* repeated rhetorical forms, identical emotional cadence, predictable paragraphing, default gestures, familiar phrasing, homogeneous dialogue; each scene acceptable, the whole recognizably machine-made. And goodharted prose: text written to satisfy the lint.
- *The literary lint platform.* Named, versioned profiles that inherit, with complete local overrides; four policies: forbidden (error), report (configured severity), watch (a density threshold in occurrences per thousand words), intentional (a protected observation with a required author reason, retained in the effective configuration and never silently disabled). Phrase families with exact and fuzzy forms, versioned by the catalogue's content hash so a changed catalogue invalidates old verdicts. Findings reference stable component identifiers and content hashes rather than line numbers. Analyzer failure produces an error, never a partial report or a clean bill. Allowed and intentional observations remain visible.
- *The texture auditor* produces heatmaps: low syntactic variance, semantic reiteration, paragraph-length regularity, sentence surprisal below the manuscript's own baseline, excessive explanatory closure, "without pretending the metric itself knows good prose."

## Cross-domain mapping to this estate's governed corpus

| Palimpsest concept | Analog for phase skills, other skills, designs, documentation | Already present | Candidate addition |
|---|---|---|---|
| The manuscript is not the database | registry data is canonical; phase skills, other skills and rules files are compiled projections | the build-plan as projection; the composition milestone generator | every skill, phase or other, generated or pinned, never hand-kept |
| Reader contract | per document: who reads it, at what phase, what it must leave them able to do, what it may assume | the retrieval-friendly rule | a stated reader contract on each skill, testable by a fresh agent |
| Protected intent with intent tags | sanctioned terms and deliberate unusual choices carry a recorded intended effect; reviewers report the effect, never silently fix | variant prohibition; the maturity lifecycle | intent tags with effects; cold review tests the effect |
| Canon levels | canon, provisional, exploratory, deprecated on every governed document and section | standing or resolved register entries; retired designs | a required level field, with a rule that canon cites only canon |
| Promise ledger | every "pending", "owed", "the named future", "lands in the next milestone" sentence is a promise with a payoff location | nothing; the 2026-10-08 reconciliation ledger was this by hand | a promise record as data, checked for a payoff or an explicit cancellation |
| Reader knowledge versus author knowledge | what a dispatched agent has been given is computable from the composition; a phase skill may not cite what the milestone did not deliver | action-time activation; the link family | a check that every reference in a phase skill resolves inside the dispatched increment |
| Hierarchical planning, chapter briefs, scene cards | contract, milestone design, increment brief, commit | contract and build-plan | the two middle levels (the stage document and the kickoff plan) |
| Information before terminology; exposition through need | define before use; deliver a rule where the act needs it | the no-coinage rule; action-time activation is this verbatim | a define-before-use lint for phase skills |
| Fresh readers, synthetic reader, reader calibration | cold review, plus calibration: fresh agents given only the phase skill and a fixture behave as intended | the convergence corpus for the phase answer | the same oracle for phase skills: fresh sessions, one fixture, did the required acts happen |
| Diagnosis separate from prescription; required outcome; rewrite radius; independent verification repeats the outcome | review findings on prose | owner, validator, evidence-graded closure | required outcome and rewrite radius as fields; a fix pass closes by re-testing the outcome, not by diffing words |
| Revision like refactoring; impact reports | an amendment's impact radius across phase skills, other skills and rules files | pin-based drift after the fact | an impact report before the change, from the route table and link graph |
| Diagnostic freeze and regression reports | a corpus-wide baseline before a release; regressions against it | the conformance engine in CI | a recorded baseline so a verdict is a delta |
| Literary lint policies | the conformance engine's vocabulary and register families | forbidden terms and anti-patterns | watch density, intentional exceptions with reasons that stay visible, catalogue versions bound by hash, findings keyed to anchors not lines |
| Texture auditor | corpus-level view of the tics across the governed documents | per-file checks | a density view across the corpus, as a pointer for editors, not a verdict |
| Controlled redundancy | what may be repeated (the phase pointer after compaction) versus what has one home | "stated once" in the slice obligations | a written redundancy policy; the Rust guidance's three homes was the failure |
| Separate creation from judgment | the operator authors the oracle; Solution Owner change authority | present | none |

**What does not transfer:** the fiction machinery (scene mechanics, emotional architecture, reveal calibration, genre delivery). **What transfers partly:** "no single quality score" fits review findings if confidence becomes a field separate from severity, which Palimpsest does and this estate does not; the goodharted-prose warning already applies to the way the label ban was satisfied by heading-name citations.

**The ones that pay off first for the phase-skill rewrite:** the reader contract, intent tags, canon levels and fresh-reader calibration, because together they say what a phase skill is for, protect what is deliberate in it, say which version governs, and give it a test. The promise ledger and the impact report pay off for the contract and the milestone documents. The lint additions are raises to the conformance engine under the three-question rule, since they are shape-tier checks over markdown.

**For the register itself:** Thermite's practice is the model: one short standard, bound by one rule, carried into every dispatched role's definition, with a comment pass run as a gated slice paired with a diff-only verifier. This estate's register spec and vocabulary registry are the same instrument; the contract already states the mechanism ("agents anchor on the register they read").

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
