---
title: "Terminology grounding for the 2026-10-08 findings: informal phrases mapped to standard terms and sibling self-words"
tags: ["design-input", "process", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Terminology grounding for the 2026-10-08 findings: informal phrases mapped to standard terms and sibling self-words

## Status

Design input for the vocabulary registry's next update; registration is the operator's act and nothing here is registered. Source: the day's chat, read against the contract's reference lexicons (OpenTelemetry, site reliability engineering, FinOps, internal-controls audit, quality engineering, change management, decision records, document control, software engineering), the reference repositories' own words, and the practitioner sources read today. Rule applied: a nickname may live in chat; governed text carries the grounded term or a plain description; no coined compounds. Claims about usage on this page state which sources were observed; none claims usage beyond them (operator correction 2026-10-08: say how many sources, not "common").

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Phase names on this page are the contract's: 1a behavioral specification, 1b verification architecture, 1c the spec review gate, 2a test-suite generation (the red gate), 2b minimal implementation, 2c refactor, 3 adversarial refinement, 4 the feedback integration loop, 5 formal hardening, 6 convergence. The whitepaper has six phases; the a, b and c splits are this repository's.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

"This estate" means vsdd-cli together with crosslink and mdatron, the two sibling tools it runs on.


Operator stance recorded 2026-10-08: term changes are welcome where they make things more understandable, even when the change is annoying; no proliferation of small rules by accident; no coinages. This page is a map for choosing words, not a rulebook; the three candidates (increment, wave, receipt) remain unregistered until the operator registers them.

## Source weighting for naming (operator stance, 2026-10-08)

Three tiers, applied to every row below. (1) Standard lexicons and academic or lab papers ground a concept and supply the term when one exists. (2) A published author or speaker's phrasing is the likeliest to be adopted widely; where tiers 1 and 2 agree, use that phrasing (today: the author of Observability Engineering, 2nd edition, through the book pages, the memory-rules gist and the clarity skill). (3) Cutting-edge practitioners' words (the Corvidae-Coding-Projects repositories; recursion.wtf) are evidence that a practice exists and the proper name of their tool's feature; they are not a source for general names, and their coinages are cited only as theirs (hylomorphism over context windows, devswarm, gauntlet in Thermite's sense, slag, praxis, burn receipt, spec commit). The operator's assessment behind tier 3, marked by the operator as unsupported: these practitioners are often ahead of the papers and the labs, implementing before the labs write about it.

**Re-weighted candidates.** Increment stands: standard agile vocabulary (iterative and incremental development) and the whitepaper's "unit of work"; Thermite's use confirms rather than sources it. Wave drops to a practitioner word: the grounded description is fork-join, a parallel batch forked from one base commit and merged back; three tier-3 repositories say "wave" and no tier-1 or tier-2 source read today does. Receipt yields to attestation, the supply-chain term (SLSA, in-toto) for a signed statement binding an artifact digest to a claim, which is what the two repositories' receipts are and a family the contract already cites; "receipt" stays as the practitioner synonym. Slop gains standing: its source is the published author's corpus-calibrated skill (tier 2), with Palimpsest's "default-model voice" as tier-3 corroboration. Palimpsest's coinages become descriptions of practices named with standard terms where they exist: protected intent as design intent and rationale; canon level as document status in the decision-record vocabulary; reader contract as audience in the technical-communication sense; promise ledger as an obligations register.

## The map

| Said today | Grounded term | Source | Standing in this estate |
|---|---|---|---|
| "the append problem", "accretion", "a labyrinth of appends" | decisions recorded in the event log but not integrated into the projection; unmarked supersession | event sourcing (log versus projection); document control (controlled document, supersession); the write-plans skill ("mark superseded decisions where their history still matters") | the contract says "events are derived at query time" and "the state artifact is a projection"; "anti-accretion" was used in the September compaction |
| "inscrutable rule references, numbers, pages" | unresolvable handles; dangling references | the contract's process-integrity query | registered |
| "the design mess", "stale design", "the ground moved" | drift: specification drift, document drift, dependency drift | software engineering (in the lexicon); Thermite's "doc drift", "freshness", "re-pin"; the register's `toolchain-pin` class | "drift" registered |
| "the mega design", "the constitution" | the umbrella (program document); the thesis | Thermite and Peritus both say "umbrella"; Thermite says "thesis" | our word is "the contract" |
| "unit of work", "bullet" | increment | Thermite ("increment 2a to 2f"); the whitepaper's "unit of work" | candidate for registration |
| a set of units dispatched in parallel from one base | wave | ExoMonad, OpenClaudia, Peritus | candidate for registration; "swarm" is retired |
| "write one, build one", "provisional stage" | receding-horizon planning; least commitment | the planning literature via the write-plans skill; Thermite's "(final)" and "(provisional)" marks | plain description |
| the commit everything forks from | spec commit; scaffold commit | ExoMonad's decision record | walking skeleton is the whole-system form (registered); scaffold is the per-wave form |
| "closure claims without oracles"; "the conformance oracle"; "the oracle re-home" | a closure without its check; the test oracle is the expected result a claim is checked against; the evidence the verifier reads is the synced trace; a review's output is a verdict record (an attestation) | quality engineering: test oracle (Weyuker 1982; Barr and others, "The Oracle Problem in Software Testing", IEEE Transactions on Software Engineering, 2015; in the contract's lexicon list). Observed: Thermite and the property-testing skill use the lexicon sense; the whitepaper, Peritus, OpenClaudia, crosslink and the published author's two documents never use the word; the proof-carrying-cognition paper uses machine learning's sense, the ground-truth selector | operator decision 2026-10-08: "oracle" keeps only the lexicon sense, the expected results the operator authors (the contract heading "The operator authors the oracle" stands); "the synced trace" or "the conformance evidence" for what the verifier reads; "verdict record" or "attestation" for what a review or CI produces; "closure claims without their checks" in new text; tracker titles that use the other senses stay as history |
| "the three evidence classes" | deterministic gates passed; independent review passed; verifier receipt recorded | OpenClaudia's status wording; Peritus's gate-then-verifier order | plain description |
| "higher standard", "ceremony", "process overhead" | high-ceremony versus low-ceremony process; manual versus automated controls | agile (the whitepaper: "high-ceremony by design"); internal-controls audit | the guardrail grades cover the control side |
| "authored but never built", "dormant gates" | authored is not exercised; design versus operating effectiveness; dormant control | the contract's law; internal-controls audit; could-not-check | registered |
| "the review sands off intent", "beige" | normalization pressure; protected intent | Palimpsest ("normalize unusual choices"; "protected intent", "intent tags") | "protected intent" is a candidate |
| late rounds that manufacture findings | a verifier losing soundness under selection pressure; Goodhart | the proof-carrying cognition paper; the domain scorecard's observation | plain description; cite the paper |
| "agent prose", "tics" | register; the tics; comment slop; default-model voice | the contract's register spec; Thermite's tone standard; the clarity skill; Palimpsest | "register" registered; "slop" is used as a named category by two practitioner sources read today (the clarity skill names "comment slop"; Palimpsest names "AI voice accumulation" and "default-model voice"); wider usage not checked |
| "what a phase skill is for" | reader contract; intent tags; status | Palimpsest; decision-record statuses (proposed, accepted, superseded, deprecated) | prefer the decision-record statuses to "canon" |
| "forgetting policy", "garbage collection" | retention policy (records); compaction (context) | records management; the context paper; Claude Code's own word | plain description |
| "surfacing knowledge at the right time" | injection; retrieval; activation | the contract ("Availability is not activation") | registered |
| "no amendments while a design waits", "the regime" | a work-in-progress limit; a cadence rule | lean and agile | avoid "regime" in governed text |
| "sibling churn", "re-entry cost" | dependency churn; resumption cost; a resumable plan | platform engineering; the write-plans skill | plain description |
| "idea cycles" | proposals; change requests | change management (in the lexicon); crucible's "design-proposal issue" | "proposal" is the word; one design document per proposal |
| "depth over breadth", "28 verifiers" | fan-out limit; agent-count ceiling | ExoMonad; the contract's spend-shape bound | registered as the ceiling |
| "the ledger" | the reconciliation; a promise record | quality engineering ("reconciliation"); Palimpsest ("promise ledger") | collides with the retired cost ledger; prefer "reconciliation" |

| "swarm" | swarm (general usage: many agents dispatched together) versus "crosslink swarm" (the command); "wave" is the narrower case of one parallel batch forked from one base | four sources read today use the general sense (the property-testing article, the context paper's "agent-swarm task", ExoMonad's "devswarm", ferrotorch's swarm work breakdown); the contract's reserved-word note for "phase" is the disambiguation precedent | what was retired is narrower than the word: "swarm invocation" as the name of a review round, and the contract's bindings to the crosslink swarm command (2026-10-02). The general sense stays usable with the tool name as the disambiguator; if crosslink extends the command, re-binding the review act to it is a paved-path-map entry under the three-question rule, not a contract change (operator, 2026-10-08) |

## Word choices decided by the operator on 2026-10-08 (in chat; recorded on `vsdd-cli#839`; not registrations)

| Old word | New word | Grounding | Standing |
|---|---|---|---|
| "phase skill" (the per-phase instruction document) | phase skill | "skill" is Claude Code's own word for the artifact; the 2026-10-02 decision on `vsdd-cli#839` already makes them skills; "phase" disambiguates from other skills | used in every page revised 2026-10-08; the contract's and the skills' own uses of "phase skill" stand until the amendment |
| "rules file" (the per-language or per-surface guidance file) | rules file | Claude Code's word for the path-scoped files under its rules folder; the 2026-10-02 delivery decision | used from 2026-10-08; file paths under `supplements/` keep their names until renamed |
| "reviewer role" | reviewer role | crosslink's and Thermite's word for the agent that receives the instruction | used from 2026-10-08 |
| "slice", "Slice N" | a milestone named by its feature ("the composition milestone"); "increment" for the unit of work dispatched as one issue | the tracker's own milestones; agile (increment); the operator's stated dislike of "slice" | used from 2026-10-08 in new text; "slice" stays registered in the contract (`vsdd-cli#821`) until the amendment, and the tracker's milestone titles keep it |
| letter-number ids of this estate's own making (the reconciliation ledger's former row ids) | the row's plain title | the project rules (no coined labels); the clarity skill's unexplained-abbreviation pattern | none in new text; the reconciliation ledger's rows were retitled 2026-10-08. A source record's own labels are cited as that record's, with its location |
| bare "`vsdd-cli#839`", "2a", "the estate" | a handle with its title at first mention or in a key; phases by name at first mention; "this estate" expanded as vsdd-cli with crosslink and mdatron | the clarity skill (unexplained abbreviations); the contract's handle grammar | used from 2026-10-08 |

## Three notes

- The one phrase for which no standard term was found is the append problem itself; event sourcing's log-versus-projection split, which the contract already uses, is its best vocabulary, and "unintegrated decisions" is the plain description.
- Registration candidates after source weighting: increment (standard agile; the whitepaper; Thermite confirms) and attestation (SLSA and in-toto; the repositories' "receipt" is the practitioner synonym). "Wave" is a practitioner word for a fork-join batch and is not proposed.
- "Slop" is used as a named category by two practitioner sources read today (a corpus-calibrated lint named for it; a failure catalogue naming default-model voice). Whether it is used more widely was not checked; cite those two sources, not usage in general.

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#821`: Terminology follow-on (contract-wide): register vertical-slice/slice (grounded, agile SWE, used ... [closed]
- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
