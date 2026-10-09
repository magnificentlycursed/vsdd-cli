---
title: "Validation of the 2026-10-08 findings against this estate's own repositories (2026-10-09)"
tags: ["design-input", "evidence", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-09
updated: 2026-10-09
---

## Design Specification

### status

Evidence page, design input for the phase-skill rewrite (`vsdd-cli#898`) and the amendment cycle (`vsdd-cli#897`). The findings of 2026-10-08 (the index `vsdd-in-practice-reference-repositories-2026-10-08`) and the agent-generated-tests paper (`agent-generated-tests-study-arxiv-2602-07900-2026-10-09`) are checked against five repositories the operator built, read on 2026-10-09 from their git histories, their crosslink hub databases and their in-repository records. Compliance with the methodology is measured from those records, never taken from the operator's account. Every number below is reproducible from the named source; the scripts are the orchestrating session's and are not part of the estate.

Phase names on this page are the contract's: 1a behavioral specification, 1c the spec review gate, 2a test-suite generation (the red gate), 2b minimal implementation, 3 adversarial refinement, 5 formal hardening. "This estate" means vsdd-cli together with crosslink and mdatron.

### the repositories and their measured compliance

| repository | commits, span | commits with a phase or red-gate marker | records of the process | red gate in the history | first property, fuzz and mutation testing |
|---|---|---|---|---|---|
| issue-tracker-cli (manual methodology, the vsdd-suite era; a subdirectory of the guild-portfolio repository) | 67, 2026-04-27 to 05-25 | 47 of 67 (70 percent) | DESIGN.md 34 KB, DECISIONS.md 30 KB, PROCESS.md 75 KB (a per-layer retrospective with dates, commits, rounds and verdicts), TODO.md, a closure protocol, twelve domain review logs holding 191 reviews and 735 findings | one red-gate commit per layer for seven layers; each named in PROCESS.md with its commit; Layer 7's red gate recorded as known to pass against the pre-implementation base, committed anyway | mutation testing at Layer 4 (2026-05-05); no property or fuzz testing |
| bookmark-cli-manual (manual methodology; the suite's reference example) | 69, 2026-05-20 to 05-25 | 52 of 69 (75 percent) | DESIGN.md 76 KB, PROCESS.md 46 KB, TODO.md 36 KB, manual-tests, the suite's review log | Layer 3: 15 failing tests at 17:52, green at 18:01 | property testing on day one, mutation on day four, fuzzing on day five |
| mdatron (little vsdd guidance) | 426, 2026-06-01 to 10-05 | 38 of 426 (9 percent) | DESIGN.md 40 KB at ratification, 77 KB now, 66 edits riding feature commits; a crosslink hub with 256 issues and 655 typed comments; spec-review rounds 3 to 7; a pre-publish roast; per-feature cold reviews | phase 2a and 2b commit pairs on 2026-06-01 and 06-02 (6 to 73 minutes apart), then a retroactive red gate in July; 24 test-only commits of 226 touching Rust | all three on day one (2026-06-01) |
| vsdd-cli (this repository) | 231, 2026-05-27 to 10-08 | 52 of 231 (22 percent) | the contract (43 KB draft to 255 KB, compacted to 104 KB, now 110 KB; 33 edits, none with code); a hub with 900 issues and 1,691 typed comments; July review rounds with terminal verify rounds | layer red gates in July (red to green in 53 minutes); the live self-governance milestone over three days; the install milestone's static half as one mixed commit; 11 test-only commits of 61 | property and fuzz on day one; mutation 2026-07-30 |
| crosslink, the operator's 79 commits on the fork | 79, 2026-07-20 to 10-09 | upstream's conventions | upstream issues and pull requests; the latest commits (2026-10-08 and 09) are an umbrella document, a scaffold commit, a fix commit and a review-round commit, in that order | 1 test-only commit of 54 | upstream's |

Pull requests, all three GitHub repositories: 184 merged, zero with a GitHub review; median time from opening to merge 0.1 to 0.3 hours; additions per merged pull request median 45 (vsdd-cli), 212 (mdatron), 888 (guild-portfolio, where a pull request is a layer).

### finding by finding

**1. Agent-written tests are probes, not checks (the paper).** Not reproduced in any of the five, nor in crosslink itself. Prints inside test bodies: 0, 0, 8, 5 and 16 across issue-tracker-cli, bookmark-cli-manual, mdatron, vsdd-cli and crosslink. Assertion kinds by the paper's four categories, over every assertion in test bodies:

| repository | assertions | exact | relational | property and predicate | sanity |
|---|---|---|---|---|---|
| issue-tracker-cli | 300 | 40 | 3 | 39 | 18 |
| bookmark-cli-manual | 130 | 58 | 0 | 28 | 15 |
| mdatron | 1,957 | 55 | 7 | 29 | 9 |
| vsdd-cli | 628 | 51 | 4 | 30 | 15 |
| crosslink | 8,616 | 49 | 3 | 32 | 16 |

Percentages. The methodology-compliant repositories do not carry stronger assertions than the lightly guided one or the reference: the strictest profile is mdatron's. Two cautions. The classifier is a rule pass over Rust assertion macros, not the paper's Python one, and one test file per repository was read to confirm the categories land as intended. And the taxonomy misreads one honest form: a test of a validation function whose whole behavior is a boolean or a rejection (`assert!(parse_label("a\tb").is_err())`) is an exact check of that function and lands in sanity or predicate; issue-tracker-cli's suite is dominated by such tests, which is where its 18 percent sanity comes from. What the methodology changed is visible elsewhere: one assertion per test (1.26 and 1.55 per test in the two manual projects against 2.45 to 2.75 in the others) and test names that state the behavior.

**2. No reference runs a red gate for feature increments; here the red gate is a commit-ordering act within one session.** Where the methodology was followed the red-gate commit exists and is named, and it precedes the implementation by minutes: two minutes (issue-tracker-cli Layer 6), nine (bookmark-cli-manual Layer 3), six to seventy-three (mdatron, June), fifty-three (vsdd-cli, July). Across all five, commits that touch tests mostly also touch source: mixed 19, test-only 1 in issue-tracker-cli; 12 and 4 in bookmark-cli-manual; 76 and 24 in mdatron; 32 and 11 in vsdd-cli; 18 and 1 in the operator's crosslink commits. The paper's "tests written alongside the fix" is the dominant commit shape in this estate too; the difference the methodology made is that a failing run was recorded first, in the same hour. Whether that ordering changed outcomes is not measurable from these records.

**3. Design-to-build latency of hours to ten days (the references).** Reproduced, and the converse holds. issue-tracker-cli: specification complete and Layer 1 red gate on the same day (2026-04-27), seven layers in four weeks. bookmark-cli-manual: three layers in six days. mdatron: design ratified 2026-07-19, features from the next day onward. vsdd-cli: the July layers same-day; the live self-governance milestone one day from its ratification (2026-07-28) to its first build commit (07-29) and three days to completion. The two designs with long latency are the two that never built: the composition milestone (ratified 2026-08-01, 70 days) and the gate-execution milestone (2026-07-30).

**4. Design documents accumulate when amended apart from code.** Reproduced in both directions. mdatron's DESIGN.md went from 40 to 77 KB over ten weeks in 66 edits, nearly all riding feature commits. issue-tracker-cli's went from 20 to 34 KB in two weeks in 11 edits, each riding a layer's review round. The contract went from 43 KB to 255 KB in fourteen days of July and August 2026 in amendment cycles that carried no code, and needed a compaction to 104 KB in September.

**5. The first round finds; later rounds verify; breadth manufactures findings.** Reproduced three times over. mdatron's spec-review rounds 3 to 7 (2026-07-19) carried 16, 13, 5, 1 and 0 child findings. vsdd-cli's July rounds: round 1 with 24, 9 and 9 children; round 2 with 8 and 3; round 3 with 1; terminal verify rounds with 3 and 1. issue-tracker-cli's PROCESS.md: Layer 4 round 1 with 23 open findings across nine domains, round 2 verification; Layer 7 round 1 with 24 substantive findings and one critical, round 2 closure. The breadth signal: in June 2026 vsdd-cli filed 192 finding-shaped issues in one month, every one now closed, 104 of them (54 percent) with dismissed, hallucinated or consolidated in the closing comment. Dispositions in issue-tracker-cli's logs are narrative sentences, not fields, and were not counted. Weight (operator, 2026-10-09): low. Finding counts vary with the prompts, the reviewer set and the method, all of which were changing quickly across these months, and fix-then-verify produces a decay by construction; the decay therefore does not show that one reviewer would have found what the first round found. The dismissed share of the June filings is the firmer number, and it too comes from an earlier process.

**6. Zero GitHub reviews, review elsewhere.** Reproduced: 184 merged pull requests, zero reviews, median minutes to merge. The reviews live in the hubs' typed comments and the suite's review logs.

**7. Formal hardening early, not fifth.** Reproduced in the operator's own practice before the references were read: property, fuzz and mutation testing on day one in mdatron (2026-06-01), property and fuzz on day one in vsdd-cli (05-27), property on day one in bookmark-cli-manual; only issue-tracker-cli waited until its fourth layer, and it ran no property or fuzz testing.

**8. Three evidence classes kept distinct; closure claims cite evidence.** Partly. Closure comments citing a commit hash: 44 percent in vsdd-cli (491 of 1,123) and 63 percent in mdatron (211 of 333); citing a test path, 17 and 40 percent. The manual projects close by GO and NO-GO verdicts in PROCESS.md naming commits. The remainder is prose.

**9. Prose control is mechanized here and nowhere else in the set.** vsdd-cli arms 14 register anti-patterns and 12 deprecated aliases; mdatron arms none in its own configuration; the suite era shipped a letter-cluster check among its 13 hooks. Whether the prose got better is not measurable from these records.

**10. Cost is not yet knowable from records.** The hubs hold 5 token-usage rows (vsdd-cli) and 0 (mdatron); the two spend escapes the contract cites are recorded in prose only. The paper's cost finding cannot be checked here.

**11. Termination.** Mechanical where the terminal verify round ran (vsdd-cli July; mdatron's round 7 with zero children as the stop); by director decision in the manual projects, with the deviations written down (Layer 2 and 3 closed without the cold-session second pass; Layer 6's manual checklist deferred to round 3).

**12. Increment size.** The shipped units: a layer in about four days (issue-tracker-cli), two days (bookmark-cli-manual), a feature in a day (mdatron), a layer in two to three days and one slice in four (vsdd-cli). The unit that did not ship was a seven-slice design amended for ten weeks.

### what this changes

Three findings from 2026-10-08 are confirmed on this estate's own evidence rather than only on the references': latency and append accumulation (3 and 4) and early hardening (7); the round decay (5) is observed but confounded and carries little weight. Two are refined: the red gate as practised here is a within-session ordering with the failing run recorded, not a separate authoring phase, and its effect on outcomes is unmeasured (2); assertion strength is not where the methodology shows, test granularity is (1). One is a gap in the records themselves (10). The operator's most recent work on the crosslink fork follows the umbrella, scaffold, increment and review-round form the decisions of 2026-10-08 adopt; the agent that did that work had read this estate's knowledge pages (operator, 2026-10-09), so this is evidence that the pages transfer the practice, not independent evidence for it.

### handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-09.

- `vsdd-cli#897`: Amend the contract in one owned cycle, the last before the build window: live milestone documents, decisions in the document, sibling bindings moved to data, the hook passages, the conformance member's build scope, the workspace sentence [open]
- `vsdd-cli#898`: Phase-skill rewrite (design-first): the ten phase skills, the prose reviewer roles and the register standard, rewritten from the 2026-10-08 reference practice [open]

