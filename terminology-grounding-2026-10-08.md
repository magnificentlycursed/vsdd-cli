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

Design input for the vocabulary registry's next update; registration is the operator's act and nothing here is registered. Source: the day's chat, read against the contract's reference lexicons (OpenTelemetry, site reliability engineering, FinOps, internal-controls audit, quality engineering, change management, decision records, document control, software engineering), the reference repositories' own words, and the practitioner sources read today. Rule applied: a nickname may live in chat; governed text carries the grounded term or a plain description; no coined compounds.


Operator stance recorded 2026-10-08: term changes are welcome where they make things more understandable, even when the change is annoying; no proliferation of small rules by accident; no coinages. This page is a map for choosing words, not a rulebook; the three candidates (increment, wave, receipt) remain unregistered until the operator registers them.

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
| "closure claims without oracles" | a closure without a receipt; test oracle | quality engineering (oracle, in the lexicon); OpenClaudia and Peritus say "receipt" | "receipt" is a candidate; "oracle" registered |
| "the three evidence classes" | deterministic gates passed; independent review passed; verifier receipt recorded | OpenClaudia's status wording; Peritus's gate-then-verifier order | plain description |
| "higher standard", "ceremony", "process overhead" | high-ceremony versus low-ceremony process; manual versus automated controls | agile (the whitepaper: "high-ceremony by design"); internal-controls audit | the guardrail grades cover the control side |
| "authored but never built", "dormant gates" | authored is not exercised; design versus operating effectiveness; dormant control | the contract's law; internal-controls audit; could-not-check | registered |
| "the review sands off intent", "beige" | normalization pressure; protected intent | Palimpsest ("normalize unusual choices"; "protected intent", "intent tags") | "protected intent" is a candidate |
| late rounds that manufacture findings | a verifier losing soundness under selection pressure; Goodhart | the proof-carrying cognition paper; the domain scorecard's observation | plain description; cite the paper |
| "agent prose", "tics" | register; the tics; comment slop; default-model voice | the contract's register spec; Thermite's tone standard; the clarity skill; Palimpsest | "register" registered; "slop" is now a practitioner term of art |
| "what a primer is for" | reader contract; intent tags; status | Palimpsest; decision-record statuses (proposed, accepted, superseded, deprecated) | prefer the decision-record statuses to "canon" |
| "forgetting policy", "garbage collection" | retention policy (records); compaction (context) | records management; the context paper; Claude Code's own word | plain description |
| "surfacing knowledge at the right time" | injection; retrieval; activation | the contract ("Availability is not activation") | registered |
| "no amendments while a design waits", "the regime" | a work-in-progress limit; a cadence rule | lean and agile | avoid "regime" in governed text |
| "sibling churn", "re-entry cost" | dependency churn; resumption cost; a resumable plan | platform engineering; the write-plans skill | plain description |
| "idea cycles" | proposals; change requests | change management (in the lexicon); crucible's "design-proposal issue" | "proposal" is the word; one design document per proposal |
| "depth over breadth", "28 verifiers" | fan-out limit; agent-count ceiling | ExoMonad; the contract's spend-shape bound | registered as the ceiling |
| "the ledger" | the reconciliation; a promise record | quality engineering ("reconciliation"); Palimpsest ("promise ledger") | collides with the retired cost ledger; prefer "reconciliation" |

| "swarm" | swarm (general usage: many agents dispatched together) versus "crosslink swarm" (the command); "wave" is the narrower case of one parallel batch forked from one base | the general sense is common usage in today's sources (the property-testing article, the context paper's "agent-swarm task", ExoMonad's "devswarm", ferrotorch's swarm work breakdown); the contract's reserved-word note for "phase" is the disambiguation precedent | what was retired is narrower than the word: "swarm invocation" as the name of a review round, and the contract's bindings to the crosslink swarm command (2026-10-02). The general sense stays usable with the tool name as the disambiguator; if crosslink extends the command, re-binding the review act to it is a paved-path-map entry under the three-question rule, not a contract change (operator, 2026-10-08) |

## Three notes

- The one phrase with no standard term is the append problem itself; event sourcing's log-versus-projection split, which the contract already uses, is its best vocabulary, and "unintegrated decisions" is the plain description.
- Three words earn registration because every reference uses them and this estate has none: increment, wave, receipt.
- "Slop" crossed from slang to a term of art in two independent practitioner sources (a corpus-calibrated lint named for it; a failure catalogue naming default-model voice), so it can be cited rather than avoided.
